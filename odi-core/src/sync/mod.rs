//! Synchronization and Remote entities

mod engine;
mod remote;
mod repository;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::issue::{Issue, IssueId};

pub use engine::{SyncEngine, SyncOptions, SyncStats, BatchConflictStrategy, RemoteInfo, detect_conflict_type};
pub use remote::Remote;
pub use repository::RemoteRepository;

/// Remote identifier type
pub type RemoteId = String;

/// Synchronization result
#[derive(Debug, Clone)]
pub struct SyncResult {
    pub pulled_issues: Vec<IssueId>,
    pub pushed_issues: Vec<IssueId>,
    pub conflicts: Vec<Conflict>,
    pub sync_time: DateTime<Utc>,
}

/// Conflict between local and remote versions
#[derive(Debug, Clone)]
pub struct Conflict {
    pub issue_id: IssueId,
    pub local_version: Issue,
    pub remote_version: Issue,
    pub conflict_type: ConflictType,
    pub detected_at: DateTime<Utc>,
}

/// Type of conflict detected
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictType {
    /// Content conflicts (title, description, etc.)
    ContentConflict,
    /// Status transition conflicts
    StatusConflict,
    /// Assignment conflicts
    AssignmentConflict,
    /// Label conflicts
    LabelConflict,
    /// Metadata conflicts (timestamps, etc.)
    MetadataConflict,
}

/// Conflict resolution strategy
#[derive(Debug, Clone)]
pub enum ConflictResolution {
    /// Accept the local version
    AcceptLocal,
    /// Accept the remote version
    AcceptRemote,
    /// Use a manually resolved version
    Manual(Issue),
}

/// Sync operation status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncStatus {
    /// Sync in progress
    InProgress,
    /// Sync completed successfully
    Success,
    /// Sync completed with conflicts
    ConflictsDetected,
    /// Sync failed
    Failed,
}

impl SyncResult {
    /// Create a new sync result
    pub fn new() -> Self {
        Self {
            pulled_issues: Vec::new(),
            pushed_issues: Vec::new(),
            conflicts: Vec::new(),
            sync_time: Utc::now(),
        }
    }
    
    /// Check if sync has conflicts
    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }
    
    /// Get total number of synced issues
    pub fn total_synced(&self) -> usize {
        self.pulled_issues.len() + self.pushed_issues.len()
    }
    
    /// Get conflict count
    pub fn conflict_count(&self) -> usize {
        self.conflicts.len()
    }
    
    /// Add pulled issue
    pub fn add_pulled(&mut self, issue_id: IssueId) {
        if !self.pulled_issues.contains(&issue_id) {
            self.pulled_issues.push(issue_id);
        }
    }
    
    /// Add pushed issue
    pub fn add_pushed(&mut self, issue_id: IssueId) {
        if !self.pushed_issues.contains(&issue_id) {
            self.pushed_issues.push(issue_id);
        }
    }
    
    /// Add conflict
    pub fn add_conflict(&mut self, conflict: Conflict) {
        self.conflicts.push(conflict);
    }
}

impl Conflict {
    /// Create a new conflict
    pub fn new(
        issue_id: IssueId,
        local_version: Issue,
        remote_version: Issue,
        conflict_type: ConflictType,
    ) -> Self {
        Self {
            issue_id,
            local_version,
            remote_version,
            conflict_type,
            detected_at: Utc::now(),
        }
    }
    
    /// Get conflict summary
    pub fn summary(&self) -> String {
        match self.conflict_type {
            ConflictType::ContentConflict => {
                "Content differs between local and remote versions".to_string()
            }
            ConflictType::StatusConflict => {
                format!(
                    "Status conflict: local={:?}, remote={:?}",
                    self.local_version.status, self.remote_version.status
                )
            }
            ConflictType::AssignmentConflict => {
                "Assignment differs between local and remote versions".to_string()
            }
            ConflictType::LabelConflict => {
                "Labels differ between local and remote versions".to_string()
            }
            ConflictType::MetadataConflict => {
                "Metadata differs between local and remote versions".to_string()
            }
        }
    }
    
    /// Check if conflicts can be auto-resolved
    pub fn can_auto_resolve(&self) -> bool {
        match self.conflict_type {
            ConflictType::MetadataConflict => true, // Usually can take latest timestamp
            _ => false, // Content conflicts need manual resolution
        }
    }
}

impl Default for SyncResult {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::issue::{Issue, IssueStatus};
    use uuid::Uuid;

    #[test]
    fn test_sync_result_creation() {
        let result = SyncResult::new();
        
        assert!(result.pulled_issues.is_empty());
        assert!(result.pushed_issues.is_empty());
        assert!(result.conflicts.is_empty());
        assert!(!result.has_conflicts());
        assert_eq!(result.total_synced(), 0);
    }

    #[test]
    fn test_sync_result_operations() {
        let mut result = SyncResult::new();
        
        let issue_id = Uuid::new_v4();
        result.add_pulled(issue_id);
        result.add_pushed(issue_id); // Same issue can be both pulled and pushed
        
        assert_eq!(result.pulled_issues.len(), 1);
        assert_eq!(result.pushed_issues.len(), 1);
        assert_eq!(result.total_synced(), 2);
        
        // Test deduplication
        result.add_pulled(issue_id);
        assert_eq!(result.pulled_issues.len(), 1);
    }

    #[test]
    fn test_conflict_creation() {
        let issue_id = Uuid::new_v4();
        let mut local_issue = Issue::new("Test Issue".to_string(), "alice".to_string());
        let mut remote_issue = local_issue.clone();
        
        local_issue.status = IssueStatus::InProgress;
        remote_issue.status = IssueStatus::Resolved;
        
        let conflict = Conflict::new(
            issue_id,
            local_issue,
            remote_issue,
            ConflictType::StatusConflict,
        );
        
        assert_eq!(conflict.issue_id, issue_id);
        assert_eq!(conflict.conflict_type, ConflictType::StatusConflict);
        assert!(!conflict.can_auto_resolve());
        assert!(conflict.summary().contains("Status conflict"));
    }

    #[test]
    fn test_conflict_auto_resolve() {
        let issue_id = Uuid::new_v4();
        let local_issue = Issue::new("Test Issue".to_string(), "alice".to_string());
        let remote_issue = local_issue.clone();
        
        let metadata_conflict = Conflict::new(
            issue_id,
            local_issue.clone(),
            remote_issue.clone(),
            ConflictType::MetadataConflict,
        );
        
        let content_conflict = Conflict::new(
            issue_id,
            local_issue,
            remote_issue,
            ConflictType::ContentConflict,
        );
        
        assert!(metadata_conflict.can_auto_resolve());
        assert!(!content_conflict.can_auto_resolve());
    }
}