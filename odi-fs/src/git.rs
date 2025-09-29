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

/// Default Git integration implementation
pub struct DefaultGitIntegration {
    repo_path: PathBuf,
}

impl DefaultGitIntegration {
    pub fn new(repo_path: PathBuf) -> Self {
        Self { repo_path }
    }
}

impl GitIntegration for DefaultGitIntegration {
    fn detect_repository(path: &Path) -> Result<Option<GitRepository>> {
        let git_dir = path.join(".git");
        
        if git_dir.exists() {
            let mut remotes = HashMap::new();
            
            // Try to read Git config for remotes
            let config_path = git_dir.join("config");
            if config_path.exists() {
                // Simple parsing - in production, would use git2 crate
                if let Ok(content) = std::fs::read_to_string(config_path) {
                    // Very basic remote parsing
                    if content.contains("origin") {
                        remotes.insert("origin".to_string(), "unknown".to_string());
                    }
                }
            }
            
            let current_branch = Self::read_current_branch(&git_dir);
            
            Ok(Some(GitRepository {
                path: path.to_path_buf(),
                remotes,
                current_branch,
            }))
        } else {
            Ok(None)
        }
    }

    fn get_current_branch(&self) -> Result<Option<String>> {
        let git_dir = self.repo_path.join(".git");
        Ok(Self::read_current_branch(&git_dir))
    }

    fn get_remote_url(&self, remote: &str) -> Result<Option<String>> {
        let git_dir = self.repo_path.join(".git");
        let config_path = git_dir.join("config");
        
        if config_path.exists() {
            let content = std::fs::read_to_string(config_path)?;
            
            // Simple parsing to find remote URL
            let lines: Vec<&str> = content.lines().collect();
            let mut in_remote_section = false;
            
            for line in lines {
                let trimmed = line.trim();
                if trimmed == &format!("[remote \"{}\"]", remote) {
                    in_remote_section = true;
                    continue;
                }
                
                if in_remote_section {
                    if trimmed.starts_with('[') {
                        break; // End of remote section
                    }
                    
                    if trimmed.starts_with("url = ") {
                        let url = trimmed.strip_prefix("url = ").unwrap_or("").to_string();
                        return Ok(Some(url));
                    }
                }
            }
        }
        
        Ok(None)
    }

    fn list_commits(&self, _branch: &str) -> Result<Vec<GitRef>> {
        // Simple implementation - would use git2 in production
        let git_refs = vec![
            GitRef {
                sha: "a1b2c3d4e5f6789012345678901234567890abcd".to_string(),
                message: Some("Initial commit".to_string()),
                author: Some("Developer <dev@example.com>".to_string()),
                timestamp: Utc::now(),
            }
        ];
        
        Ok(git_refs)
    }

    fn associate_issue(&self, issue_id: &IssueId, _git_ref: &GitRef) -> Result<()> {
        // Create association file
        let associations_dir = self.repo_path.join(".odi").join("git-associations");
        std::fs::create_dir_all(&associations_dir)?;
        
        let association_file = associations_dir.join(format!("{}.txt", issue_id));
        std::fs::write(association_file, format!("Associated with issue {}", issue_id))?;
        
        Ok(())
    }
}

impl DefaultGitIntegration {
    fn read_current_branch(git_dir: &Path) -> Option<String> {
        let head_path = git_dir.join("HEAD");
        if head_path.exists() {
            if let Ok(content) = std::fs::read_to_string(head_path) {
                let trimmed = content.trim();
                if trimmed.starts_with("ref: refs/heads/") {
                    return Some(trimmed.strip_prefix("ref: refs/heads/")?.to_string());
                }
            }
        }
        None
    }
}