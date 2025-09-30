//! User and Team repository traits

use crate::{Result, user::{User, UserId, Team, TeamId}};

/// User query filters
#[derive(Debug, Default)]
pub struct UserQuery {
    pub team_id: Option<TeamId>,
    pub email_domain: Option<String>,
    pub active_since: Option<chrono::DateTime<chrono::Utc>>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// User update operations
#[derive(Debug)]
pub struct UserUpdate {
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<Option<String>>,
    pub teams: Option<Vec<TeamId>>,
}

/// Team query filters
#[derive(Debug, Default)]
pub struct TeamQuery {
    pub member_id: Option<UserId>,
    pub min_members: Option<usize>,
    pub max_members: Option<usize>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Team update operations
#[derive(Debug)]
pub struct TeamUpdate {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub members: Option<Vec<UserId>>,
}

/// User repository trait for data access operations
#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    /// Create a new user
    async fn create_user(&self, user: User) -> Result<User>;
    
    /// Get user by ID
    async fn get_user(&self, id: &UserId) -> Result<Option<User>>;
    
    /// Get user by email
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>>;
    
    /// Update an existing user
    async fn update_user(&self, id: &UserId, update: UserUpdate) -> Result<Option<User>>;
    
    /// Delete a user
    async fn delete_user(&self, id: &UserId) -> Result<bool>;
    
    /// List users with optional filtering
    async fn list_users(&self, query: UserQuery) -> Result<Vec<User>>;
    
    /// Count users matching query
    async fn count_users(&self, query: UserQuery) -> Result<usize>;
    
    /// Search users by name or email
    async fn search_users(&self, query: &str) -> Result<Vec<User>>;
    
    /// Create a new team
    async fn create_team(&self, team: Team) -> Result<Team>;
    
    /// Get team by ID
    async fn get_team(&self, id: &TeamId) -> Result<Option<Team>>;
    
    /// Update an existing team
    async fn update_team(&self, id: &TeamId, update: TeamUpdate) -> Result<Option<Team>>;
    
    /// Delete a team
    async fn delete_team(&self, id: &TeamId) -> Result<bool>;
    
    /// List teams with optional filtering
    async fn list_teams(&self, query: TeamQuery) -> Result<Vec<Team>>;
    
    /// Count teams matching query
    async fn count_teams(&self, query: TeamQuery) -> Result<usize>;
    
    /// Get teams for a user
    async fn get_user_teams(&self, user_id: &UserId) -> Result<Vec<Team>>;
    
    /// Get members of a team
    async fn get_team_members(&self, team_id: &TeamId) -> Result<Vec<User>>;
    
    /// Add user to team
    async fn add_team_member(&self, team_id: &TeamId, user_id: &UserId) -> Result<()>;
    
    /// Remove user from team
    async fn remove_team_member(&self, team_id: &TeamId, user_id: &UserId) -> Result<()>;
}

impl UserQuery {
    /// Create a new query builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Filter by team membership
    pub fn team(mut self, team_id: TeamId) -> Self {
        self.team_id = Some(team_id);
        self
    }
    
    /// Filter by email domain
    pub fn email_domain(mut self, domain: String) -> Self {
        self.email_domain = Some(domain);
        self
    }
    
    /// Filter by active since date
    pub fn active_since(mut self, date: chrono::DateTime<chrono::Utc>) -> Self {
        self.active_since = Some(date);
        self
    }
    
    /// Set limit
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
    
    /// Set offset
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
}

impl UserUpdate {
    /// Create a new update builder
    pub fn new() -> Self {
        Self {
            name: None,
            email: None,
            avatar: None,
            teams: None,
        }
    }
    
    /// Update name
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    
    /// Update email
    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
    
    /// Update avatar
    pub fn avatar(mut self, avatar: Option<String>) -> Self {
        self.avatar = Some(avatar);
        self
    }
    
    /// Update teams
    pub fn teams(mut self, teams: Vec<TeamId>) -> Self {
        self.teams = Some(teams);
        self
    }
}

impl TeamQuery {
    /// Create a new query builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Filter by member
    pub fn member(mut self, user_id: UserId) -> Self {
        self.member_id = Some(user_id);
        self
    }
    
    /// Filter by minimum members
    pub fn min_members(mut self, min: usize) -> Self {
        self.min_members = Some(min);
        self
    }
    
    /// Filter by maximum members
    pub fn max_members(mut self, max: usize) -> Self {
        self.max_members = Some(max);
        self
    }
    
    /// Set limit
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
    
    /// Set offset
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
}

impl TeamUpdate {
    /// Create a new update builder
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            members: None,
        }
    }
    
    /// Update name
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    
    /// Update description
    pub fn description(mut self, description: Option<String>) -> Self {
        self.description = Some(description);
        self
    }
    
    /// Update members
    pub fn members(mut self, members: Vec<UserId>) -> Self {
        self.members = Some(members);
        self
    }
}