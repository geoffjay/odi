//! T008: Contract test for odi-core Issue entity serialization
//!
//! Tests the core Issue entity serialization, validation, and state transitions.
//! This test MUST FAIL initially as per Constitutional Principle I (TDD).

use odi_core::{Issue, IssueId, IssueStatus, Priority, UserId};
use serde_json;

#[test]
fn test_issue_creation() {
    // Test basic issue creation
    let author = UserId::from("john");
    let issue = Issue::new("Fix login bug".to_string(), author.clone());
    
    assert_eq!(issue.title, "Fix login bug");
    assert_eq!(issue.author, author);
    assert_eq!(issue.status, IssueStatus::Open);
    assert_eq!(issue.priority, Priority::Medium);
    assert!(issue.assignees.is_empty());
    assert!(issue.co_authors.is_empty());
    assert!(issue.labels.is_empty());
}

#[test]
fn test_issue_serialization() {
    // Test Issue serialization to/from JSON
    let author = UserId::from("alice");
    let mut issue = Issue::new("Implement feature X".to_string(), author);
    issue.description = Some("Detailed description here".to_string());
    issue.priority = Priority::High;
    
    // Serialize to JSON
    let json = serde_json::to_string(&issue).expect("Should serialize to JSON");
    assert!(json.contains("Implement feature X"));
    assert!(json.contains("High"));
    
    // Deserialize from JSON
    let deserialized: Issue = serde_json::from_str(&json).expect("Should deserialize from JSON");
    assert_eq!(deserialized.title, issue.title);
    assert_eq!(deserialized.priority, issue.priority);
    assert_eq!(deserialized.description, issue.description);
}

#[test]
fn test_issue_status_transitions() {
    // Test valid status transitions
    let author = UserId::from("bob");
    let mut issue = Issue::new("Test issue".to_string(), author);
    
    // Open -> InProgress (valid)
    issue.status = IssueStatus::InProgress;
    assert_eq!(issue.status, IssueStatus::InProgress);
    
    // InProgress -> Resolved (valid)
    issue.status = IssueStatus::Resolved;
    assert_eq!(issue.status, IssueStatus::Resolved);
    
    // Resolved -> Closed (valid)
    issue.status = IssueStatus::Closed;
    assert_eq!(issue.status, IssueStatus::Closed);
}

#[test]
fn test_issue_validation_rules() {
    // Test title validation
    let author = UserId::from("validator");
    
    // Valid title (1-100 characters)
    let issue1 = Issue::new("Valid title".to_string(), author.clone());
    assert_eq!(issue1.title.len(), 11);
    
    // Edge case: 100 character title
    let long_title = "a".repeat(100);
    let issue2 = Issue::new(long_title.clone(), author.clone());
    assert_eq!(issue2.title.len(), 100);
    
    // This should be validated by the domain logic (not implemented yet)
    // Empty title should be rejected
    // Title > 100 chars should be rejected
}

#[test]
fn test_issue_assignee_management() {
    // Test assignee operations
    let author = UserId::from("manager");
    let mut issue = Issue::new("Team task".to_string(), author);
    
    // Add assignees
    issue.assignees.push(UserId::from("dev1"));
    issue.assignees.push(UserId::from("dev2"));
    
    assert_eq!(issue.assignees.len(), 2);
    assert!(issue.assignees.contains(&UserId::from("dev1")));
    assert!(issue.assignees.contains(&UserId::from("dev2")));
    
    // Remove assignee
    issue.assignees.retain(|id| id != &UserId::from("dev1"));
    assert_eq!(issue.assignees.len(), 1);
    assert!(!issue.assignees.contains(&UserId::from("dev1")));
}

#[test]
fn test_issue_priority_levels() {
    // Test all priority levels
    let author = UserId::from("tester");
    
    let priorities = vec![
        Priority::Low,
        Priority::Medium, 
        Priority::High,
        Priority::Critical,
    ];
    
    for priority in priorities {
        let mut issue = Issue::new("Priority test".to_string(), author.clone());
        issue.priority = priority.clone();
        assert_eq!(issue.priority, priority);
    }
}

#[test]
fn test_issue_timestamps() {
    // Test timestamp behavior
    let author = UserId::from("timekeeper");
    let issue = Issue::new("Time test".to_string(), author);
    
    // created_at and updated_at should be set
    assert!(issue.created_at <= chrono::Utc::now());
    assert!(issue.updated_at <= chrono::Utc::now());
    assert_eq!(issue.created_at, issue.updated_at); // Should be same on creation
    
    // closed_at should be None for new issue
    assert!(issue.closed_at.is_none());
}

#[test]
fn test_issue_unique_ids() {
    // Test that each issue gets a unique ID
    let author = UserId::from("id_tester");
    let issue1 = Issue::new("Issue 1".to_string(), author.clone());
    let issue2 = Issue::new("Issue 2".to_string(), author);
    
    assert_ne!(issue1.id, issue2.id);
    
    // IDs should be valid UUIDs
    assert!(issue1.id.to_string().len() == 36); // UUID string length
    assert!(issue2.id.to_string().len() == 36);
}