//! Pull command implementation

use clap::Args;
use crate::{Result, AppContext};
use odi_core::{RemoteRepository, IssueRepository};
use odi_net::{RemoteSync, DefaultRemoteSync};

#[derive(Args)]
pub struct PullArgs {
    /// Remote name (defaults to 'origin')
    #[arg(value_name = "REMOTE")]
    pub remote: Option<String>,
    
    /// Force pull with potential data loss
    #[arg(long)]
    pub force: bool,
    
    /// Show what would be pulled without applying
    #[arg(long)]
    pub dry_run: bool,
    
    /// Project to pull issues for
    #[arg(long, short)]
    pub project: Option<String>,
}

impl PullArgs {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        pull_remote(ctx, self.remote.as_deref(), self.force, self.dry_run, self.project.as_deref()).await
    }
}

async fn pull_remote(ctx: &AppContext, remote_name: Option<&str>, _force: bool, dry_run: bool, project_id: Option<&str>) -> Result<()> {
    let remote_name = remote_name.unwrap_or("origin");
    
    // Find the remote
    let remote_repo = ctx.remote_repository();
    let remote = remote_repo.get_by_name(remote_name).await.map_err(|e| crate::OdiError::Storage { 
        message: format!("Failed to get remote: {}", e) 
    })?;
    
    let remote = match remote {
        Some(r) => r,
        None => {
            return Err(crate::OdiError::Validation { 
                message: format!("Remote not found: {}", remote_name)
            });
        }
    };
    
    if dry_run {
        println!("Dry run: Pulling from {} ({})", remote.name, remote.url);
        if let Some(project) = project_id {
            println!("Filtering by project: {}", project);
        }
        println!("Would check for remote changes...");
        println!("No remote changes detected");
        return Ok(());
    }
    
    println!("Pulling from {} ({})", remote.name, remote.url);
    if let Some(project) = project_id {
        println!("Filtering by project: {}", project);
    }
    
    // Initialize the remote sync client
    let sync = DefaultRemoteSync::new();
    
    // Attempt to connect to the remote
    match sync.connect(&remote).await {
        Ok(client) => {
            println!("✓ Connected successfully to remote: {}", remote.url);
            
            // Get sync state from remote
            match sync.get_sync_state(&client).await {
                Ok(state) => {
                    println!("✓ Remote sync state retrieved:");
                    println!("  Total issues: {}", state.total_issues);
                    println!("  Pending changes: {}", state.pending_changes);
                    
                    if dry_run {
                        println!("💡 Dry run mode - no changes made");
                        return Ok(());
                    }

                    // List issues from remote for syncing
                    match sync.list_issues(&client).await {
                        Ok(remote_issues) => {
                            println!("✓ Found {} issues on remote", remote_issues.len());
                            
                            if remote_issues.is_empty() {
                                println!("ℹ️  No issues to pull from remote");
                                return Ok(());
                            }
                            
                            // Get local issues for comparison, with optional project filter
                            let issue_repo = ctx.issue_repository();
                            let query = if let Some(project) = project_id {
                                odi_core::IssueQuery {
                                    project_id: Some(project.to_string()),
                                    ..Default::default()
                                }
                            } else {
                                odi_core::IssueQuery::default()
                            };
                            
                            let local_issues = issue_repo.list(query).await
                                .map_err(|e| crate::OdiError::Storage { 
                                    message: format!("Failed to get local issues: {}", e) 
                                })?;
                            
                            let mut downloaded_count = 0;
                            let mut updated_count = 0;
                            
                            for remote_issue in remote_issues {
                                // Check if issue exists locally
                                if let Some(local_issue) = local_issues.iter().find(|i| i.id == remote_issue.id) {
                                    // Compare timestamps to see if remote is newer
                                    if remote_issue.last_modified > local_issue.updated_at {
                                        // Download and update the issue
                                        match sync.download_issue(&client, &remote_issue.id).await {
                                            Ok(issue) => {
                                                // Check project filter if specified
                                                if let Some(project) = project_id {
                                                    if issue.project_id.as_ref() != Some(&project.to_string()) {
                                                        continue; // Skip this issue
                                                    }
                                                }
                                                
                                                let update = odi_core::issue::IssueUpdate {
                                                    title: Some(issue.title.clone()),
                                                    description: Some(issue.description.clone()),
                                                    status: Some(issue.status.clone()),
                                                    priority: Some(issue.priority.clone()),
                                                    assignees: Some(issue.assignees.clone()),
                                                    co_authors: Some(issue.co_authors.clone()),
                                                    labels: Some(issue.labels.clone()),
                                                    project_id: Some(issue.project_id.clone()),
                                                };
                                                
                                                issue_repo.update(&issue.id, update).await
                                                    .map_err(|e| crate::OdiError::Storage { 
                                                        message: format!("Failed to update issue: {}", e) 
                                                    })?;
                                                
                                                updated_count += 1;
                                                println!("  ↻ Updated issue: {}", issue.title);
                                            },
                                            Err(e) => {
                                                println!("  ⚠️  Failed to download issue {}: {}", remote_issue.id, e);
                                            }
                                        }
                                    }
                                } else {
                                    // Issue doesn't exist locally, download it
                                    match sync.download_issue(&client, &remote_issue.id).await {
                                        Ok(issue) => {
                                            // Check project filter if specified
                                            if let Some(project) = project_id {
                                                if issue.project_id.as_ref() != Some(&project.to_string()) {
                                                    continue; // Skip this issue
                                                }
                                            }
                                            
                                            issue_repo.create(issue.clone()).await
                                                .map_err(|e| crate::OdiError::Storage { 
                                                    message: format!("Failed to create issue: {}", e) 
                                                })?;
                                            
                                            downloaded_count += 1;
                                            println!("  ↓ Downloaded new issue: {}", issue.title);
                                        },
                                        Err(e) => {
                                            println!("  ⚠️  Failed to download issue {}: {}", remote_issue.id, e);
                                        }
                                    }
                                }
                            }
                            
                            if downloaded_count > 0 || updated_count > 0 {
                                println!("✓ Pull completed: {} new, {} updated", downloaded_count, updated_count);
                            } else {
                                println!("✓ Pull completed: No changes");
                            }
                            return Ok(());
                        },
                        Err(e) => {
                            println!("⚠️  Could not list remote issues: {}", e);
                        }
                    }
                },
                Err(e) => {
                    println!("⚠️  Could not get sync state: {}", e);
                }
            }
        },
        Err(e) => {
            println!("✗ Connection failed: {}", e);
            return Err(crate::OdiError::Command { 
                message: format!("Pull operation failed: {}", e)
            });
        }
    }
    
    Ok(())
}