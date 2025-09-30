//! User and Team entities

mod repository;
mod team;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub use repository::{UserRepository, UserQuery, UserUpdate, TeamQuery, TeamUpdate};
pub use team::Team;

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
        email.contains('@') && email.contains('.') && email.len() > 5 && !email.starts_with('@')
    }
    
    /// Add user to team
    pub fn add_to_team(&mut self, team_id: TeamId) {
        if !self.teams.contains(&team_id) {
            self.teams.push(team_id);
        }
    }
    
    /// Remove user from team
    pub fn remove_from_team(&mut self, team_id: &TeamId) {
        self.teams.retain(|id| id != team_id);
    }
    
    /// Update last active timestamp
    pub fn update_last_active(&mut self) {
        self.last_active = Utc::now();
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
    fn test_user_validation() {
        assert!(User::validate_id("john_doe"));
        assert!(User::validate_id("user-123"));
        assert!(!User::validate_id("user@domain"));
        assert!(!User::validate_id(""));

        assert!(User::validate_email("user@example.com"));
        assert!(!User::validate_email("invalid"));
        assert!(!User::validate_email("@example.com"));
    }

    #[test]
    fn test_team_membership() {
        let mut user = User::new(
            "alice".to_string(),
            "Alice Smith".to_string(),
            "alice@example.com".to_string(),
        );
        
        user.add_to_team("backend_team".to_string());
        user.add_to_team("frontend_team".to_string());
        user.add_to_team("backend_team".to_string()); // Duplicate should be ignored
        
        assert_eq!(user.teams.len(), 2);
        assert!(user.teams.contains(&"backend_team".to_string()));
        
        user.remove_from_team(&"backend_team".to_string());
        assert_eq!(user.teams.len(), 1);
        assert!(!user.teams.contains(&"backend_team".to_string()));
    }
}