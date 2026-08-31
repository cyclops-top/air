use clap::Parser;
use local_ip_address::local_ip;
use std::path::PathBuf;
use std::sync::Arc;
use tokio_stream::StreamExt;

mod domain;
mod application;
mod infrastructure;

mod cert;
mod dashboard;
mod fs_utils;
mod view;

use crate::application::file_service::FileService;
use crate::application::discovery_manager::DiscoveryManager;
use crate::infrastructure::discovery::mdns::MdnsDiscoveryProvider;
use crate::infrastructure::filesystem::local::{LocalFileRepository, MmapCache};
use crate::infrastructure::ui::html_renderer::HtmlRenderer;
use crate::infrastructure::server::start_server;
use crate::domain::models::{AppState, Stats};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(default_value = ".")]
    path: PathBuf,
    #[arg(short, long)]
    port: Option<u16>,
    #[arg(long, default_value_t = false)]
    https: bool,
}

#[derive(clap::Subcommand)]
enum Commands {
    Discover {
        #[arg(short, long, default_value_t = 3)]
        duration: u64,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // 1. Initialize Infrastructure
    let _mmap_cache = Arc::new(MmapCache::new()); // Keep for compatibility if needed, but repo doesn't use it
    let file_repo = Arc::new(LocalFileRepository::new());
    let discovery_provider = Arc::new(MdnsDiscoveryProvider::new()?);
    let ui_renderer = Arc::new(HtmlRenderer::new());

    // 2. 初始化应用层服务
    let discovery_manager = Arc::new(DiscoveryManager::new(discovery_provider));

    // --- Discover 模式 ---
    if let Some(Commands::Discover { .. }) = cli.command {
        crossterm::terminal::enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
        let backend = ratatui::backend::CrosstermBackend::new(stdout);
        let mut terminal = ratatui::Terminal::new(backend)?;

        let mut ui = view::DiscoverUI::new();
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);
        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
        
        let dm_clone = discovery_manager.clone();
        tokio::spawn(async move { let _ = dm_clone.start_discovery(tx, shutdown_rx).await; });

        let mut event_reader = crossterm::event::EventStream::new();
        loop {
            terminal.draw(|f| view::render_discover(f, &mut ui))?;
            tokio::select! {
                Some(msg) = rx.recv() => { ui.update_nodes(msg); }
                event = event_reader.next() => {
                    if let Some(Ok(crossterm::event::Event::Key(key))) = event {
                        if key.kind == crossterm::event::KeyEventKind::Press {
                            match key.code {
                                crossterm::event::KeyCode::Char('q') | crossterm::event::KeyCode::Char('Q') => break,
                                crossterm::event::KeyCode::Up => ui.previous(),
                                crossterm::event::KeyCode::Down => ui.next(),
                                crossterm::event::KeyCode::Enter => {
                                    if let Some(node) = ui.selected_node() {
                                        let url = format!("{}://{}:{}", node.scheme, node.ip, node.port);
                                        #[cfg(target_os = "macos")] let _ = std::process::Command::new("open").arg(&url).spawn();
                                        #[cfg(target_os = "linux")] let _ = std::process::Command::new("xdg-open").arg(&url).spawn();
                                        #[cfg(target_os = "windows")] let _ = std::process::Command::new("cmd").args(["/C", "start", &url]).spawn();
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {}
            }
        }
        let _ = shutdown_tx.send(());
        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        std::process::exit(0);
    }

    // --- Share 模式 ---
    let root_path = std::fs::canonicalize(&cli.path).unwrap_or_else(|e| {
        eprintln!("Error: Cannot access path: {}", e);
        std::process::exit(1);
    });

    let lan_ip = local_ip().unwrap_or_else(|_| "127.0.0.1".parse().unwrap());
    let host_name = hostname::get().ok().and_then(|h| h.into_string().ok());

    let app_state = Arc::new(AppState {
        root_path: root_path.clone(),
        stats: Arc::new(Stats::default()),
        enable_https: cli.https,
        lan_ip: lan_ip.to_string(),
        port: 0, // Placeholder
    });

    let file_service = Arc::new(FileService::new(file_repo, app_state.clone()));

    // 启动服务器
    let used_port = start_server(
        cli.port,
        root_path,
        cli.https,
        lan_ip,
        file_service,
        ui_renderer,
        app_state.clone()
    ).await?;

    // 注册 mDNS
    let discovery_msg = domain::models::DiscoveryMsg {
        id: rand::random::<u32>().to_string(),
        name: host_name.clone().unwrap_or_else(|| "Unknown".to_string()),
        ip: lan_ip,
        port: used_port,
        scheme: if cli.https { "https".to_string() } else { "http".to_string() },
        is_online: true,
    };
    let fullname = discovery_manager.register_service(&discovery_msg)?;

    if crossterm::tty::IsTty::is_tty(&std::io::stdout()) {
        crossterm::terminal::enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
        let backend = ratatui::backend::CrosstermBackend::new(stdout);
        let mut terminal = ratatui::Terminal::new(backend)?;

        // 使用标准探测方式，10.x 版本通常能自动处理 iTerm2
        let picker = if std::env::var("TERM").map(|t| t == "dumb").unwrap_or(false) {
            None
        } else {
            ratatui_image::picker::Picker::from_query_stdio().ok()
        };

        let mut ui_state = dashboard::DashboardState {
            scroll_offset: 0,
            lan_ip: lan_ip.to_string(),
            port: used_port,
            picker,
            image_state: None,
        };

        let mut event_reader = crossterm::event::EventStream::new();
        loop {
            terminal.draw(|f| dashboard::render(f, &app_state.stats, &mut ui_state))?;
            tokio::select! {
                event = event_reader.next() => {
                    if let Some(Ok(crossterm::event::Event::Key(key))) = event {
                        if key.kind == crossterm::event::KeyEventKind::Press {
                            match key.code {
                                crossterm::event::KeyCode::Char('q') | crossterm::event::KeyCode::Char('Q') => break,
                                crossterm::event::KeyCode::Up => { ui_state.scroll_offset += 1; }
                                crossterm::event::KeyCode::Down => { if ui_state.scroll_offset > 0 { ui_state.scroll_offset -= 1; } }
                                _ => {}
                            }
                        }
                    }
                }
                _ = tokio::time::sleep(std::time::Duration::from_millis(200)) => {}
            }
        }
        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
        terminal.show_cursor()?;
    } else {
        tokio::signal::ctrl_c().await?;
    }

    let _ = discovery_manager.unregister_service(&fullname);
    println!("\nSummary of this session:");
    println!("  ➜  Files downloaded: {}", app_state.stats.total_files.load(std::sync::atomic::Ordering::Relaxed));
    println!("  ➜  Total volume:    {}", view::format_size(app_state.stats.total_bytes.load(std::sync::atomic::Ordering::Relaxed)));
    println!("  ➜  Total uptime:    {}", view::format_duration(app_state.stats.start_time.elapsed()));

    Ok(())
}