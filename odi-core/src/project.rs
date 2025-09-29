//! Project, Workspace, and Label entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::issue::IssueId;
use crate::user::TeamId;

/// Project identifier type
pub type ProjectId = String;

/// Workspace identifier type
pub type WorkspaceId = String;

/// Label identifier type
pub type LabelId = String;

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

/// Workspace entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub path: PathBuf,
    pub projects: Vec<ProjectId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Label entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub id: LabelId,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub created_at: DateTime<Utc>,
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
        }
    }

    /// Remove workspace reference
    pub fn remove_workspace(&mut self, workspace_id: &WorkspaceId) {
        self.workspaces.retain(|id| id != workspace_id);
    }
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

    /// Add project reference (many-to-many relationship)
    pub fn add_project(&mut self, project_id: ProjectId) {
        if !self.projects.contains(&project_id) {
            self.projects.push(project_id);
        }
    }

    /// Remove project reference
    pub fn remove_project(&mut self, project_id: &ProjectId) {
        self.projects.retain(|id| id != project_id);
    }
}

impl Label {
    /// Create a new label
    pub fn new(id: LabelId, name: String, color: String) -> Self {
        Self {
            id,
            name,
            description: None,
            color,
            created_at: Utc::now(),
        }
    }

    /// Validate hex color format
    pub fn validate_color(color: &str) -> bool {
        color.len() == 7 && 
        color.starts_with('#') && 
        color[1..].chars().all(|c| c.is_ascii_hexdigit())
    }

    /// Validate label name (1-50 characters)
    pub fn validate_name(name: &str) -> bool {
        !name.is_empty() && name.len() <= 50
    }
}