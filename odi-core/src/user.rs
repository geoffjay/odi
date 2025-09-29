//! User and Team entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// User identifier type
pub type UserId = String;

/// Team identifier type  
pub type TeamId = String;

/// User entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>,
    pub teams: Vec<TeamId>,
    pub created_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
}

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

impl User {
    /// Create a new user
    pub fn new(id: UserId, name: String, email: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            email,
            avatar: None,
            teams: Vec::new(),
            created_at: now,
            last_active: now,
        }
    }

    /// Validate user ID (alphanumeric + underscore/dash)
    pub fn validate_id(id: &str) -> bool {
        !id.is_empty() && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    }

    /// Validate email format (basic validation)
    pub fn validate_email(email: &str) -> bool {
        email.contains('@') && email.contains('.') && email.len() > 5
    }
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

    /// Add member to team (with deduplication)
    pub fn add_member(&mut self, user_id: UserId) {
        if !self.members.contains(&user_id) {
            self.members.push(user_id);
        }
    }

    /// Remove member from team
    pub fn remove_member(&mut self, user_id: &UserId) {
        self.members.retain(|id| id != user_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(
            "john_doe".to_string(),
            "John Doe".to_string(),
            "john@example.com".to_string(),
        );
        
        assert_eq!(user.id, "john_doe");
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@example.com");
        assert!(user.teams.is_empty());
    }

    #[test]
    fn test_team_creation() {
        let team = Team::new("backend_team".to_string(), "Backend Team".to_string());
        
        assert_eq!(team.id, "backend_team");
        assert_eq!(team.name, "Backend Team");
        assert!(team.members.is_empty());
    }

    #[test]
    fn test_team_member_management() {
        let mut team = Team::new("dev_team".to_string(), "Development Team".to_string());
        
        team.add_member("alice".to_string());
        team.add_member("bob".to_string());
        team.add_member("alice".to_string()); // Duplicate should be ignored
        
        assert_eq!(team.members.len(), 2);
        assert!(team.members.contains(&"alice".to_string()));
        
        team.remove_member(&"alice".to_string());
        assert_eq!(team.members.len(), 1);
        assert!(!team.members.contains(&"alice".to_string()));
    }
}