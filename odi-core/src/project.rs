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
}