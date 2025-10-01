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
    /// Show remote details
    Show {
        /// Remote name
        name: String,
    },
    /// Remove a remote
    Remove {
        /// Remote name
        name: String,
    },
    /// Check synchronization status with remote
    SyncStatus {
        /// Remote name (defaults to 'origin')
        remote: Option<String>,
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
            RemoteSubcommand::Show { name } => {
                show_remote(ctx, name).await
            },
            RemoteSubcommand::Remove { name } => {
                remove_remote(ctx, name).await
            },
            RemoteSubcommand::SyncStatus { remote } => {
                sync_status(ctx, remote.as_deref()).await
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

async fn show_remote(ctx: &AppContext, name: &str) -> Result<()> {
    let remote_repo = ctx.remote_repository();
    let remotes = remote_repo.list().await
        .map_err(crate::OdiError::Core)?;
    
    let remote = remotes.iter().find(|r| r.name == name);
    
    match remote {
        Some(remote) => {
            println!("Remote: {}", remote.name);
            println!("URL: {}", remote.url);
            println!("Created: {}", remote.created_at.format("%Y-%m-%d %H:%M:%S"));
            if let Some(last_sync) = &remote.last_sync {
                println!("Last Sync: {}", last_sync.format("%Y-%m-%d %H:%M:%S"));
            }
            Ok(())
        },
        None => {
            eprintln!("❌ Remote Not Found");
            eprintln!("Remote '{}' does not exist", name);
            eprintln!();
            eprintln!("💡 Tip: Use 'odi remote list' to see available remotes");
            Err(crate::OdiError::Core(odi_core::CoreError::ValidationError { field: "remote".to_string(), message: format!("Remote '{}' not found", name) }))
        }
    }
}

async fn remove_remote(ctx: &AppContext, name: &str) -> Result<()> {
    let remote_repo = ctx.remote_repository();
    
    // Check if remote exists and delete it
    let existed = remote_repo.delete(&name.to_string()).await.map_err(|e| crate::OdiError::Storage { 
        message: format!("Failed to remove remote: {}", e) 
    })?;
    
    if existed {
        println!("Removed remote: {}", name);
        Ok(())
    } else {
        eprintln!("❌ Remote Not Found");
        eprintln!("Remote '{}' does not exist", name);
        eprintln!();
        eprintln!("💡 Tip: Use 'odi remote list' to see available remotes");
        Err(crate::OdiError::Core(odi_core::CoreError::ValidationError { 
            field: "remote".to_string(), 
            message: format!("Remote '{}' not found", name) 
        }))
    }
}
async fn sync_status(ctx: &AppContext, remote_name: Option<&str>) -> Result<()> {
    let remote_name = remote_name.unwrap_or("origin");
    
    let remote_repo = ctx.remote_repository();
    let remotes = remote_repo.list().await
        .map_err(crate::OdiError::Core)?;
    
    let remote = remotes.iter().find(|r| r.name == remote_name)
        .ok_or_else(|| crate::OdiError::Core(odi_core::CoreError::ValidationError { 
            field: "remote".to_string(), 
            message: format!("Remote '{}' not found", remote_name) 
        }))?;
    
    println!("Synchronization status for remote: {}", remote.name);
    println!("URL: {}", remote.url);
    println!();
    
    // Initialize the remote sync client
    let sync = DefaultRemoteSync::new();
    
    // Attempt to connect to the remote
    match sync.connect(remote).await {
        Ok(client) => {
            println!("✓ Remote is accessible");
            
            // Get sync state from remote
            match sync.get_sync_state(&client).await {
                Ok(state) => {
                    println!("✓ Sync state retrieved:");
                    println!("  Remote issues: {}", state.total_issues);
                    println!("  Pending changes: {}", state.pending_changes);
                    
                    // Get local issue count
                    let issue_repo = ctx.issue_repository();
                    let local_issues = issue_repo.list(odi_core::issue::IssueQuery::default()).await
                        .map_err(crate::OdiError::Core)?;
                    
                    println!("  Local issues: {}", local_issues.len());
                    
                    if state.pending_changes > 0 {
                        println!("⚠️  Synchronization needed");
                        println!("💡 Run 'odi remote pull {}' to sync changes", remote_name);
                    } else {
                        println!("✓ Remote and local are in sync");
                    }
                },
                Err(e) => {
                    println!("✗ Failed to get sync state: {}", e);
                }
            }
        },
        Err(e) => {
            println!("✗ Cannot connect to remote: {}", e);
        }
    }
    
    Ok(())
}
