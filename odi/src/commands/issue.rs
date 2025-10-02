//! Issue command implementation

use clap::{Args, Subcommand};
use crate::{Result, AppContext};
use odi_core::{Issue, IssueStatus, Priority, IssueRepository};

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
        /// Issue ID (will be generated if not provided)
        #[arg(long)]
        id: Option<String>,
    },
    /// List all issues
    List {
        /// Filter by status
        #[arg(long, value_enum)]
        status: Option<IssueStatus>,
        /// Filter by project
        #[arg(long, short)]
        project: Option<String>,
        /// Filter by issue ID
        #[arg(long)]
        id: Option<String>,
        /// Filter by issue description
        #[arg(long)]
        description: Option<String>,
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
            IssueSubcommand::Create { title, description, project, priority, id } => {
                let mut issue = Issue::new(
                    title.clone(),
                    std::env::var("USER").unwrap_or_else(|_| "unknown".to_string()), // Get from environment
                );
                
                // Set custom ID if provided
                if let Some(custom_id) = id {
                    let issue_id = match uuid::Uuid::parse_str(custom_id) {
                        Ok(uuid) => uuid,
                        Err(_) => {
                            eprintln!("❌ Invalid Issue ID");
                            eprintln!("Issue ID must be a valid UUID");
                            eprintln!("💡 Tip: Omit --id to auto-generate a UUID");
                            return Err(crate::OdiError::Core(odi_core::CoreError::ValidationError {
                                field: "issue_id".to_string(),
                                message: format!("Invalid UUID format: {}", custom_id)
                            }));
                        }
                    };
                    issue.id = issue_id;
                }
                
                // Set description if provided
                if let Some(desc) = description {
                    issue.description = Some(desc.clone());
                }
                
                // Set priority if provided
                if let Some(p) = priority {
                    issue.priority = p.clone();
                }
                
                // Set project if provided
                if let Some(project_id) = project {
                    issue.project_id = Some(project_id.clone());
                }
                
                let created_issue = ctx.issue_repository().create(issue).await
                    .map_err(crate::OdiError::Core)?;
                
                println!("Created issue: {} ({})", created_issue.title, created_issue.id);
                Ok(())
            },
            IssueSubcommand::List { status, project, id, description } => {
                // Build query with filters
                let mut query = odi_core::IssueQuery::default();
                
                if let Some(s) = status {
                    query.status = Some(s.clone());
                }
                
                if let Some(p) = project {
                    query.project_id = Some(p.clone());
                }
                
                let issues = ctx.issue_repository().list(query).await
                    .map_err(crate::OdiError::Core)?;
                
                // Apply additional filtering that's not in the repository query
                let filtered_issues: Vec<_> = issues.into_iter().filter(|issue| {
                    // Filter by ID if specified
                    if let Some(filter_id) = id {
                        if !issue.id.to_string().contains(filter_id) {
                            return false;
                        }
                    }
                    
                    // Filter by description if specified
                    if let Some(filter_desc) = description {
                        match &issue.description {
                            Some(desc) => {
                                if !desc.to_lowercase().contains(&filter_desc.to_lowercase()) {
                                    return false;
                                }
                            },
                            None => return false, // No description, but filter requires one
                        }
                    }
                    
                    true
                }).collect();
                
                if filtered_issues.is_empty() {
                    println!("No issues found.");
                } else {
                    println!("Issues:");
                    for issue in filtered_issues {
                        let assignee_str = if issue.assignees.is_empty() {
                            "Unassigned".to_string()
                        } else {
                            format!("Assigned to: {}", issue.assignees.join(", "))
                        };
                        
                        println!("  {} ({}) - {} [{:?}] {}", 
                                 issue.title, 
                                 issue.id,
                                 issue.description.as_deref().unwrap_or("No description"), 
                                 issue.status,
                                 assignee_str);
                    }
                }
                Ok(())
            },
            IssueSubcommand::Show { id, project: _ } => {
                // Parse the ID as UUID
                let issue_id = match uuid::Uuid::parse_str(id) {
                    Ok(uuid) => uuid,
                    Err(_) => {
                        eprintln!("❌ Invalid Issue ID");
                        eprintln!("Issue ID must be a valid UUID");
                        eprintln!("💡 Tip: Use 'odi issue list' to see available issues");
                        return Err(crate::OdiError::Core(odi_core::CoreError::ValidationError {
                            field: "issue_id".to_string(),
                            message: format!("Invalid UUID format: {}", id)
                        }));
                    }
                };
                
                match ctx.issue_repository().get(&issue_id).await.map_err(crate::OdiError::Core)? {
                    Some(issue) => {
                        println!("Issue: {}", issue.title);
                        println!("ID: {}", issue.id);
                        if let Some(desc) = &issue.description {
                            println!("Description: {}", desc);
                        }
                        println!("Status: {:?}", issue.status);
                        println!("Priority: {:?}", issue.priority);
                        println!("Author: {}", issue.author);
                        
                        if !issue.assignees.is_empty() {
                            println!("Assignees: {}", issue.assignees.join(", "));
                        }
                        
                        if let Some(project_id) = &issue.project_id {
                            println!("Project: {}", project_id);
                        }
                        
                        if !issue.labels.is_empty() {
                            println!("Labels: {}", issue.labels.iter().map(|l| l.to_string()).collect::<Vec<_>>().join(", "));
                        }
                        
                        println!("Created: {}", issue.created_at.format("%Y-%m-%d %H:%M:%S"));
                        println!("Updated: {}", issue.updated_at.format("%Y-%m-%d %H:%M:%S"));
                        
                        if let Some(closed_at) = &issue.closed_at {
                            println!("Closed: {}", closed_at.format("%Y-%m-%d %H:%M:%S"));
                        }
                    },
                    None => {
                        eprintln!("❌ Issue Not Found");
                        eprintln!("Issue '{}' does not exist", id);
                        eprintln!("💡 Tip: Use 'odi issue list' to see available issues");
                        return Err(crate::OdiError::Core(odi_core::CoreError::ValidationError {
                            field: "issue_id".to_string(),
                            message: format!("Issue '{}' not found", id)
                        }));
                    }
                }
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
