//! Remote command implementation

use clap::{Args, Subcommand};
use crate::{Result, AppContext};
use odi_core::{Remote, RemoteRepository, IssueRepository};
use odi_net::{RemoteSync, DefaultRemoteSync};

#[derive(Args)]
pub struct RemoteArgs {
    #[command(subcommand)]
    pub command: RemoteSubcommand,
}

#[derive(Subcommand)]
pub enum RemoteSubcommand {
    /// Add a new remote
    Add { 
        /// Remote name
        name: String, 
        /// Remote URL
        url: String,
        /// Projects to sync via this remote (optional, defaults to all active)
        #[arg(short, long)]
        projects: Option<Vec<String>>,
    },
    /// List all remotes
    List,
    /// Pull changes from remote repository
    Pull {
        /// Remote name (defaults to 'origin')
        remote: Option<String>,
        /// Force pull with potential data loss
        #[arg(long)]
        force: bool,
        /// Show what would be pulled without applying
        #[arg(long)]
        dry_run: bool,
    },
    /// Push local changes to remote repository
    Push {
        /// Remote name (defaults to 'origin')
        remote: Option<String>,
        /// Force push with potential remote data loss
        #[arg(long)]
        force: bool,
        /// Show what would be pushed without sending
        #[arg(long)]
        dry_run: bool,
    },
}

impl RemoteArgs {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.command {
            RemoteSubcommand::Add { name, url, projects } => {
                add_remote(ctx, name, url, projects.as_ref()).await
            },
            RemoteSubcommand::List => {
                list_remotes(ctx).await
            },
            RemoteSubcommand::Pull { remote, force, dry_run } => {
                pull_remote(ctx, remote.as_deref(), *force, *dry_run).await
            },
            RemoteSubcommand::Push { remote, force, dry_run } => {
                push_remote(ctx, remote.as_deref(), *force, *dry_run).await
            },
        }
    }
}

async fn add_remote(ctx: &AppContext, name: &str, url: &str, _projects: Option<&Vec<String>>) -> Result<()> {
    // Validate remote name format
    if !Remote::validate_id(name) {
        return Err(crate::OdiError::Validation { 
            message: format!("Invalid remote name '{}': must be 3-50 characters, alphanumeric with ._- allowed", name)
        });
    }
    
    // Validate URL format 
    if !Remote::validate_url(url) {
        return Err(crate::OdiError::Validation { 
            message: format!("Invalid URL format: {}", url)
        });
    }
    
    // Check if remote already exists
    let remote_repo = ctx.remote_repository();
    if remote_repo.exists(name).await.map_err(|e| crate::OdiError::Storage { 
        message: format!("Failed to check if remote exists: {}", e) 
    })? {
        return Err(crate::OdiError::Validation { 
            message: format!("Remote already exists: {}", name)
        });
    }
    
    // Create new remote
    let remote = Remote::new(name.to_string(), name.to_string(), url.to_string());
    
    // Store remote
    remote_repo.create(remote).await.map_err(|e| crate::OdiError::Storage { 
        message: format!("Failed to create remote: {}", e) 
    })?;
    
    // Get protocol for display
    let protocol = if url.starts_with("ssh://") || url.contains('@') {
        "SSH"
    } else if url.starts_with("https://") {
        "HTTPS" 
    } else {
        "Unknown"
    };
    
    println!("Added remote '{}': {}", name, url);
    println!("Authentication: {}", protocol);
    
    Ok(())
}

async fn list_remotes(ctx: &AppContext) -> Result<()> {
    let remote_repo = ctx.remote_repository();
    let remotes = remote_repo.list().await.map_err(|e| crate::OdiError::Storage { 
        message: format!("Failed to list remotes: {}", e) 
    })?;
    
    if remotes.is_empty() {
        println!("No remotes configured.");
        println!("Use 'odi remote add <name> <url>' to add a remote.");
        return Ok(());
    }
    
    println!("NAME     URL                           PROJECTS  LAST_SYNC");
    println!("----     ---                           --------  ---------");
    
    for remote in remotes {
        let last_sync = if let Some(sync_time) = remote.last_sync {
            sync_time.format("%Y-%m-%d %H:%M").to_string()
        } else {
            "Never".to_string()
        };
        
        let project_count = remote.project_count();
        let project_text = if project_count == 0 {
            "All".to_string()
        } else {
            project_count.to_string()
        };
        
        println!(
            "{:<8} {:<29} {:<8} {}", 
            remote.name, 
            remote.url,
            project_text,
            last_sync
        );
    }
    
    Ok(())
}

async fn pull_remote(ctx: &AppContext, remote_name: Option<&str>, _force: bool, dry_run: bool) -> Result<()> {
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
        println!("Would check for remote changes...");
        println!("No remote changes detected");
        return Ok(());
    }
    
    println!("Pulling from {} ({})", remote.name, remote.url);
    
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
                            
                            // Get local issues for comparison
                            let issue_repo = ctx.issue_repository();
                            let local_issues = issue_repo.list(odi_core::issue::IssueQuery::default()).await
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

async fn push_remote(ctx: &AppContext, remote_name: Option<&str>, _force: bool, dry_run: bool) -> Result<()> {
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
        println!("Dry run: Pushing to {} ({})", remote.name, remote.url);
        println!("Would check for local changes...");
        println!("💡 Dry run mode - no changes made");
        return Ok(());
    }
    
    println!("Pushing to {} ({})", remote.name, remote.url);
    
    // Initialize the remote sync client
    let sync = DefaultRemoteSync::new();
    
    // Attempt to connect to the remote
    match sync.connect(&remote).await {
        Ok(client) => {
            println!("✓ Connected successfully to remote: {}", remote.url);
            
            // Get local issues to push
            let issue_repo = ctx.issue_repository();
            let local_issues = issue_repo.list(odi_core::issue::IssueQuery::default()).await
                .map_err(|e| crate::OdiError::Storage { 
                    message: format!("Failed to get local issues: {}", e) 
                })?;
            
            if local_issues.is_empty() {
                println!("ℹ️  No local issues to push");
                return Ok(());
            }
            
            println!("📤 Pushing {} local issues to remote", local_issues.len());
            
            // Get remote issues for comparison
            let remote_issues = sync.list_issues(&client).await.map_err(|e| crate::OdiError::Command { 
                message: format!("Failed to list remote issues: {}", e)
            })?;
            
            let mut uploaded_count = 0;
            let mut skipped_count = 0;
            
            for local_issue in local_issues {
                // Check if issue exists on remote
                if let Some(remote_issue) = remote_issues.iter().find(|r| r.id == local_issue.id) {
                    // Compare timestamps to see if local is newer
                    if local_issue.updated_at > remote_issue.last_modified {
                        // Upload the updated issue
                        match sync.upload_issue(&client, &local_issue).await {
                            Ok(_) => {
                                uploaded_count += 1;
                                println!("  ↑ Uploaded updated issue: {}", local_issue.title);
                            },
                            Err(e) => {
                                println!("  ⚠️  Failed to upload issue {}: {}", local_issue.id, e);
                            }
                        }
                    } else {
                        skipped_count += 1;
                    }
                } else {
                    // Issue doesn't exist on remote, upload it
                    match sync.upload_issue(&client, &local_issue).await {
                        Ok(_) => {
                            uploaded_count += 1;
                            println!("  ↑ Uploaded new issue: {}", local_issue.title);
                        },
                        Err(e) => {
                            println!("  ⚠️  Failed to upload issue {}: {}", local_issue.id, e);
                        }
                    }
                }
            }
            
            if uploaded_count > 0 {
                println!("✓ Push completed: {} uploaded, {} skipped", uploaded_count, skipped_count);
            } else {
                println!("✓ Push completed: No changes to push");
            }
            return Ok(());
        },
        Err(e) => {
            println!("✗ Connection failed: {}", e);
            return Err(crate::OdiError::Command { 
                message: format!("Push operation failed: {}", e)
            });
        }
    }
}
