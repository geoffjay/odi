//! Config command implementation

use clap::{Args, Subcommand};
use crate::{Result, AppContext, OdiError};
use odi_fs::{Config, save_config, ConfigLoader, FileConfigLoader};

#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigSubcommand,
}

#[derive(Subcommand)]
pub enum ConfigSubcommand {
    /// Get a configuration value
    Get { 
        /// Configuration key (e.g., user.name, user.email, project.name, remotes.origin.url)
        key: String 
    },
    /// Set a configuration value
    Set { 
        /// Configuration key (e.g., user.name, user.email, project.name, project.description)
        key: String, 
        /// Configuration value
        value: String 
    },
    /// List all configuration values
    List,
}

impl ConfigArgs {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.command {
            ConfigSubcommand::Get { key } => {
                get_config_value(ctx, key).await
            },
            ConfigSubcommand::Set { key, value } => {
                set_config_value(ctx, key, value).await
            },
            ConfigSubcommand::List => {
                list_config_values(ctx).await
            },
        }
    }
}

async fn get_config_value(ctx: &AppContext, key: &str) -> Result<()> {
    let config = ctx.config();
    
    match get_value_by_path(config, key) {
        Some(value) => {
            println!("{}", value);
            Ok(())
        },
        None => {
            return Err(OdiError::Config { 
                message: format!("Configuration key '{}' not found", key)
            });
        }
    }
}

async fn set_config_value(ctx: &AppContext, key: &str, value: &str) -> Result<()> {
    // Load current configuration
    let mut config = ctx.config().clone();
    
    // Update the configuration value
    set_value_by_path(&mut config, key, value)?;
    
    // Validate the updated configuration
    FileConfigLoader::validate(&config).map_err(|e| {
        OdiError::Config { 
            message: format!("Configuration validation failed: {}", e) 
        }
    })?;
    
    // Save the updated configuration
    save_config(&config).map_err(|e| {
        OdiError::Config { 
            message: format!("Failed to save configuration: {}", e) 
        }
    })?;
    
    println!("Configuration updated: {} = {}", key, value);
    Ok(())
}

async fn list_config_values(ctx: &AppContext) -> Result<()> {
    let config = ctx.config();
    
    println!("USER");
    println!("  user.name = {}", config.user.name);
    println!("  user.email = {}", config.user.email);
    println!();
    
    println!("PROJECT");
    println!("  project.name = {}", config.project.name);
    if let Some(ref description) = config.project.description {
        println!("  project.description = {}", description);
    }
    if let Some(ref branch) = config.project.default_branch {
        println!("  project.default_branch = {}", branch);
    }
    println!();
    
    if !config.remotes.is_empty() {
        println!("REMOTES");
        for (name, remote) in &config.remotes {
            println!("  remotes.{}.url = {}", name, remote.url);
            println!("  remotes.{}.protocol = {}", name, remote.protocol);
        }
    }
    
    Ok(())
}

fn get_value_by_path(config: &Config, key: &str) -> Option<String> {
    let parts: Vec<&str> = key.split('.').collect();
    
    match parts.as_slice() {
        ["user", "name"] => Some(config.user.name.clone()),
        ["user", "email"] => Some(config.user.email.clone()),
        ["project", "name"] => Some(config.project.name.clone()),
        ["project", "description"] => config.project.description.clone(),
        ["project", "default_branch"] => config.project.default_branch.clone(),
        ["remotes", remote_name, "url"] => {
            config.remotes.get(*remote_name).map(|r| r.url.clone())
        },
        ["remotes", remote_name, "protocol"] => {
            config.remotes.get(*remote_name).map(|r| r.protocol.clone())
        },
        _ => None,
    }
}

fn set_value_by_path(config: &mut Config, key: &str, value: &str) -> Result<()> {
    let parts: Vec<&str> = key.split('.').collect();
    
    match parts.as_slice() {
        ["user", "name"] => {
            if value.trim().is_empty() {
                return Err(OdiError::Validation { 
                    message: "User name cannot be empty".to_string() 
                });
            }
            config.user.name = value.to_string();
        },
        ["user", "email"] => {
            if value.trim().is_empty() || !value.contains('@') {
                return Err(OdiError::Validation { 
                    message: "User email must be a valid email address".to_string() 
                });
            }
            config.user.email = value.to_string();
        },
        ["project", "name"] => {
            if value.trim().is_empty() {
                return Err(OdiError::Validation { 
                    message: "Project name cannot be empty".to_string() 
                });
            }
            config.project.name = value.to_string();
        },
        ["project", "description"] => {
            config.project.description = if value.trim().is_empty() {
                None
            } else {
                Some(value.to_string())
            };
        },
        ["project", "default_branch"] => {
            config.project.default_branch = if value.trim().is_empty() {
                None
            } else {
                Some(value.to_string())
            };
        },
        ["remotes", remote_name, "url"] => {
            if let Some(remote) = config.remotes.get_mut(*remote_name) {
                remote.url = value.to_string();
            } else {
                return Err(OdiError::Config { 
                    message: format!("Remote '{}' does not exist. Use 'odi remote add' first.", remote_name) 
                });
            }
        },
        ["remotes", remote_name, "protocol"] => {
            if !["ssh", "https"].contains(&value) {
                return Err(OdiError::Validation { 
                    message: "Protocol must be 'ssh' or 'https'".to_string() 
                });
            }
            if let Some(remote) = config.remotes.get_mut(*remote_name) {
                remote.protocol = value.to_string();
            } else {
                return Err(OdiError::Config { 
                    message: format!("Remote '{}' does not exist. Use 'odi remote add' first.", remote_name) 
                });
            }
        },
        _ => {
            return Err(OdiError::Config { 
                message: format!("Unknown configuration key: {}. Supported keys: user.name, user.email, project.name, project.description, project.default_branch, remotes.<name>.url, remotes.<name>.protocol", key) 
            });
        }
    }
    
    Ok(())
}
