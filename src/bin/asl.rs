use clap::{Parser, Subcommand};
use color_eyre::eyre::{self, Result};
use tracing_forest::ForestLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Location of database. Defaults to current directory.
    db_url: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// dump metadata about the database
    Info,
}

#[tokio::main]
async fn main() -> Result<(), color_eyre::eyre::Report> {
    color_eyre::install()?;
    Registry::default()
        .with(ForestLayer::default())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let cli = Cli::parse();

    let db_url = cli.db_url.unwrap_or_else(|| ".".to_string());

    eprintln!("Database location: {}", db_url);

    match &cli.command {
        Some(Commands::Info) => {
            let _db = async_lsm::OpenOptions::new().open(&db_url).await?;
        }
        None => {
            eyre::bail!("No command specified");
        }
    }
    Ok(())
}
