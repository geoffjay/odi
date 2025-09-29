//! Integration layer for cross-crate dependencies
//!
//! This module implements T073-T077 from Phase 3.5:
//! - T073: Integrate odi-core with odi-fs for persistent storage
//! - T074: Integrate odi-core with odi-net for remote synchronization  
//! - T075: Implement dependency injection for repository traits
//! - T076: Add comprehensive error handling and user-friendly messages
//! - T077: Implement configuration loading and validation

use std::path::{Path, PathBuf};
use std::sync::Arc;

use odi_fs::{FileSystemStorage, FileConfigLoader, Config, ConfigLoader};
use odi_net::sync::DefaultRemoteSync;
use crate::{Result, OdiError};

/// Application context providing dependency injection
/// 
/// This struct acts as a service locator and provides access to all
/// repository implementations and shared services across the application.
#[derive(Clone)]
pub struct AppContext {
    /// Current workspace path
    workspace_path: PathBuf,
    /// Filesystem storage engine
    storage: Arc<FileSystemStorage>,
    /// Configuration
    config: Config,
    /// Remote sync service
    remote_sync: Arc<DefaultRemoteSync>,
}

impl AppContext {
    /// Initialize application context for a workspace
    /// 
    /// This method implements configuration loading hierarchy:
    /// 1. Load global config from ~/.odi/config
    /// 2. Load local config from workspace/.odi/config
    /// 3. Merge configurations (local takes precedence)
    /// 4. Initialize storage and networking services
    pub async fn new(workspace_path: Option<PathBuf>) -> Result<Self> {
        let workspace_path = workspace_path.unwrap_or_else(|| std::env::current_dir().unwrap());
        
        // T077: Implement configuration loading and validation
        let config = Self::load_configuration(&workspace_path).await?;
        
        // T073: Integrate odi-core with odi-fs for persistent storage
        let storage_path = workspace_path.join(".odi");
        let storage = Arc::new(
            FileSystemStorage::new(storage_path)
                .map_err(|e| OdiError::Storage { 
                    message: format!("Failed to initialize storage: {}", e) 
                })?
        );
        
        // T074: Integrate odi-core with odi-net for remote synchronization
        let remote_sync = Arc::new(DefaultRemoteSync::new());
        
        Ok(Self {
            workspace_path,
            storage,
            config,
            remote_sync,
        })
    }
    
    /// Load and validate configuration from hierarchy
    async fn load_configuration(workspace_path: &Path) -> Result<Config> {
        // Load global configuration
        let global_config = FileConfigLoader::load_global()
            .map_err(|e| OdiError::Config { 
                message: format!("Failed to load global config: {}", e) 
            })?;
            
        // Load local configuration
        let local_config = FileConfigLoader::load_local(workspace_path)
            .map_err(|e| OdiError::Config { 
                message: format!("Failed to load local config: {}", e) 
            })?;
            
        // Merge configurations
        let config = FileConfigLoader::merge(global_config, local_config);
        
        // Validate final configuration
        FileConfigLoader::validate(&config)
            .map_err(|e| OdiError::Config { 
                message: format!("Configuration validation failed: {}", e) 
            })?;
            
        Ok(config)
    }
    
    /// Get current workspace path
    pub fn workspace_path(&self) -> &Path {
        &self.workspace_path
    }
    
    /// Get configuration
    pub fn config(&self) -> &Config {
        &self.config
    }
    
    /// Get storage engine reference
    pub fn storage(&self) -> &Arc<FileSystemStorage> {
        &self.storage
    }
    
    /// Get remote sync service reference
    pub fn remote_sync(&self) -> &Arc<DefaultRemoteSync> {
        &self.remote_sync
    }
    
    /// Check if current directory is an ODI workspace
    pub fn is_odi_workspace(path: Option<&Path>) -> bool {
        let check_path = path.unwrap_or(Path::new("."));
        check_path.join(".odi").exists()
    }
    
    /// Require that current directory is an ODI workspace
    /// 
    /// This method provides user-friendly error messages (T076)
    pub fn require_workspace(path: Option<&Path>) -> Result<()> {
        if !Self::is_odi_workspace(path) {
            let current_path = path.unwrap_or(Path::new("."));
            return Err(OdiError::NotInitialized { 
                message: format!(
                    "Not in an ODI workspace. Current directory: {}\n\
                     Run 'odi init' to initialize a new workspace.",
                    current_path.display()
                )
            });
        }
        Ok(())
    }
    
    /// Initialize ODI workspace in the given directory
    pub async fn init_workspace(path: &Path) -> Result<Self> {
        let odi_path = path.join(".odi");
        
        // Create .odi directory structure
        std::fs::create_dir_all(&odi_path)
            .map_err(|e| OdiError::Io { 
                message: format!("Failed to create .odi directory: {}", e) 
            })?;
            
        // Initialize storage
        let storage = Arc::new(
            FileSystemStorage::new(odi_path)
                .map_err(|e| OdiError::Storage { 
                    message: format!("Failed to initialize storage: {}", e) 
                })?
        );
        
        // Create default configuration
        let config = Config::default();
        
        // Save configuration to local .odi/config
        odi_fs::save_config(&config)
            .map_err(|e| OdiError::Config { 
                message: format!("Failed to save default config: {}", e) 
            })?;
        
        let remote_sync = Arc::new(DefaultRemoteSync::new());
        
        Ok(Self {
            workspace_path: path.to_path_buf(),
            storage,
            config,
            remote_sync,
        })
    }
}