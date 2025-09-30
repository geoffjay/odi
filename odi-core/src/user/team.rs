//! Team entity and operations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::user::{TeamId, UserId};

/// Team entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: TeamId,
    pub name: String,
    pub description: Option<String>,
    pub members: Vec<UserId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Team {
    /// Create a new team
    pub fn new(id: TeamId, name: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            description: None,
            members: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Validate team ID (3-50 characters, alphanumeric + underscore/dash)
    pub fn validate_id(id: &str) -> bool {
        id.len() >= 3 && id.len() <= 50 && 
        id.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    }

    /// Validate team name (1-100 characters)
    pub fn validate_name(name: &str) -> bool {
        !name.is_empty() && name.len() <= 100
    }

    /// Add member to team (with deduplication)
    pub fn add_member(&mut self, user_id: UserId) {
        if !self.members.contains(&user_id) {
            self.members.push(user_id);
            self.updated_at = Utc::now();
        }
    }

    /// Remove member from team
    pub fn remove_member(&mut self, user_id: &UserId) {
        let old_len = self.members.len();
        self.members.retain(|id| id != user_id);
        if self.members.len() != old_len {
            self.updated_at = Utc::now();
        }
    }

    /// Check if user is a member
    pub fn is_member(&self, user_id: &UserId) -> bool {
        self.members.contains(user_id)
    }

    /// Get member count
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Update team description
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_team_creation() {
        let team = Team::new("backend_team".to_string(), "Backend Team".to_string());
        
        assert_eq!(team.id, "backend_team");
        assert_eq!(team.name, "Backend Team");
        assert!(team.members.is_empty());
        assert_eq!(team.member_count(), 0);
    }

    #[test]
    fn test_team_validation() {
        assert!(Team::validate_id("backend_team"));
        assert!(Team::validate_id("team-123"));
        assert!(!Team::validate_id("ab")); // Too short
        assert!(!Team::validate_id("")); // Empty
        
        let long_id = "a".repeat(51);
        assert!(!Team::validate_id(&long_id)); // Too long

        assert!(Team::validate_name("Backend Team"));
        assert!(!Team::validate_name(""));
        
        let long_name = "a".repeat(101);
        assert!(!Team::validate_name(&long_name));
    }

    #[test]
    fn test_team_member_management() {
        let mut team = Team::new("dev_team".to_string(), "Development Team".to_string());
        
        team.add_member("alice".to_string());
        team.add_member("bob".to_string());
        team.add_member("alice".to_string()); // Duplicate should be ignored
        
        assert_eq!(team.member_count(), 2);
        assert!(team.is_member(&"alice".to_string()));
        assert!(team.is_member(&"bob".to_string()));
        assert!(!team.is_member(&"charlie".to_string()));
        
        team.remove_member(&"alice".to_string());
        assert_eq!(team.member_count(), 1);
        assert!(!team.is_member(&"alice".to_string()));
        assert!(team.is_member(&"bob".to_string()));
    }

    #[test]
    fn test_team_description() {
        let mut team = Team::new("test_team".to_string(), "Test Team".to_string());
        
        assert!(team.description.is_none());
        
        team.set_description(Some("A team for testing".to_string()));
        assert_eq!(team.description, Some("A team for testing".to_string()));
        
        team.set_description(None);
        assert!(team.description.is_none());
    }
}