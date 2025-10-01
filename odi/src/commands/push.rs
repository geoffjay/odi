//! Push command implementation

use clap::Args;
use crate::{Result, AppContext};
use odi_core::{RemoteRepository, IssueRepository};
use odi_net::{RemoteSync, DefaultRemoteSync};

#[derive(Args)]
pub struct PushArgs {
    /// Remote name (defaults to 'origin')
    #[arg(value_name = "REMOTE")]
    pub remote: Option<String>,
    
    /// Force push with potential remote data loss
    #[arg(long)]
    pub force: bool,
    
    /// Show what would be pushed without sending
    #[arg(long)]
    pub dry_run: bool,
}

impl PushArgs {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        push_remote(ctx, self.remote.as_deref(), self.force, self.dry_run).await
    }
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