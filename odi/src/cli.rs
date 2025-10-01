//! Command-line interface definition

use clap::{Parser, Subcommand};

use crate::commands::*;
use crate::{Result, AppContext};

/// ODI - Distributed Issue Tracking
/// 
/// A Git-like distributed issue tracking system for offline-first project management.
/// 
/// EXAMPLES:
///     odi init                    # Initialize workspace in current directory
///     odi issue create "Bug fix"  # Create a new issue
///     odi remote add origin ssh://git@server/repo.odi
///     odi push origin             # Push issues to remote (defaults to origin)
///     odi pull origin             # Pull issues from remote (defaults to origin)
#[derive(Parser)]
#[command(name = "odi")]
#[command(about = "A Git-like distributed issue tracking system")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(long_about = "ODI provides distributed, offline-first issue tracking similar to Git.\nUse 'odi <command> --help' for detailed command information.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available commands
#[derive(Subcommand)]
pub enum Commands {
    /// Initialize ODI workspace in current directory
    #[command(about = "Initialize ODI workspace\n\nCreates .odi directory with configuration and storage.\nDetects Git repository and associates issues with it.")]
    Init(InitArgs),
    
    /// Project management commands
    #[command(about = "Project management commands\n\nManage projects that group related issues together.")]
    Project(ProjectArgs),
    
    /// Issue management commands  
    #[command(about = "Issue management commands\n\nCreate, list, and manage issues in your workspace.")]
    Issue(IssueArgs),
    
    /// Push local changes to remote repository
    #[command(about = "Push local issues to remote repository\n\nUpload local issues to the specified remote (defaults to 'origin').")]
    Push(PushArgs),
    
    /// Pull changes from remote repository
    #[command(about = "Pull remote issues to local repository\n\nDownload issues from the specified remote (defaults to 'origin').")]
    Pull(PullArgs),
    
    /// Remote repository commands
    #[command(about = "Remote repository management\n\nAdd, list, and manage remote repositories for synchronization.")]
    Remote(RemoteArgs),
    
    /// Team management commands
    #[command(about = "Team management commands\n\nManage users and teams for collaborative workflows.")]
    Team(TeamArgs),
    
    /// Configuration commands
    #[command(about = "Configuration management\n\nView and modify ODI settings in ~/.odiconfig and .odi/config")]
    Config(ConfigArgs),
    
    /// Label management commands
    #[command(about = "Label management commands\n\nCreate and manage labels to categorize issues.")]
    Label(LabelArgs),
    
    /// Filesystem check and repair
    #[command(about = "Check and repair ODI data integrity\n\nValidate object store and fix corruption issues.")]
    Fsck(FsckArgs),
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
            Commands::Push(args) => {
                // Require workspace for push commands
                AppContext::require_workspace(None)?;
                let ctx = AppContext::new(None).await?;
                args.execute(&ctx).await
            },
            Commands::Pull(args) => {
                // Require workspace for pull commands
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
            Commands::Fsck(args) => {
                // Require workspace for fsck
                AppContext::require_workspace(None)?;
                let ctx = AppContext::new(None).await?;
                args.execute(&ctx).await
            },
        }
    }
}