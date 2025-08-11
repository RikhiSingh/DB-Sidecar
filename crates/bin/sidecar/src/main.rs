use clap::{Parser, Subcommand};
use feytra_api::build_router;
use feytra_core::{jobs, Config};
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "sidecar")] 
#[command(about = "Feytra schema intelligence sidecar", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Serve,
    Bootstrap,
    Refresh,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();
    let cfg = Config::from_env();
    match Cli::parse().command {
        Command::Serve => {
            let app = build_router(cfg.clone());
            let addr = std::net::SocketAddr::from(([0,0,0,0], cfg.port));
            tracing::info!(%addr, "serving API");
            axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
        }
        Command::Bootstrap => {
            let v = jobs::execute_bootstrap(&cfg).await?;
            println!("version={}", v);
        }
        Command::Refresh => {
            let v = jobs::execute_refresh(&cfg).await?;
            println!("version={}", v);
        }
    }
    Ok(())
}


