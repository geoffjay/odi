//! Workspace entity and operations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::project::ProjectId;

/// Workspace identifier type
pub type WorkspaceId = String;

/// Workspace entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub path: PathBuf,
    pub projects: Vec<ProjectId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Workspace {
    /// Create a new workspace
    pub fn new(id: WorkspaceId, path: PathBuf) -> Self {
        let now = Utc::now();
        Self {
            id,
            path,
            projects: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Validate workspace ID (3-100 characters, alphanumeric + ._-)
    pub fn validate_id(id: &str) -> bool {
        id.len() >= 3 && id.len() <= 100 && 
        id.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-')
    }

    /// Add project reference (many-to-many relationship)
    pub fn add_project(&mut self, project_id: ProjectId) {
        if !self.projects.contains(&project_id) {
            self.projects.push(project_id);
            self.updated_at = Utc::now();
        }
    }

    /// Remove project reference
    pub fn remove_project(&mut self, project_id: &ProjectId) {
        let old_len = self.projects.len();
        self.projects.retain(|id| id != project_id);
        if self.projects.len() != old_len {
            self.updated_at = Utc::now();
        }
    }

    /// Check if workspace contains project
    pub fn has_project(&self, project_id: &ProjectId) -> bool {
        self.projects.contains(project_id)
    }

    /// Get project count
    pub fn project_count(&self) -> usize {
        self.projects.len()
    }

    /// Update workspace path
    pub fn set_path(&mut self, path: PathBuf) {
        self.path = path;
        self.updated_at = Utc::now();
    }

    /// Check if the workspace path exists
    pub fn path_exists(&self) -> bool {
        self.path.exists()
    }

    /// Check if the workspace path is a directory
    pub fn is_directory(&self) -> bool {
        self.path.is_dir()
    }

    /// Get absolute path
    pub fn absolute_path(&self) -> Result<PathBuf, std::io::Error> {
        self.path.canonicalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_workspace_creation() {
        let path = PathBuf::from("/tmp/workspace");
        let workspace = Workspace::new("workspace1".to_string(), path.clone());
        
        assert_eq!(workspace.id, "workspace1");
        assert_eq!(workspace.path, path);
        assert!(workspace.projects.is_empty());
        assert_eq!(workspace.project_count(), 0);
    }

    #[test]
    fn test_workspace_validation() {
        assert!(Workspace::validate_id("workspace1"));
        assert!(Workspace::validate_id("work.space"));
        assert!(Workspace::validate_id("work-space"));
        assert!(!Workspace::validate_id("ab")); // Too short
        assert!(!Workspace::validate_id("")); // Empty
        
        let long_id = "a".repeat(101);
        assert!(!Workspace::validate_id(&long_id)); // Too long
    }

    #[test]
    fn test_project_management() {
        let path = PathBuf::from("/tmp/workspace");
        let mut workspace = Workspace::new("workspace1".to_string(), path);
        
        workspace.add_project("project1".to_string());
        workspace.add_project("project2".to_string());
        workspace.add_project("project1".to_string()); // Duplicate should be ignored
        
        assert_eq!(workspace.project_count(), 2);
        assert!(workspace.has_project(&"project1".to_string()));
        assert!(workspace.has_project(&"project2".to_string()));
        assert!(!workspace.has_project(&"project3".to_string()));
        
        workspace.remove_project(&"project1".to_string());
        assert_eq!(workspace.project_count(), 1);
        assert!(!workspace.has_project(&"project1".to_string()));
        assert!(workspace.has_project(&"project2".to_string()));
    }

    #[test]
    fn test_path_operations() {
        let mut workspace = Workspace::new(
            "workspace1".to_string(), 
            PathBuf::from("/tmp/workspace")
        );
        
        // Update path
        let new_path = PathBuf::from("/tmp/new_workspace");
        workspace.set_path(new_path.clone());
        assert_eq!(workspace.path, new_path);
        
        // Test with current directory (should exist)
        let current_dir = env::current_dir().unwrap();
        workspace.set_path(current_dir.clone());
        assert!(workspace.path_exists());
        assert!(workspace.is_directory());
    }
}