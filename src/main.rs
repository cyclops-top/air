use clap::Parser;
use local_ip_address::local_ip;
use std::path::PathBuf;

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

    /// Port to listen on
    #[arg(short, long, default_value_t = 8000)]
    port: u16,
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

    // 2. Get LAN IP
    let lan_ip = local_ip().unwrap_or_else(|_| "127.0.0.1".parse().unwrap());

    // 3. Start server
    let app_state = server::start(cli.port, root_path).await?;

    // 4. Setup TUI
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    let mut ui_state = dashboard::DashboardState {
        scroll_offset: 0,
        lan_ip: lan_ip.to_string(),
        port: cli.port,
    };

    use tokio_stream::StreamExt;
    let mut event_reader = crossterm::event::EventStream::new();

    loop {
        terminal.draw(|f| dashboard::render(f, &app_state, &ui_state))?;

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

    Ok(())
}