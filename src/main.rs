use clap::Parser;
use local_ip_address::local_ip;
use std::path::PathBuf;

mod cert;
mod dashboard;
mod fs_utils;
mod handlers;
mod logger;
mod server;
mod view;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the directory to share
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Port to listen on. If not provided, a random port between 10000-65535 will be used.
    #[arg(short, long)]
    port: Option<u16>,

    /// Enable HTTPS with a self-signed certificate
    #[arg(long, default_value_t = false)]
    https: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // 1. Resolve absolute path
    let root_path = match std::fs::canonicalize(&cli.path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: Cannot access path '{}': {}", cli.path.display(), e);
            std::process::exit(1);
        }
    };

    // 2. Get LAN IP and Hostname
    let lan_ip = local_ip().unwrap_or_else(|_| "127.0.0.1".parse().unwrap());
    let host_name = hostname::get().ok().and_then(|h| h.into_string().ok());

    // 3. Start server
    let (app_state, used_port) = server::start(cli.port, root_path, cli.https, lan_ip).await?;

    // 4. Setup TUI (only if TTY)
    let is_tty = crossterm::tty::IsTty::is_tty(&std::io::stdout()) && 
                 crossterm::tty::IsTty::is_tty(&std::io::stdin());
    
    if is_tty {
        crossterm::terminal::enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
        let backend = ratatui::backend::CrosstermBackend::new(stdout);
        let mut terminal = ratatui::Terminal::new(backend)?;

        let picker = ratatui_image::picker::Picker::from_query_stdio().ok();

        let mut ui_state = dashboard::DashboardState {
            scroll_offset: 0,
            lan_ip: lan_ip.to_string(),
            port: used_port,
            hostname: host_name,
            picker,
            image_state: None,
        };

        use tokio_stream::StreamExt;
        let mut event_reader = crossterm::event::EventStream::new();

        loop {
            terminal.draw(|f| dashboard::render(f, &app_state, &mut ui_state))?;

            tokio::select! {
                event = event_reader.next() => {
                    if let Some(Ok(crossterm::event::Event::Key(key))) = event {
                        if key.kind == crossterm::event::KeyEventKind::Press {
                            match key.code {
                                crossterm::event::KeyCode::Char('q') | crossterm::event::KeyCode::Char('Q') => break,
                                crossterm::event::KeyCode::Up => {
                                    ui_state.scroll_offset += 1;
                                }
                                crossterm::event::KeyCode::Down => {
                                    if ui_state.scroll_offset > 0 {
                                        ui_state.scroll_offset -= 1;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ = tokio::time::sleep(std::time::Duration::from_millis(200)) => {
                    // Periodic refresh
                }
            }
        }

        // 5. Cleanup TUI
        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(
            terminal.backend_mut(),
            crossterm::terminal::LeaveAlternateScreen
        )?;
        terminal.show_cursor()?;
    } else {
        println!("User defined path: {}", app_state.root_path.display());
        println!("Security Check: SANDBOX ENABLED 🔒");
        println!();
        let protocol = if cli.https { "https" } else { "http" };
        println!("Air is serving at:");
        println!("  ➜  Network: {}://{}:{}", protocol, lan_ip, used_port);
        if let Some(ref h) = host_name {
            println!("  ➜  Host:    {}://{}:{}", protocol, h, used_port);
        }
        println!();
        println!("Non-interactive mode: Waiting for signal (Ctrl-C) to stop...");
        
        tokio::signal::ctrl_c().await?;
    }

    // 6. Print Summary
    println!();
    println!("Summary of this session:");
    println!(
        "  ➜  Files downloaded: {}",
        app_state.stats.total_files.load(std::sync::atomic::Ordering::Relaxed)
    );
    println!(
        "  ➜  Total volume:    {}",
        view::format_size(app_state.stats.total_bytes.load(std::sync::atomic::Ordering::Relaxed))
    );
    println!(
        "  ➜  Total uptime:    {}",
        view::format_duration(app_state.stats.start_time.elapsed())
    );

    Ok(())
}