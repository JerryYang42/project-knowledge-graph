use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::cli::{
    branch::BranchCommand,
    dashboard::DashboardCommand,
    database::DatabaseCommand, 
    project::ProjectCommand,
    repository::RepositoryCommand,
    ticket::TicketCommand,
};

pub mod branch;
pub mod dashboard;
pub mod database;
pub mod project;
pub mod repository;
pub mod ticket;

/// Project Knowledge Graph - A CLI tool for managing project information in a graph database
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize and manage the database
    #[clap(name = "db")]
    Database(DatabaseCommand),
    
    /// Manage projects
    #[clap(name = "project")]
    Project(ProjectCommand),
    
    /// Manage repositories
    #[clap(name = "repo")]
    Repository(RepositoryCommand),
    
    /// Manage branches
    #[clap(name = "branch")]
    Branch(BranchCommand),
    
    /// Manage tickets
    #[clap(name = "ticket")]
    Ticket(TicketCommand),
    
    /// Manage dashboards
    #[clap(name = "dashboard")]
    Dashboard(DashboardCommand),
    
    /// Initialize the database with required constraints and indexes
    #[clap(name = "init")]
    Init,
}

impl Cli {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            Commands::Database(cmd) => cmd.run().await,
            Commands::Project(cmd) => cmd.run().await,
            Commands::Repository(cmd) => cmd.run().await,
            Commands::Branch(cmd) => cmd.run().await,
            Commands::Ticket(cmd) => cmd.run().await,
            Commands::Dashboard(cmd) => cmd.run().await,
            Commands::Init => {
                let db_conn = crate::db::connection::get_connection().await?;
                crate::db::schema::initialize_database(&db_conn).await?;
                println!("Database initialized successfully");
                Ok(())
            }
        }
    }
}
