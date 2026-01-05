use clap::Parser;
use local_ip_address::local_ip;
use std::path::PathBuf;

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
    let root_path = std::fs::canonicalize(&cli.path)?;

    // 2. Get LAN IP
    let lan_ip = local_ip().unwrap_or_else(|_| "127.0.0.1".parse().unwrap());

    // 3. Print Banner
    println!("User defined path: {}", root_path.display());
    println!("Security Check: SANDBOX ENABLED 🔒");
    println!();
    println!("Air is serving at:");
    println!("  ➜  Local:   http://localhost:{}", cli.port);
    println!("  ➜  Network: http://{}:{}", lan_ip, cli.port);
    println!();
    println!("Hit CTRL-C to stop the server");

        // Start server

        server::start(cli.port, root_path).await?;

        

        Ok(())

    }

    