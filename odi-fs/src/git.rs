//! Git integration

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::Result;
use odi_core::IssueId;

/// Git repository information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitRepository {
    pub path: PathBuf,
    pub remotes: HashMap<String, String>,
    pub current_branch: Option<String>,
}

/// Git reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitRef {
    pub sha: String,
    pub message: Option<String>,
    pub author: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Git integration trait
pub trait GitIntegration {
    fn detect_repository(path: &Path) -> Result<Option<GitRepository>>;
    fn get_current_branch(&self) -> Result<Option<String>>;
    fn get_remote_url(&self, remote: &str) -> Result<Option<String>>;
    fn list_commits(&self, branch: &str) -> Result<Vec<GitRef>>;
    fn associate_issue(&self, issue_id: &IssueId, git_ref: &GitRef) -> Result<()>;
}