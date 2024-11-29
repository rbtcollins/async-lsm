use clap::{Parser, Subcommand, ValueEnum};
use color_eyre::eyre::{self, Result};
use tracing_forest::ForestLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Location of database. Defaults to current directory.
    db_url: Option<String>,

    /// What storage backend to use
    #[clap(short = 's', long = "storage", default_value = "file")]
    storage: Storage,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// dump metadata about the database
    Info,
}

/// What storage backend to use
#[derive(Clone, Debug, ValueEnum)]
enum Storage {
    /// Use the file storage backend
    File,
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

    let storage = match &cli.storage {
        Storage::File => async_lsm::features::filestorage::FileStorage::default(),
    };

    match &cli.command {
        Some(Commands::Info) => {
            let _db = async_lsm::OpenOptions::new(storage).open(&db_url).await?;
        }
        None => {
            eyre::bail!("No command specified");
        }
    }
    Ok(())
}
