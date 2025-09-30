//! Command-line interface definition

use clap::{Parser, Subcommand};

use crate::commands::*;
use crate::{Result, AppContext};

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
    /// Label management commands
    Label(LabelArgs),
}

impl Cli {
    /// Execute the CLI command
    pub async fn execute(&self) -> Result<()> {
        match &self.command {
            Commands::Init(args) => {
                // Init doesn't need context as it creates the workspace
                args.execute().await
            },
            Commands::Project(args) => {
                // Require workspace for project commands
                AppContext::require_workspace(None)?;
                let ctx = AppContext::new(None).await?;
                args.execute(&ctx).await
            },
            Commands::Issue(args) => {
                // Require workspace for issue commands
                AppContext::require_workspace(None)?;
                let ctx = AppContext::new(None).await?;
                args.execute(&ctx).await
            },
            Commands::Remote(args) => {
                // Require workspace for remote commands
                AppContext::require_workspace(None)?;
                let ctx = AppContext::new(None).await?;
                args.execute(&ctx).await
            },
            Commands::Team(args) => {
                // Require workspace for team commands
                AppContext::require_workspace(None)?;
                let ctx = AppContext::new(None).await?;
                args.execute(&ctx).await
            },
            Commands::Config(args) => {
                // Config might work without workspace, but let's require it for now
                AppContext::require_workspace(None)?;
                let ctx = AppContext::new(None).await?;
                args.execute(&ctx).await
            },
            Commands::Label(args) => {
                // Require workspace for label commands
                AppContext::require_workspace(None)?;
                let ctx = AppContext::new(None).await?;
                args.execute(&ctx).await
            },
        }
    }
}