//! Command-line interface definition

use clap::{Parser, Subcommand};

use crate::commands::*;
use crate::Result;

/// ODI - Distributed Issue Tracking
#[derive(Parser)]
#[command(name = "odi")]
#[command(about = "A Git-like distributed issue tracking system")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available commands
#[derive(Subcommand)]
pub enum Commands {
    /// Initialize ODI workspace
    Init(InitArgs),
    /// Project management commands
    Project(ProjectArgs),
    /// Issue management commands  
    Issue(IssueArgs),
    /// Remote repository commands
    Remote(RemoteArgs),
    /// Team management commands
    Team(TeamArgs),
    /// Configuration commands
    Config(ConfigArgs),
}

impl Cli {
    /// Execute the CLI command
    pub async fn execute(&self) -> Result<()> {
        match &self.command {
            Commands::Init(args) => args.execute().await,
            Commands::Project(args) => args.execute().await,
            Commands::Issue(args) => args.execute().await,
            Commands::Remote(args) => args.execute().await,
            Commands::Team(args) => args.execute().await,
            Commands::Config(args) => args.execute().await,
        }
    }
}