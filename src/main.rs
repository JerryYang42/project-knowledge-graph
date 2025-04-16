mod cli;
mod db;
mod models;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use dotenv::dotenv;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file if present
    dotenv().ok();

    // Initialize logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    info!("Starting Project Knowledge Graph CLI");

    // Parse command line arguments
    let cli = Cli::parse();

    // Run the command
    cli.run().await?;

    Ok(())
}