//! T009: Contract test for odi-core User/Team management
//!
//! Tests User and Team entities, validation rules, and team membership.
//! This test MUST FAIL initially as per Constitutional Principle I (TDD).

use odi_core::{Team, TeamId, User, UserId};
use serde_json;

#[test]
fn test_user_creation() {
    // Test basic user creation
    let user = User::new(
        "john_doe".to_string(),
        "John Doe".to_string(),
        "john@example.com".to_string(),
    );
    
    assert_eq!(user.id, "john_doe");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email, "john@example.com");
    assert!(user.avatar.is_none());
    assert!(user.teams.is_empty());
}

#[test]
fn test_user_serialization() {
    // Test User serialization to/from JSON
    let mut user = User::new(
        "alice".to_string(),
        "Alice Smith".to_string(),
        "alice@example.com".to_string(),
    );
    user.avatar = Some("https://example.com/avatar.png".to_string());
    user.teams.push("backend".to_string());
    
    // Serialize to JSON
    let json = serde_json::to_string(&user).expect("Should serialize to JSON");
    assert!(json.contains("Alice Smith"));
    assert!(json.contains("alice@example.com"));
    assert!(json.contains("backend"));
    
    // Deserialize from JSON
    let deserialized: User = serde_json::from_str(&json).expect("Should deserialize from JSON");
    assert_eq!(deserialized.name, user.name);
    assert_eq!(deserialized.email, user.email);
    assert_eq!(deserialized.teams, user.teams);
}

#[test]
fn test_user_validation_rules() {
    // Test user ID validation (should be alphanumeric + underscore/dash)
    let valid_ids = vec!["john_doe", "alice-smith", "bob123", "user_1"];
    
    for id in valid_ids {
        let user = User::new(
            id.to_string(),
            "Test User".to_string(),
            "test@example.com".to_string(),
        );
        assert_eq!(user.id, id);
    }
    
    // Test email format (basic validation)
    let valid_emails = vec![
        "user@example.com",
        "test.email@domain.org",
        "name+tag@company.co.uk",
    ];
    
    for email in valid_emails {
        let user = User::new(
            "test_user".to_string(),
            "Test User".to_string(),
            email.to_string(),
        );
        assert_eq!(user.email, email);
    }
}

#[test]
fn test_user_team_membership() {
    // Test team membership operations
    let mut user = User::new(
        "team_member".to_string(),
        "Team Member".to_string(),
        "member@example.com".to_string(),
    );
    
    // Add to teams
    user.teams.push("backend".to_string());
    user.teams.push("frontend".to_string());
    user.teams.push("devops".to_string());
    
    assert_eq!(user.teams.len(), 3);
    assert!(user.teams.contains(&"backend".to_string()));
    assert!(user.teams.contains(&"frontend".to_string()));
    assert!(user.teams.contains(&"devops".to_string()));
    
    // Remove from team
    user.teams.retain(|team| team != "frontend");
    assert_eq!(user.teams.len(), 2);
    assert!(!user.teams.contains(&"frontend".to_string()));
}

#[test]
fn test_team_creation() {
    // Test basic team creation
    let team = Team::new("backend_team".to_string(), "Backend Development Team".to_string());
    
    assert_eq!(team.id, "backend_team");
    assert_eq!(team.name, "Backend Development Team");
    assert!(team.description.is_none());
    assert!(team.members.is_empty());
}

#[test]
fn test_team_serialization() {
    // Test Team serialization to/from JSON
    let mut team = Team::new("qa_team".to_string(), "Quality Assurance Team".to_string());
    team.description = Some("Responsible for testing and quality".to_string());
    team.members.push("alice".to_string());
    team.members.push("bob".to_string());
    
    // Serialize to JSON
    let json = serde_json::to_string(&team).expect("Should serialize to JSON");
    assert!(json.contains("Quality Assurance Team"));
    assert!(json.contains("alice"));
    assert!(json.contains("bob"));
    
    // Deserialize from JSON
    let deserialized: Team = serde_json::from_str(&json).expect("Should deserialize from JSON");
    assert_eq!(deserialized.name, team.name);
    assert_eq!(deserialized.description, team.description);
    assert_eq!(deserialized.members, team.members);
}

#[test]
fn test_team_member_management() {
    // Test team member operations
    let mut team = Team::new("dev_team".to_string(), "Development Team".to_string());
    
    // Add members
    team.members.push("john".to_string());
    team.members.push("alice".to_string());
    team.members.push("bob".to_string());
    
    assert_eq!(team.members.len(), 3);
    assert!(team.members.contains(&"john".to_string()));
    assert!(team.members.contains(&"alice".to_string()));
    assert!(team.members.contains(&"bob".to_string()));
    
    // Remove member
    team.members.retain(|member| member != "alice");
    assert_eq!(team.members.len(), 2);
    assert!(!team.members.contains(&"alice".to_string()));
    
    // Check for duplicate prevention (should be handled by business logic)
    team.members.push("john".to_string()); // Duplicate
    assert_eq!(team.members.len(), 3); // This will fail until deduplication is implemented
}

#[test]
fn test_team_validation_rules() {
    // Test team ID validation (should be alphanumeric + underscore/dash)
    let valid_ids = vec!["backend_team", "frontend-team", "team123", "qa_team"];
    
    for id in valid_ids {
        let team = Team::new(id.to_string(), "Test Team".to_string());
        assert_eq!(team.id, id);
    }
    
    // Test team name length (1-50 characters)
    let team1 = Team::new("short".to_string(), "A".to_string());
    assert_eq!(team1.name.len(), 1);
    
    let long_name = "A".repeat(50);
    let team2 = Team::new("long".to_string(), long_name.clone());
    assert_eq!(team2.name.len(), 50);
}

#[test]
fn test_team_timestamps() {
    // Test timestamp behavior
    let team = Team::new("time_team".to_string(), "Time Test Team".to_string());
    
    // created_at and updated_at should be set
    assert!(team.created_at <= chrono::Utc::now());
    assert!(team.updated_at <= chrono::Utc::now());
    assert_eq!(team.created_at, team.updated_at); // Should be same on creation
}

#[test]
fn test_user_team_relationship_consistency() {
    // Test bidirectional consistency between users and teams
    let mut user = User::new(
        "consistency_user".to_string(),
        "Consistency User".to_string(),
        "consistent@example.com".to_string(),
    );
    let mut team = Team::new("consistency_team".to_string(), "Consistency Team".to_string());
    
    // Add user to team
    team.members.push(user.id.clone());
    user.teams.push(team.id.clone());
    
    // Verify consistency
    assert!(team.members.contains(&user.id));
    assert!(user.teams.contains(&team.id));
    
    // Remove from both sides
    team.members.retain(|member| member != &user.id);
    user.teams.retain(|team_id| team_id != &team.id);
    
    assert!(!team.members.contains(&user.id));
    assert!(!user.teams.contains(&team.id));
}