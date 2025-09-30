//! Project, Workspace, and Label entities

mod repository;
mod workspace;
mod label;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::issue::IssueId;
use crate::user::TeamId;

pub use repository::{ProjectRepository, ProjectQuery, ProjectUpdate, WorkspaceQuery, WorkspaceUpdate, LabelQuery};
pub use workspace::{Workspace, WorkspaceId};
pub use label::{Label, LabelId};

/// Project identifier type
pub type ProjectId = String;

/// Project entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub description: Option<String>,
    pub issues: Vec<IssueId>,
    pub teams: Vec<TeamId>,
    pub labels: Vec<Label>,
    pub workspaces: Vec<WorkspaceId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    /// Create a new project
    pub fn new(id: ProjectId, name: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            description: None,
            issues: Vec::new(),
            teams: Vec::new(),
            labels: Vec::new(),
            workspaces: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Validate project ID (3-100 characters, alphanumeric + ._-)
    pub fn validate_id(id: &str) -> bool {
        id.len() >= 3 && id.len() <= 100 && 
        id.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-')
    }

    /// Validate project name (1-100 characters)
    pub fn validate_name(name: &str) -> bool {
        !name.is_empty() && name.len() <= 100
    }

    /// Add workspace reference
    pub fn add_workspace(&mut self, workspace_id: WorkspaceId) {
        if !self.workspaces.contains(&workspace_id) {
            self.workspaces.push(workspace_id);
            self.updated_at = Utc::now();
        }
    }

    /// Remove workspace reference
    pub fn remove_workspace(&mut self, workspace_id: &WorkspaceId) {
        let old_len = self.workspaces.len();
        self.workspaces.retain(|id| id != workspace_id);
        if self.workspaces.len() != old_len {
            self.updated_at = Utc::now();
        }
    }

    /// Add team to project
    pub fn add_team(&mut self, team_id: TeamId) {
        if !self.teams.contains(&team_id) {
            self.teams.push(team_id);
            self.updated_at = Utc::now();
        }
    }

    /// Remove team from project
    pub fn remove_team(&mut self, team_id: &TeamId) {
        let old_len = self.teams.len();
        self.teams.retain(|id| id != team_id);
        if self.teams.len() != old_len {
            self.updated_at = Utc::now();
        }
    }

    /// Add issue to project
    pub fn add_issue(&mut self, issue_id: IssueId) {
        if !self.issues.contains(&issue_id) {
            self.issues.push(issue_id);
            self.updated_at = Utc::now();
        }
    }

    /// Remove issue from project
    pub fn remove_issue(&mut self, issue_id: &IssueId) {
        let old_len = self.issues.len();
        self.issues.retain(|id| id != issue_id);
        if self.issues.len() != old_len {
            self.updated_at = Utc::now();
        }
    }

    /// Add label to project
    pub fn add_label(&mut self, label: Label) {
        // Check if label with same ID already exists
        if !self.labels.iter().any(|l| l.id == label.id) {
            self.labels.push(label);
            self.updated_at = Utc::now();
        }
    }

    /// Remove label from project
    pub fn remove_label(&mut self, label_id: &LabelId) {
        let old_len = self.labels.len();
        self.labels.retain(|l| &l.id != label_id);
        if self.labels.len() != old_len {
            self.updated_at = Utc::now();
        }
    }

    /// Get label by ID
    pub fn get_label(&self, label_id: &LabelId) -> Option<&Label> {
        self.labels.iter().find(|l| &l.id == label_id)
    }

    /// Update project description
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation() {
        let project = Project::new("my_project".to_string(), "My Project".to_string());
        
        assert_eq!(project.id, "my_project");
        assert_eq!(project.name, "My Project");
        assert!(project.issues.is_empty());
        assert!(project.teams.is_empty());
        assert!(project.workspaces.is_empty());
    }

    #[test]
    fn test_project_validation() {
        assert!(Project::validate_id("my_project"));
        assert!(Project::validate_id("project.v1"));
        assert!(Project::validate_id("project-123"));
        assert!(!Project::validate_id("ab")); // Too short
        assert!(!Project::validate_id("")); // Empty
        
        let long_id = "a".repeat(101);
        assert!(!Project::validate_id(&long_id)); // Too long

        assert!(Project::validate_name("My Project"));
        assert!(!Project::validate_name(""));
        
        let long_name = "a".repeat(101);
        assert!(!Project::validate_name(&long_name));
    }

    #[test]
    fn test_workspace_management() {
        let mut project = Project::new("test_project".to_string(), "Test Project".to_string());
        
        project.add_workspace("workspace1".to_string());
        project.add_workspace("workspace2".to_string());
        project.add_workspace("workspace1".to_string()); // Duplicate should be ignored
        
        assert_eq!(project.workspaces.len(), 2);
        assert!(project.workspaces.contains(&"workspace1".to_string()));
        
        project.remove_workspace(&"workspace1".to_string());
        assert_eq!(project.workspaces.len(), 1);
        assert!(!project.workspaces.contains(&"workspace1".to_string()));
    }
}