//! Issue entity and related types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::user::UserId;
use crate::project::{ProjectId, LabelId};

/// Issue identifier type
pub type IssueId = Uuid;

/// Issue status enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssueStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
}

/// Issue priority enumeration  
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Core Issue entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: IssueId,
    pub title: String,
    pub description: Option<String>,
    pub status: IssueStatus,
    pub priority: Priority,
    pub assignees: Vec<UserId>,
    pub author: UserId,
    pub co_authors: Vec<UserId>,
    pub labels: Vec<LabelId>,
    pub project_id: Option<ProjectId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub git_refs: Vec<String>, // Placeholder for GitRef
}

impl Issue {
    /// Create a new issue
    pub fn new(title: String, author: UserId) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description: None,
            status: IssueStatus::Open,
            priority: Priority::Medium,
            assignees: Vec::new(),
            author,
            co_authors: Vec::new(),
            labels: Vec::new(),
            project_id: None,
            created_at: now,
            updated_at: now,
            closed_at: None,
            git_refs: Vec::new(),
        }
    }
}