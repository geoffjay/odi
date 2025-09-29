//! Configuration management

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

use crate::Result;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub user: UserConfig,
    pub workspace: Option<WorkspaceConfig>,
    pub project: HashMap<String, ProjectConfig>,
    pub remote: HashMap<String, RemoteConfig>,
    pub ui: UiConfig,
    pub sync: SyncConfig,
}

/// User configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub name: String,
    pub email: String,
    pub ssh_key: Option<PathBuf>,
}

/// Workspace configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub active_projects: Vec<String>,
    pub default_assignee: Option<String>,
}

/// Project configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub default_labels: Vec<String>,
    pub git_integration: bool,
}

/// Remote configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteConfig {
    pub url: String,
    pub protocol: String,
    pub projects: Vec<String>,
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub color: String,
    pub pager: bool,
    pub editor: Option<String>,
}

/// Sync configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub auto_pull: bool,
    pub conflict_strategy: String,
    pub compress_objects: bool,
}

/// Configuration loader trait
pub trait ConfigLoader {
    fn load_global() -> Result<Option<Config>>;
    fn load_local(workspace_path: &Path) -> Result<Option<Config>>;
    fn merge(global: Option<Config>, local: Option<Config>) -> Config;
    fn validate(config: &Config) -> Result<()>;
}

/// Configuration loader implementation
pub struct DefaultConfigLoader;

impl ConfigLoader for DefaultConfigLoader {
    fn load_global() -> Result<Option<Config>> {
        let home_dir = std::env::var("HOME").map_err(|_| crate::FsError::ConfigError { 
            message: "HOME environment variable not set".to_string() 
        })?;
        let global_config_path = std::path::PathBuf::from(home_dir).join(".odi").join("config");
        
        if global_config_path.exists() {
            let content = std::fs::read_to_string(global_config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(Some(config))
        } else {
            Ok(None)
        }
    }

    fn load_local(workspace_path: &Path) -> Result<Option<Config>> {
        let local_config_path = workspace_path.join(".odi").join("config");
        
        if local_config_path.exists() {
            let content = std::fs::read_to_string(local_config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(Some(config))
        } else {
            Ok(None)
        }
    }

    fn merge(global: Option<Config>, local: Option<Config>) -> Config {
        match (global, local) {
            (None, None) => Config::default(),
            (Some(global), None) => global,
            (None, Some(local)) => local,
            (Some(mut global), Some(local)) => {
                // Local config overrides global
                global.user = local.user;
                if local.workspace.is_some() {
                    global.workspace = local.workspace;
                }
                // Merge project configs
                for (key, value) in local.project {
                    global.project.insert(key, value);
                }
                // Merge remote configs
                for (key, value) in local.remote {
                    global.remote.insert(key, value);
                }
                // Local UI and sync settings override global
                global.ui = local.ui;
                global.sync = local.sync;
                global
            }
        }
    }

    fn validate(config: &Config) -> Result<()> {
        // Validate user config
        if config.user.name.is_empty() {
            return Err(crate::FsError::ConfigError {
                message: "User name cannot be empty".to_string(),
            });
        }

        if !config.user.email.contains('@') {
            return Err(crate::FsError::ConfigError {
                message: "Invalid email format".to_string(),
            });
        }

        // Validate workspace references existing projects
        if let Some(ref workspace) = config.workspace {
            for project_id in &workspace.active_projects {
                if !config.project.contains_key(project_id) {
                    return Err(crate::FsError::ConfigError {
                        message: format!("Workspace references undefined project: {}", project_id),
                    });
                }
            }
        }

        // Validate UI color setting
        if !["auto", "always", "never"].contains(&config.ui.color.as_str()) {
            return Err(crate::FsError::ConfigError {
                message: format!("Invalid UI color setting: {}", config.ui.color),
            });
        }

        // Validate sync conflict strategy
        if !["manual", "local", "remote"].contains(&config.sync.conflict_strategy.as_str()) {
            return Err(crate::FsError::ConfigError {
                message: format!("Invalid conflict strategy: {}", config.sync.conflict_strategy),
            });
        }

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            user: UserConfig {
                name: "Anonymous".to_string(),
                email: "anonymous@example.com".to_string(),
                ssh_key: None,
            },
            workspace: None,
            project: HashMap::new(),
            remote: HashMap::new(),
            ui: UiConfig {
                color: "auto".to_string(),
                pager: true,
                editor: None,
            },
            sync: SyncConfig {
                auto_pull: false,
                conflict_strategy: "manual".to_string(),
                compress_objects: true,
            },
        }
    }
}