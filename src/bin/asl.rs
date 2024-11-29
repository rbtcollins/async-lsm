use clap::{Parser, Subcommand};
use color_eyre::eyre::{self, Result};

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

fn main() -> Result<(), color_eyre::eyre::Report> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let db_url = cli.db_url.unwrap_or_else(|| ".".to_string());

    eprintln!("Database location: {}", db_url);

    match &cli.command {
        Some(Commands::Info) => {}
        None => {
            eyre::bail!("No command specified");
        }
    }
    Ok(())
}
