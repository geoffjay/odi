//! Project, Workspace, and Label repository traits

use crate::{
    Result,
    project::{Project, ProjectId, Workspace, WorkspaceId, Label, LabelId},
    user::TeamId,
    issue::IssueId,
};
use std::path::PathBuf;

/// Project query filters
#[derive(Debug, Default)]
pub struct ProjectQuery {
    pub workspace_id: Option<WorkspaceId>,
    pub team_id: Option<TeamId>,
    pub has_issues: Option<bool>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Project update operations
#[derive(Debug)]
pub struct ProjectUpdate {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub teams: Option<Vec<TeamId>>,
    pub workspaces: Option<Vec<WorkspaceId>>,
}

/// Workspace query filters
#[derive(Debug, Default)]
pub struct WorkspaceQuery {
    pub project_id: Option<ProjectId>,
    pub path_prefix: Option<PathBuf>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Workspace update operations
#[derive(Debug)]
pub struct WorkspaceUpdate {
    pub path: Option<PathBuf>,
    pub projects: Option<Vec<ProjectId>>,
}

/// Label query filters
#[derive(Debug, Default)]
pub struct LabelQuery {
    pub project_id: Option<ProjectId>,
    pub color: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Project repository trait for data access operations
#[async_trait::async_trait]
pub trait ProjectRepository: Send + Sync {
    /// Create a new project
    async fn create_project(&self, project: Project) -> Result<Project>;
    
    /// Get project by ID
    async fn get_project(&self, id: &ProjectId) -> Result<Option<Project>>;
    
    /// Update an existing project
    async fn update_project(&self, id: &ProjectId, update: ProjectUpdate) -> Result<Option<Project>>;
    
    /// Delete a project
    async fn delete_project(&self, id: &ProjectId) -> Result<bool>;
    
    /// List projects with optional filtering
    async fn list_projects(&self, query: ProjectQuery) -> Result<Vec<Project>>;
    
    /// Count projects matching query
    async fn count_projects(&self, query: ProjectQuery) -> Result<usize>;
    
    /// Search projects by name or description
    async fn search_projects(&self, query: &str) -> Result<Vec<Project>>;
    
    /// Create a new workspace
    async fn create_workspace(&self, workspace: Workspace) -> Result<Workspace>;
    
    /// Get workspace by ID
    async fn get_workspace(&self, id: &WorkspaceId) -> Result<Option<Workspace>>;
    
    /// Get workspace by path
    async fn get_workspace_by_path(&self, path: &PathBuf) -> Result<Option<Workspace>>;
    
    /// Update an existing workspace
    async fn update_workspace(&self, id: &WorkspaceId, update: WorkspaceUpdate) -> Result<Option<Workspace>>;
    
    /// Delete a workspace
    async fn delete_workspace(&self, id: &WorkspaceId) -> Result<bool>;
    
    /// List workspaces with optional filtering
    async fn list_workspaces(&self, query: WorkspaceQuery) -> Result<Vec<Workspace>>;
    
    /// Count workspaces matching query
    async fn count_workspaces(&self, query: WorkspaceQuery) -> Result<usize>;
    
    /// Get workspaces for a project
    async fn get_project_workspaces(&self, project_id: &ProjectId) -> Result<Vec<Workspace>>;
    
    /// Get projects in a workspace
    async fn get_workspace_projects(&self, workspace_id: &WorkspaceId) -> Result<Vec<Project>>;
    
    /// Link project to workspace (many-to-many)
    async fn link_project_workspace(&self, project_id: &ProjectId, workspace_id: &WorkspaceId) -> Result<()>;
    
    /// Unlink project from workspace
    async fn unlink_project_workspace(&self, project_id: &ProjectId, workspace_id: &WorkspaceId) -> Result<()>;
    
    /// Create a new label in project
    async fn create_label(&self, project_id: &ProjectId, label: Label) -> Result<Label>;
    
    /// Get label by ID in project
    async fn get_label(&self, project_id: &ProjectId, label_id: &LabelId) -> Result<Option<Label>>;
    
    /// Update an existing label in project
    async fn update_label(&self, project_id: &ProjectId, label_id: &LabelId, label: Label) -> Result<Option<Label>>;
    
    /// Delete a label from project
    async fn delete_label(&self, project_id: &ProjectId, label_id: &LabelId) -> Result<bool>;
    
    /// List labels in project with optional filtering
    async fn list_labels(&self, project_id: &ProjectId, query: LabelQuery) -> Result<Vec<Label>>;
    
    /// Get all labels across projects
    async fn get_all_labels(&self, query: LabelQuery) -> Result<Vec<Label>>;
    
    /// Add issue to project
    async fn add_project_issue(&self, project_id: &ProjectId, issue_id: &IssueId) -> Result<()>;
    
    /// Remove issue from project
    async fn remove_project_issue(&self, project_id: &ProjectId, issue_id: &IssueId) -> Result<()>;
    
    /// Add team to project
    async fn add_project_team(&self, project_id: &ProjectId, team_id: &TeamId) -> Result<()>;
    
    /// Remove team from project
    async fn remove_project_team(&self, project_id: &ProjectId, team_id: &TeamId) -> Result<()>;
}

impl ProjectQuery {
    /// Create a new query builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Filter by workspace
    pub fn workspace(mut self, workspace_id: WorkspaceId) -> Self {
        self.workspace_id = Some(workspace_id);
        self
    }
    
    /// Filter by team
    pub fn team(mut self, team_id: TeamId) -> Self {
        self.team_id = Some(team_id);
        self
    }
    
    /// Filter by whether project has issues
    pub fn has_issues(mut self, has_issues: bool) -> Self {
        self.has_issues = Some(has_issues);
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

impl ProjectUpdate {
    /// Create a new update builder
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            teams: None,
            workspaces: None,
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
    
    /// Update teams
    pub fn teams(mut self, teams: Vec<TeamId>) -> Self {
        self.teams = Some(teams);
        self
    }
    
    /// Update workspaces
    pub fn workspaces(mut self, workspaces: Vec<WorkspaceId>) -> Self {
        self.workspaces = Some(workspaces);
        self
    }
}

impl WorkspaceQuery {
    /// Create a new query builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Filter by project
    pub fn project(mut self, project_id: ProjectId) -> Self {
        self.project_id = Some(project_id);
        self
    }
    
    /// Filter by path prefix
    pub fn path_prefix(mut self, path: PathBuf) -> Self {
        self.path_prefix = Some(path);
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

impl WorkspaceUpdate {
    /// Create a new update builder
    pub fn new() -> Self {
        Self {
            path: None,
            projects: None,
        }
    }
    
    /// Update path
    pub fn path(mut self, path: PathBuf) -> Self {
        self.path = Some(path);
        self
    }
    
    /// Update projects
    pub fn projects(mut self, projects: Vec<ProjectId>) -> Self {
        self.projects = Some(projects);
        self
    }
}

impl LabelQuery {
    /// Create a new query builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Filter by project
    pub fn project(mut self, project_id: ProjectId) -> Self {
        self.project_id = Some(project_id);
        self
    }
    
    /// Filter by color
    pub fn color(mut self, color: String) -> Self {
        self.color = Some(color);
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