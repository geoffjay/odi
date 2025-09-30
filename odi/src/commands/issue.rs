//! Issue command implementation

use clap::{Args, Subcommand};
use crate::{Result, AppContext};
use odi_core::{Issue, IssueId, IssueStatus, Priority, UserId, IssueRepository};

#[derive(Args)]
pub struct IssueArgs {
    #[command(subcommand)]
    pub command: IssueSubcommand,
}

#[derive(Subcommand)]  
pub enum IssueSubcommand {
    /// Create a new issue
    Create { 
        /// Issue title
        title: String,
        /// Issue description
        #[arg(long, short)]
        description: Option<String>,
    },
    /// List all issues
    List,
}

impl IssueArgs {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.command {
            IssueSubcommand::Create { title, description } => {
                let mut issue = Issue::new(
                    title.clone(),
                    std::env::var("USER").unwrap_or_else(|_| "unknown".to_string()), // Get from environment
                );
                
                // Set description if provided
                if let Some(desc) = description {
                    issue.description = Some(desc.clone());
                }
                
                let created_issue = ctx.issue_repository().create(issue).await
                    .map_err(crate::OdiError::Core)?;
                
                println!("Created issue: {} ({})", created_issue.title, created_issue.id);
                Ok(())
            },
            IssueSubcommand::List => {
                let issues = ctx.issue_repository().list(odi_core::IssueQuery::default()).await
                    .map_err(crate::OdiError::Core)?;
                
                if issues.is_empty() {
                    println!("No issues found.");
                } else {
                    println!("Issues:");
                    for issue in issues {
                        println!("  {} - {} [{:?}]", issue.title, issue.description.as_deref().unwrap_or("No description"), issue.status);
                    }
                }
                Ok(())
            },
        }
    }
}
