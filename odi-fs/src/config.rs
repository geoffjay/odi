use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),
    
    #[error("Validation error: {message}")]
    Validation { message: String },
}

pub type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub user: UserConfig,
    pub project: ProjectConfig,
    pub remotes: HashMap<String, RemoteConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserConfig {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectConfig {
    pub name: String,
    pub description: Option<String>,
    pub default_branch: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RemoteConfig {
    pub url: String,
    pub protocol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkspaceConfig {
    pub name: String,
    pub path: PathBuf,
    pub projects: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            user: UserConfig {
                name: "Unknown User".to_string(),
                email: "user@example.com".to_string(),
            },
            project: ProjectConfig {
                name: "default".to_string(),
                description: None,
                default_branch: Some("main".to_string()),
            },
            remotes: HashMap::new(),
        }
    }
}

pub trait ConfigLoader {
    fn load_global() -> crate::Result<Option<Config>>;
    fn load_local(workspace_path: &Path) -> crate::Result<Option<Config>>;
    fn merge(global: Option<Config>, local: Option<Config>) -> Config;
    fn validate(config: &Config) -> crate::Result<()>;
}

pub struct FileConfigLoader;

impl ConfigLoader for FileConfigLoader {
    fn load_global() -> crate::Result<Option<Config>> {
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let global_path = PathBuf::from(home_dir).join(".odi").join("config");
        
        if global_path.exists() {
            let content = fs::read_to_string(&global_path)
                .map_err(|e| crate::FsError::Io(e))?;
            let config: Config = toml::from_str(&content)
                .map_err(|e| crate::FsError::TomlError(e))?;
            Self::validate(&config)?;
            Ok(Some(config))
        } else {
            Ok(None)
        }
    }
    
    fn load_local(workspace_path: &Path) -> crate::Result<Option<Config>> {
        let local_path = workspace_path.join(".odi").join("config");
        
        if local_path.exists() {
            let content = fs::read_to_string(local_path)
                .map_err(|e| crate::FsError::Io(e))?;
            let config: Config = toml::from_str(&content)
                .map_err(|e| crate::FsError::TomlError(e))?;
            Self::validate(&config)?;
            Ok(Some(config))
        } else {
            Ok(None)
        }
    }
    
    fn merge(global: Option<Config>, local: Option<Config>) -> Config {
        match (global, local) {
            (None, None) => Config::default(),
            (Some(g), None) => g,
            (None, Some(l)) => l,
            (Some(mut g), Some(l)) => {
                // Local config takes precedence
                g.user = l.user;
                g.project = l.project;
                // Merge remotes (local takes precedence)
                for (name, remote) in l.remotes {
                    g.remotes.insert(name, remote);
                }
                g
            }
        }
    }
    
    fn validate(config: &Config) -> crate::Result<()> {
        if config.user.name.trim().is_empty() {
            return Err(crate::FsError::ConfigError {
                message: "User name cannot be empty".to_string(),
            });
        }
        
        if config.user.email.trim().is_empty() {
            return Err(crate::FsError::ConfigError {
                message: "User email cannot be empty".to_string(),
            });
        }
        
        if !config.user.email.contains('@') {
            return Err(crate::FsError::ConfigError {
                message: "User email must be valid".to_string(),
            });
        }
        
        if config.project.name.trim().is_empty() {
            return Err(crate::FsError::ConfigError {
                message: "Project name cannot be empty".to_string(),
            });
        }
        
        // Validate remotes
        for (name, remote) in &config.remotes {
            if name.trim().is_empty() {
                return Err(crate::FsError::ConfigError {
                    message: "Remote name cannot be empty".to_string(),
                });
            }
            
            if remote.url.trim().is_empty() {
                return Err(crate::FsError::ConfigError {
                    message: format!("Remote '{}' URL cannot be empty", name),
                });
            }
            
            if !["ssh", "https"].contains(&remote.protocol.as_str()) {
                return Err(crate::FsError::ConfigError {
                    message: format!("Remote '{}' protocol must be 'ssh' or 'https'", name),
                });
            }
        }
        
        Ok(())
    }
}

/// Load configuration with hierarchy: global < local
pub fn load_config() -> crate::Result<Config> {
    let global = FileConfigLoader::load_global()?;
    let local = FileConfigLoader::load_local(Path::new("."))?;
    Ok(FileConfigLoader::merge(global, local))
}

/// Save configuration to local .odi/config file
pub fn save_config(config: &Config) -> crate::Result<()> {
    FileConfigLoader::validate(config)?;
    
    // Ensure .odi directory exists
    let odi_dir = Path::new(".odi");
    if !odi_dir.exists() {
        fs::create_dir_all(odi_dir).map_err(|e| crate::FsError::Io(e))?;
    }
    
    let config_path = odi_dir.join("config");
    let toml_string = toml::to_string_pretty(config).map_err(|e| {
        crate::FsError::ConfigError {
            message: format!("Failed to serialize config: {}", e),
        }
    })?;
    
    fs::write(config_path, toml_string).map_err(|e| crate::FsError::Io(e))?;
    Ok(())
}