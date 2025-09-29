//! Synchronization and Remote entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::issue::{Issue, IssueId};
use crate::project::ProjectId;

/// Remote identifier type
pub type RemoteId = String;

/// Remote entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Remote {
    pub id: RemoteId,
    pub name: String,
    pub url: String,
    pub projects: Vec<ProjectId>,
    pub last_sync: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Synchronization result
#[derive(Debug, Clone)]
pub struct SyncResult {
    pub pulled_issues: Vec<IssueId>,
    pub pushed_issues: Vec<IssueId>,
    pub conflicts: Vec<Conflict>,
}

/// Conflict type
#[derive(Debug, Clone)]
pub struct Conflict {
    pub issue_id: IssueId,
    pub local_version: Issue,
    pub remote_version: Issue,
    pub conflict_type: ConflictType,
}

/// Type of conflict
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConflictType {
    ContentConflict,
    StatusConflict,
    AssignmentConflict,
}

/// Conflict resolution strategy
#[derive(Debug, Clone)]
pub enum ConflictResolution {
    AcceptLocal,
    AcceptRemote,
    Manual(Issue),
}

/// Synchronization engine trait
#[async_trait::async_trait]
pub trait SyncEngine {
    async fn pull(&self, remote: &Remote) -> crate::Result<SyncResult>;
    async fn push(&self, remote: &Remote) -> crate::Result<SyncResult>;
    async fn resolve_conflict(&self, conflict: &Conflict, resolution: ConflictResolution) -> crate::Result<()>;
}

impl Remote {
    /// Create a new remote
    pub fn new(id: RemoteId, name: String, url: String) -> Self {
        Self {
            id,
            name,
            url,
            projects: Vec::new(),
            last_sync: None,
            created_at: Utc::now(),
        }
    }
}