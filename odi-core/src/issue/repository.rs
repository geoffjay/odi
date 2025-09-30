//! Issue repository trait and related types

use crate::{Result, issue::{Issue, IssueId, IssueStatus, Priority}, user::UserId, project::{ProjectId, LabelId}};

/// Issue query filters
#[derive(Debug, Default)]
pub struct IssueQuery {
    pub project_id: Option<ProjectId>,
    pub assignee: Option<UserId>,
    pub author: Option<UserId>,
    pub status: Option<IssueStatus>,
    pub priority: Option<Priority>,
    pub labels: Vec<LabelId>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Issue update operations
#[derive(Debug)]
pub struct IssueUpdate {
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub status: Option<IssueStatus>,
    pub priority: Option<Priority>,
    pub assignees: Option<Vec<UserId>>,
    pub co_authors: Option<Vec<UserId>>,
    pub labels: Option<Vec<LabelId>>,
    pub project_id: Option<Option<ProjectId>>,
}

/// Issue repository trait for data access operations
#[async_trait::async_trait]
pub trait IssueRepository: Send + Sync {
    /// Create a new issue
    async fn create(&self, issue: Issue) -> Result<Issue>;
    
    /// Get issue by ID
    async fn get(&self, id: &IssueId) -> Result<Option<Issue>>;
    
    /// Update an existing issue
    async fn update(&self, id: &IssueId, update: IssueUpdate) -> Result<Option<Issue>>;
    
    /// Delete an issue
    async fn delete(&self, id: &IssueId) -> Result<bool>;
    
    /// List issues with optional filtering
    async fn list(&self, query: IssueQuery) -> Result<Vec<Issue>>;
    
    /// Count issues matching query
    async fn count(&self, query: IssueQuery) -> Result<usize>;
    
    /// Get issues assigned to a user
    async fn get_assigned_to(&self, user_id: &UserId) -> Result<Vec<Issue>>;
    
    /// Get issues authored by a user
    async fn get_authored_by(&self, user_id: &UserId) -> Result<Vec<Issue>>;
    
    /// Get issues for a project
    async fn get_by_project(&self, project_id: &ProjectId) -> Result<Vec<Issue>>;
    
    /// Search issues by text
    async fn search(&self, query: &str) -> Result<Vec<Issue>>;
    
    /// Get recently updated issues
    async fn get_recent(&self, limit: usize) -> Result<Vec<Issue>>;
}

impl IssueQuery {
    /// Create a new query builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Filter by project
    pub fn project(mut self, project_id: ProjectId) -> Self {
        self.project_id = Some(project_id);
        self
    }
    
    /// Filter by assignee
    pub fn assignee(mut self, user_id: UserId) -> Self {
        self.assignee = Some(user_id);
        self
    }
    
    /// Filter by author
    pub fn author(mut self, user_id: UserId) -> Self {
        self.author = Some(user_id);
        self
    }
    
    /// Filter by status
    pub fn status(mut self, status: IssueStatus) -> Self {
        self.status = Some(status);
        self
    }
    
    /// Filter by priority
    pub fn priority(mut self, priority: Priority) -> Self {
        self.priority = Some(priority);
        self
    }
    
    /// Filter by labels
    pub fn labels(mut self, labels: Vec<LabelId>) -> Self {
        self.labels = labels;
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

impl IssueUpdate {
    /// Create a new update builder
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            status: None,
            priority: None,
            assignees: None,
            co_authors: None,
            labels: None,
            project_id: None,
        }
    }
    
    /// Update title
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }
    
    /// Update description
    pub fn description(mut self, description: Option<String>) -> Self {
        self.description = Some(description);
        self
    }
    
    /// Update status
    pub fn status(mut self, status: IssueStatus) -> Self {
        self.status = Some(status);
        self
    }
    
    /// Update priority
    pub fn priority(mut self, priority: Priority) -> Self {
        self.priority = Some(priority);
        self
    }
    
    /// Update assignees
    pub fn assignees(mut self, assignees: Vec<UserId>) -> Self {
        self.assignees = Some(assignees);
        self
    }
    
    /// Update co-authors
    pub fn co_authors(mut self, co_authors: Vec<UserId>) -> Self {
        self.co_authors = Some(co_authors);
        self
    }
    
    /// Update labels
    pub fn labels(mut self, labels: Vec<LabelId>) -> Self {
        self.labels = Some(labels);
        self
    }
    
    /// Update project
    pub fn project(mut self, project_id: Option<ProjectId>) -> Self {
        self.project_id = Some(project_id);
        self
    }
}