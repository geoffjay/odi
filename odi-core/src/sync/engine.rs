//! Synchronization engine trait and related types

use crate::{
    Result,
    sync::{Remote, SyncResult, Conflict, ConflictResolution, ConflictType},
    issue::{Issue, IssueId},
    project::ProjectId,
};
use chrono::{DateTime, Utc};

/// Sync options for controlling synchronization behavior
#[derive(Debug, Clone)]
pub struct SyncOptions {
    /// Only sync specific projects
    pub projects: Option<Vec<ProjectId>>,
    /// Force sync even if no changes detected
    pub force: bool,
    /// Dry run - don't actually sync, just show what would be synced
    pub dry_run: bool,
    /// Auto-resolve conflicts when possible
    pub auto_resolve: bool,
    /// Maximum number of issues to sync in one operation
    pub batch_size: Option<usize>,
    /// Timeout for sync operation
    pub timeout: Option<std::time::Duration>,
}

/// Sync statistics for reporting
#[derive(Debug, Clone)]
pub struct SyncStats {
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub issues_pulled: usize,
    pub issues_pushed: usize,
    pub conflicts_detected: usize,
    pub conflicts_resolved: usize,
    pub bytes_transferred: u64,
    pub duration: Option<std::time::Duration>,
}

/// Conflict resolution strategy for batch operations
#[derive(Debug, Clone)]
pub enum BatchConflictStrategy {
    /// Stop on first conflict
    StopOnConflict,
    /// Skip conflicts and continue
    SkipConflicts,
    /// Use default resolution for all conflicts
    AutoResolve(ConflictResolution),
}

/// Synchronization engine trait for distributed issue tracking
#[async_trait::async_trait]
pub trait SyncEngine: Send + Sync {
    /// Pull changes from remote
    async fn pull(&self, remote: &Remote, options: SyncOptions) -> Result<SyncResult>;
    
    /// Push changes to remote
    async fn push(&self, remote: &Remote, options: SyncOptions) -> Result<SyncResult>;
    
    /// Synchronize (pull then push)
    async fn sync(&self, remote: &Remote, options: SyncOptions) -> Result<SyncResult>;
    
    /// Resolve a specific conflict
    async fn resolve_conflict(&self, conflict: &Conflict, resolution: ConflictResolution) -> Result<()>;
    
    /// Resolve multiple conflicts with batch strategy
    async fn resolve_conflicts(
        &self, 
        conflicts: &[Conflict], 
        strategy: BatchConflictStrategy
    ) -> Result<Vec<IssueId>>;
    
    /// Check for conflicts without syncing
    async fn check_conflicts(&self, remote: &Remote, project_id: Option<&ProjectId>) -> Result<Vec<Conflict>>;
    
    /// Get sync status for a remote
    async fn get_sync_status(&self, remote: &Remote) -> Result<SyncStats>;
    
    /// Get list of changed issues since last sync
    async fn get_changes_since(&self, remote: &Remote, since: DateTime<Utc>) -> Result<Vec<IssueId>>;
    
    /// Validate remote connectivity
    async fn validate_remote(&self, remote: &Remote) -> Result<bool>;
    
    /// Clone/initialize from remote
    async fn clone(&self, remote: &Remote, target_path: std::path::PathBuf) -> Result<()>;
    
    /// Get remote information without syncing
    async fn get_remote_info(&self, remote: &Remote) -> Result<RemoteInfo>;
}

/// Information about a remote repository
#[derive(Debug, Clone)]
pub struct RemoteInfo {
    pub name: String,
    pub url: String,
    pub last_activity: Option<DateTime<Utc>>,
    pub issue_count: usize,
    pub project_count: usize,
    pub supported_features: Vec<String>,
}

impl SyncOptions {
    /// Create default sync options
    pub fn new() -> Self {
        Self {
            projects: None,
            force: false,
            dry_run: false,
            auto_resolve: false,
            batch_size: Some(100),
            timeout: Some(std::time::Duration::from_secs(300)), // 5 minutes
        }
    }
    
    /// Set specific projects to sync
    pub fn projects(mut self, projects: Vec<ProjectId>) -> Self {
        self.projects = Some(projects);
        self
    }
    
    /// Enable force sync
    pub fn force(mut self) -> Self {
        self.force = true;
        self
    }
    
    /// Enable dry run mode
    pub fn dry_run(mut self) -> Self {
        self.dry_run = true;
        self
    }
    
    /// Enable auto-resolve conflicts
    pub fn auto_resolve(mut self) -> Self {
        self.auto_resolve = true;
        self
    }
    
    /// Set batch size
    pub fn batch_size(mut self, size: usize) -> Self {
        self.batch_size = Some(size);
        self
    }
    
    /// Set timeout
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

impl SyncStats {
    /// Create new sync stats
    pub fn new() -> Self {
        Self {
            started_at: Utc::now(),
            completed_at: None,
            issues_pulled: 0,
            issues_pushed: 0,
            conflicts_detected: 0,
            conflicts_resolved: 0,
            bytes_transferred: 0,
            duration: None,
        }
    }
    
    /// Mark sync as completed
    pub fn complete(&mut self) {
        let now = Utc::now();
        self.completed_at = Some(now);
        self.duration = Some((now - self.started_at).to_std().unwrap_or_default());
    }
    
    /// Check if sync is completed
    pub fn is_completed(&self) -> bool {
        self.completed_at.is_some()
    }
    
    /// Get total issues processed
    pub fn total_issues(&self) -> usize {
        self.issues_pulled + self.issues_pushed
    }
    
    /// Get conflict resolution rate
    pub fn resolution_rate(&self) -> f64 {
        if self.conflicts_detected == 0 {
            1.0
        } else {
            self.conflicts_resolved as f64 / self.conflicts_detected as f64
        }
    }
}

impl Default for SyncOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SyncStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to detect conflict type between two issues
pub fn detect_conflict_type(local: &Issue, remote: &Issue) -> Option<ConflictType> {
    if local.id != remote.id {
        return None; // Not the same issue
    }
    
    // Check for status conflicts
    if local.status != remote.status {
        return Some(ConflictType::StatusConflict);
    }
    
    // Check for assignment conflicts
    if local.assignees != remote.assignees {
        return Some(ConflictType::AssignmentConflict);
    }
    
    // Check for label conflicts
    if local.labels != remote.labels {
        return Some(ConflictType::LabelConflict);
    }
    
    // Check for content conflicts
    if local.title != remote.title || local.description != remote.description {
        return Some(ConflictType::ContentConflict);
    }
    
    // Check for metadata conflicts
    if local.priority != remote.priority || 
       local.author != remote.author ||
       local.co_authors != remote.co_authors {
        return Some(ConflictType::MetadataConflict);
    }
    
    None // No conflicts detected
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::issue::{Issue, IssueStatus, Priority};
    use uuid::Uuid;

    #[test]
    fn test_sync_options() {
        let options = SyncOptions::new()
            .projects(vec!["project1".to_string()])
            .force()
            .dry_run()
            .auto_resolve()
            .batch_size(50)
            .timeout(std::time::Duration::from_secs(60));
        
        assert_eq!(options.projects, Some(vec!["project1".to_string()]));
        assert!(options.force);
        assert!(options.dry_run);
        assert!(options.auto_resolve);
        assert_eq!(options.batch_size, Some(50));
        assert_eq!(options.timeout, Some(std::time::Duration::from_secs(60)));
    }

    #[test]
    fn test_sync_stats() {
        let mut stats = SyncStats::new();
        
        assert!(!stats.is_completed());
        assert_eq!(stats.total_issues(), 0);
        
        stats.issues_pulled = 5;
        stats.issues_pushed = 3;
        stats.conflicts_detected = 2;
        stats.conflicts_resolved = 1;
        
        assert_eq!(stats.total_issues(), 8);
        assert_eq!(stats.resolution_rate(), 0.5);
        
        stats.complete();
        assert!(stats.is_completed());
        assert!(stats.duration.is_some());
    }

    #[test]
    fn test_conflict_detection() {
        let issue_id = Uuid::new_v4();
        let mut local_issue = Issue::new("Test Issue".to_string(), "alice".to_string());
        local_issue.id = issue_id;
        
        let mut remote_issue = local_issue.clone();
        
        // No conflicts
        assert_eq!(detect_conflict_type(&local_issue, &remote_issue), None);
        
        // Status conflict
        remote_issue.status = IssueStatus::Resolved;
        assert_eq!(
            detect_conflict_type(&local_issue, &remote_issue), 
            Some(ConflictType::StatusConflict)
        );
        
        // Reset and test content conflict
        remote_issue = local_issue.clone();
        remote_issue.title = "Different Title".to_string();
        assert_eq!(
            detect_conflict_type(&local_issue, &remote_issue), 
            Some(ConflictType::ContentConflict)
        );
        
        // Reset and test assignment conflict
        remote_issue = local_issue.clone();
        remote_issue.assignees = vec!["bob".to_string()];
        assert_eq!(
            detect_conflict_type(&local_issue, &remote_issue), 
            Some(ConflictType::AssignmentConflict)
        );
        
        // Reset and test metadata conflict
        remote_issue = local_issue.clone();
        remote_issue.priority = Priority::High;
        assert_eq!(
            detect_conflict_type(&local_issue, &remote_issue), 
            Some(ConflictType::MetadataConflict)
        );
    }

    #[test]
    fn test_batch_conflict_strategy() {
        // Test the enum variants exist and can be created
        let _stop = BatchConflictStrategy::StopOnConflict;
        let _skip = BatchConflictStrategy::SkipConflicts;
        let _auto = BatchConflictStrategy::AutoResolve(ConflictResolution::AcceptLocal);
        
        // Just ensure they compile and can be matched
        match BatchConflictStrategy::StopOnConflict {
            BatchConflictStrategy::StopOnConflict => assert!(true),
            _ => assert!(false),
        }
    }
}