//! Issue command implementation

use clap::{Args, Subcommand, ValueEnum};
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
        /// Project to create issue in
        #[arg(long, short)]
        project: Option<String>,
        /// Priority level
        #[arg(long, value_enum)]
        priority: Option<Priority>,
    },
    /// List all issues
    List {
        /// Filter by status
        #[arg(long, value_enum)]
        status: Option<IssueStatus>,
        /// Filter by project
        #[arg(long, short)]
        project: Option<String>,
    },
    /// Show issue details
    Show {
        /// Issue ID
        id: String,
        /// Project name (if needed for disambiguation)
        #[arg(long, short)]
        project: Option<String>,
    },
    /// Assign issue to user
    Assign {
        /// Issue ID
        id: String,
        /// User to assign to
        user: String,
        /// Project name (if needed for disambiguation)
        #[arg(long, short)]
        project: Option<String>,
    },
    /// Update issue status
    Status {
        /// Issue ID
        id: String,
        /// New status
        status: IssueStatus,
        /// Project name (if needed for disambiguation)
        #[arg(long, short)]
        project: Option<String>,
    },
    /// Add label to issue
    Label {
        /// Issue ID
        id: String,
        /// Label to add
        label: String,
        /// Project name (if needed for disambiguation)
        #[arg(long, short)]
        project: Option<String>,
    },
}

impl IssueArgs {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.command {
            IssueSubcommand::Create { title, description, project, priority } => {
                let mut issue = Issue::new(
                    title.clone(),
                    std::env::var("USER").unwrap_or_else(|_| "unknown".to_string()), // Get from environment
                );
                
                // Set description if provided
                if let Some(desc) = description {
                    issue.description = Some(desc.clone());
                }
                
                // Set priority if provided
                if let Some(p) = priority {
                    issue.priority = p.clone();
                }
                
                // TODO: Handle project assignment when project management is implemented
                
                let created_issue = ctx.issue_repository().create(issue).await
                    .map_err(crate::OdiError::Core)?;
                
                println!("Created issue: {} ({})", created_issue.title, created_issue.id);
                Ok(())
            },
            IssueSubcommand::List { status, project } => {
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
            IssueSubcommand::Show { id, project: _ } => {
                // TODO: Implement issue show functionality
                println!("Showing issue: {}", id);
                println!("Note: Issue show functionality not yet implemented");
                Ok(())
            },
            IssueSubcommand::Assign { id, user, project: _ } => {
                // TODO: Implement issue assignment functionality
                println!("Assigning issue {} to user {}", id, user);
                println!("Note: Issue assignment functionality not yet implemented");
                Ok(())
            },
            IssueSubcommand::Status { id, status, project: _ } => {
                // TODO: Implement issue status update functionality
                println!("Setting issue {} status to {:?}", id, status);
                println!("Note: Issue status update functionality not yet implemented");
                Ok(())
            },
            IssueSubcommand::Label { id, label, project: _ } => {
                // TODO: Implement issue labeling functionality
                println!("Adding label '{}' to issue {}", label, id);
                println!("Note: Issue labeling functionality not yet implemented");
                Ok(())
            },
        }
    }
}
