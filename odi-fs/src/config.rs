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