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
}