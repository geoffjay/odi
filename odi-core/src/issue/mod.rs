//! Issue entity and related operations

mod repository;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::user::UserId;
use crate::project::{ProjectId, LabelId};

pub use repository::{IssueRepository, IssueQuery, IssueUpdate};

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

    /// Validate issue title (1-100 characters)
    pub fn validate_title(title: &str) -> bool {
        !title.is_empty() && title.len() <= 100
    }

    /// Validate status transition
    pub fn can_transition_to(&self, new_status: &IssueStatus) -> bool {
        use IssueStatus::*;
        match (&self.status, new_status) {
            (Open, InProgress) | (Open, Closed) => true,
            (InProgress, Open) | (InProgress, Resolved) | (InProgress, Closed) => true,
            (Resolved, Open) | (Resolved, Closed) => true,
            (Closed, Open) => true,
            _ => false,
        }
    }

    /// Add assignee (with deduplication)
    pub fn add_assignee(&mut self, user_id: UserId) {
        if !self.assignees.contains(&user_id) {
            self.assignees.push(user_id);
        }
    }

    /// Remove assignee
    pub fn remove_assignee(&mut self, user_id: &UserId) {
        self.assignees.retain(|id| id != user_id);
    }

    /// Update status and timestamps
    pub fn update_status(&mut self, new_status: IssueStatus) -> Result<(), String> {
        if !self.can_transition_to(&new_status) {
            return Err(format!("Invalid status transition: {:?} -> {:?}", self.status, new_status));
        }
        
        self.status = new_status.clone();
        self.updated_at = Utc::now();
        
        if matches!(new_status, IssueStatus::Closed) {
            self.closed_at = Some(Utc::now());
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_issue_creation() {
        let author = "john".to_string();
        let issue = Issue::new("Fix login bug".to_string(), author.clone());
        
        assert_eq!(issue.title, "Fix login bug");
        assert_eq!(issue.author, author);
        assert_eq!(issue.status, IssueStatus::Open);
        assert_eq!(issue.priority, Priority::Medium);
        assert!(issue.assignees.is_empty());
    }

    #[test]
    fn test_issue_serialization() {
        let author = "alice".to_string();
        let mut issue = Issue::new("Test issue".to_string(), author);
        issue.description = Some("Test description".to_string());
        issue.priority = Priority::High;
        
        let json = serde_json::to_string(&issue).expect("Should serialize");
        assert!(json.contains("Test issue"));
        assert!(json.contains("High"));
        
        let deserialized: Issue = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.title, issue.title);
        assert_eq!(deserialized.priority, issue.priority);
    }

    #[test]
    fn test_status_transitions() {
        let author = "bob".to_string();
        let mut issue = Issue::new("Test issue".to_string(), author);
        
        // Open -> InProgress
        assert!(issue.update_status(IssueStatus::InProgress).is_ok());
        assert_eq!(issue.status, IssueStatus::InProgress);
        
        // InProgress -> Resolved
        assert!(issue.update_status(IssueStatus::Resolved).is_ok());
        assert_eq!(issue.status, IssueStatus::Resolved);
    }

    #[test]
    fn test_assignee_management() {
        let author = "manager".to_string();
        let mut issue = Issue::new("Team task".to_string(), author);
        
        issue.add_assignee("dev1".to_string());
        issue.add_assignee("dev2".to_string());
        issue.add_assignee("dev1".to_string()); // Duplicate should be ignored
        
        assert_eq!(issue.assignees.len(), 2);
        assert!(issue.assignees.contains(&"dev1".to_string()));
        
        issue.remove_assignee(&"dev1".to_string());
        assert_eq!(issue.assignees.len(), 1);
        assert!(!issue.assignees.contains(&"dev1".to_string()));
    }
}