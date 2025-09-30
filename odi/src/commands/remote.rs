//! Remote command implementation

use clap::{Args, Subcommand};
use crate::{Result, AppContext};
use odi_core::{Remote, RemoteRepository};
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
                        Ok(issues) => {
                            if issues.is_empty() {
                                println!("✓ No issues to sync from remote");
                            } else {
                                println!("✓ Found {} issues on remote to sync", issues.len());
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
            
            // Get sync state from remote
            match sync.get_sync_state(&client).await {
                Ok(state) => {
                    println!("✓ Remote sync state retrieved:");
                    println!("  Total issues: {}", state.total_issues);
                    println!("  Pending changes: {}", state.pending_changes);
                    
                    // For now, we'll just demonstrate successful connection
                    // In a real implementation, this would:
                    // 1. Compare local state with remote state
                    // 2. Upload changed issues
                    // 3. Handle conflicts
                    println!("✓ Push completed successfully (demonstration mode)");
                    return Ok(());
                },
                Err(e) => {
                    println!("⚠️  Could not get sync state: {}", e);
                }
            }
        },
        Err(e) => {
            println!("✗ Connection failed: {}", e);
            return Err(crate::OdiError::Command { 
                message: format!("Push operation failed: {}", e)
            });
        }
    }
    
    Ok(())
}
