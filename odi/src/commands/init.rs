//! Init command implementation

use clap::Args;
use std::path::PathBuf;
use odi_fs::{FileSystemStorage, save_config, Config, UserConfig, ProjectConfig};
use std::collections::HashMap;
use crate::Result;

/// Arguments for init command
#[derive(Args)]
#[command(about = "Initialize an ODI workspace in the current directory")]
#[command(long_about = "Initialize a new ODI workspace for distributed issue tracking.
This creates a .odi directory with the necessary storage structure and configuration files.

ODI workspaces are similar to Git repositories but designed specifically for issue tracking.
Once initialized, you can create projects, issues, teams, and sync with remote repositories.")]
pub struct InitArgs {
    /// Create an initial project with this name
    #[arg(long, help = "Create initial project (e.g., --project my-app)")]
    pub project: Option<String>,
    
    /// Associate with existing Git repository  
    #[arg(long, help = "Associate with Git repository path")]
    pub git_repo: Option<PathBuf>,
    
    /// Add initial remote repository
    #[arg(long, help = "Add remote repository URL (e.g., --remote https://github.com/user/repo.git)")]
    pub remote: Option<String>,
    
    /// Use alternative config file
    #[arg(long, help = "Use custom config file path")]
    pub config: Option<PathBuf>,
}

impl InitArgs {
    /// Execute the init command
    pub async fn execute(&self) -> Result<()> {
        println!("🚀 Initializing ODI workspace...");
        
        // Check if already initialized
        if std::path::Path::new(".odi").exists() {
            return Err(crate::OdiError::Command { 
                message: "ODI workspace already initialized in this directory".to_string() 
            });
        }
        
        // T073: Integrate odi-core with odi-fs for persistent storage
        let _storage = FileSystemStorage::init().map_err(|e| {
            crate::OdiError::Storage { 
                message: format!("Failed to initialize storage: {}", e) 
            }
        })?;
        
        println!("✅ Created .odi directory structure");
        println!("✅ Initialized storage engine");
        
        // T077: Implement configuration loading and validation
        // Create default configuration
        let config = Config {
            user: UserConfig {
                name: "Default User".to_string(),
                email: "user@example.com".to_string(),
            },
            project: ProjectConfig {
                name: self.project.as_deref().unwrap_or("main").to_string(),
                description: Some("Default project".to_string()),
                default_branch: Some("main".to_string()),
            },
            remotes: HashMap::new(),
        };
        
        save_config(&config).map_err(|e| {
            crate::OdiError::Config { 
                message: format!("Failed to save configuration: {}", e) 
            }
        })?;
        
        println!("✅ Created default configuration");
        
        // Handle optional parameters
        if let Some(project_name) = &self.project {
            println!("📋 Project '{}' will be created", project_name);
        }
        
        if let Some(_git_path) = &self.git_repo {
            println!("🔗 Git integration will be set up");
        }
        
        if let Some(remote_url) = &self.remote {
            println!("🌐 Remote '{}' will be configured", remote_url);
        }
        
        println!("\n🎉 ODI workspace initialized successfully!");
        println!("💡 Next steps:");
        println!("   • Run 'odi project create <name>' to create a project");
        println!("   • Run 'odi issue create' to create your first issue");
        println!("   • Run 'odi config set user.name \"Your Name\"' to set your name");
        println!("   • Run 'odi config set user.email \"you@example.com\"' to set your email");
        
        Ok(())
    }
}