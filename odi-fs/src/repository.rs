//! Repository implementations using filesystem storage
//!
//! This module provides concrete implementations of the repository traits
//! defined in odi-core, backed by the filesystem storage engine.

use crate::{storage::{FileSystemStorage, ObjectStorage, ObjectType, ObjectRef}};
use odi_core::*;
use serde_json;

/// Issue repository implementation using filesystem storage
pub struct FsIssueRepository {
    storage: FileSystemStorage,
}

impl FsIssueRepository {
    pub fn new(storage: FileSystemStorage) -> Self {
        Self { storage }
    }
}

#[async_trait::async_trait]
impl IssueRepository for FsIssueRepository {
    async fn create(&self, issue: Issue) -> odi_core::Result<Issue> {
        let serialized = serde_json::to_vec(&issue)
            .map_err(CoreError::Serialization)?;
        
        let hash = self.storage.store_object(ObjectType::Issue, &serialized)
            .map_err(|e| CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
            
        // Create reference for easy lookup
        self.storage.create_ref(&format!("issues/{}", issue.id.to_string()), &hash, ObjectType::Issue)
            .map_err(|e| CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
        
        Ok(issue)
    }
    
    async fn get(&self, id: &IssueId) -> odi_core::Result<Option<Issue>> {
        let ref_name = format!("issues/{}", id.to_string());
        
        let ref_obj = self.storage.get_ref(&ref_name)
            .map_err(|e| CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
            
        if let Some(obj_ref) = ref_obj {
            let storage_obj = self.storage.retrieve_object(&obj_ref.hash)
                .map_err(|e| CoreError::ValidationError { 
                    field: "storage".to_string(), 
                    message: e.to_string() 
                })?;
                
            if let Some(obj) = storage_obj {
                let issue: Issue = serde_json::from_slice(&obj.data)
                    .map_err(CoreError::Serialization)?;
                Ok(Some(issue))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    
    async fn update(&self, id: &IssueId, update: IssueUpdate) -> odi_core::Result<Option<Issue>> {
        if let Some(mut issue) = self.get(id).await? {
            // Apply updates
            if let Some(title) = update.title {
                issue.title = title;
            }
            if let Some(description) = update.description {
                issue.description = description;
            }
            if let Some(status) = update.status {
                issue.status = status;
            }
            if let Some(priority) = update.priority {
                issue.priority = priority;
            }
            if let Some(assignees) = update.assignees {
                issue.assignees = assignees;
            }
            if let Some(co_authors) = update.co_authors {
                issue.co_authors = co_authors;
            }
            if let Some(labels) = update.labels {
                issue.labels = labels;
            }
            if let Some(project_id) = update.project_id {
                issue.project_id = project_id;
            }
            
            // Update timestamps
            issue.updated_at = chrono::Utc::now();
            
            // Store updated issue
            self.create(issue.clone()).await?;
            Ok(Some(issue))
        } else {
            Ok(None)
        }
    }
    
    async fn delete(&self, id: &IssueId) -> odi_core::Result<bool> {
        let ref_name = format!("issues/{}", id.to_string());
        
        let ref_obj = self.storage.get_ref(&ref_name)
            .map_err(|e| CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
            
        if let Some(obj_ref) = ref_obj {
            // Delete the object
            let deleted_obj = self.storage.delete_object(&obj_ref.hash)
                .map_err(|e| CoreError::ValidationError { 
                    field: "storage".to_string(), 
                    message: e.to_string() 
                })?;
                
            // Delete the reference
            let deleted_ref = self.storage.delete_ref(&ref_name)
                .map_err(|e| CoreError::ValidationError { 
                    field: "storage".to_string(), 
                    message: e.to_string() 
                })?;
                
            Ok(deleted_obj && deleted_ref)
        } else {
            Ok(false)
        }
    }
    
    async fn list(&self, query: IssueQuery) -> odi_core::Result<Vec<Issue>> {
        let mut issues = Vec::new();
        
        // Get all issue objects
        let hashes = self.storage.list_objects(Some(ObjectType::Issue))
            .map_err(|e| CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
            
        for hash in hashes {
            if let Some(storage_obj) = self.storage.retrieve_object(&hash)
                .map_err(|e| CoreError::ValidationError { 
                    field: "storage".to_string(), 
                    message: e.to_string() 
                })? {
                
                let issue: Issue = serde_json::from_slice(&storage_obj.data)
                    .map_err(CoreError::Serialization)?;
                    
                // Apply filters
                let mut include = true;
                
                if let Some(project_id) = &query.project_id {
                    if issue.project_id.as_ref() != Some(project_id) {
                        include = false;
                    }
                }
                
                if let Some(assignee) = &query.assignee {
                    if !issue.assignees.contains(assignee) {
                        include = false;
                    }
                }
                
                if let Some(author) = &query.author {
                    if &issue.author != author {
                        include = false;
                    }
                }
                
                if let Some(status) = &query.status {
                    if &issue.status != status {
                        include = false;
                    }
                }
                
                if let Some(priority) = &query.priority {
                    if &issue.priority != priority {
                        include = false;
                    }
                }
                
                if !query.labels.is_empty() {
                    let has_any_label = query.labels.iter().any(|l| issue.labels.contains(l));
                    if !has_any_label {
                        include = false;
                    }
                }
                
                if include {
                    issues.push(issue);
                }
            }
        }
        
        // Apply limit and offset
        if let Some(offset) = query.offset {
            if offset >= issues.len() {
                issues.clear();
            } else {
                issues.drain(0..offset);
            }
        }
        
        if let Some(limit) = query.limit {
            issues.truncate(limit);
        }
        
        Ok(issues)
    }
    
    async fn count(&self, query: IssueQuery) -> odi_core::Result<usize> {
        let issues = self.list(query).await?;
        Ok(issues.len())
    }
    
    async fn get_assigned_to(&self, user_id: &UserId) -> odi_core::Result<Vec<Issue>> {
        let query = IssueQuery {
            assignee: Some(user_id.clone()),
            ..Default::default()
        };
        self.list(query).await
    }
    
    async fn get_authored_by(&self, user_id: &UserId) -> odi_core::Result<Vec<Issue>> {
        let query = IssueQuery {
            author: Some(user_id.clone()),
            ..Default::default()
        };
        self.list(query).await
    }
    
    async fn get_by_project(&self, project_id: &ProjectId) -> odi_core::Result<Vec<Issue>> {
        let query = IssueQuery {
            project_id: Some(project_id.clone()),
            ..Default::default()
        };
        self.list(query).await
    }
    
    async fn search(&self, query: &str) -> odi_core::Result<Vec<Issue>> {
        let mut issues = Vec::new();
        let all_issues = self.list(IssueQuery::default()).await?;
        
        for issue in all_issues {
            if issue.title.to_lowercase().contains(&query.to_lowercase()) 
                || issue.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&query.to_lowercase())) {
                issues.push(issue);
            }
        }
        
        Ok(issues)
    }
    
    async fn get_recent(&self, limit: usize) -> odi_core::Result<Vec<Issue>> {
        let mut issues = self.list(IssueQuery::default()).await?;
        
        // Sort by updated_at descending
        issues.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        issues.truncate(limit);
        
        Ok(issues)
    }
}

/// Project repository implementation using filesystem storage
pub struct FsProjectRepository {
    storage: FileSystemStorage,
}

impl FsProjectRepository {
    pub fn new(storage: FileSystemStorage) -> Self {
        Self { storage }
    }
}

#[async_trait::async_trait]
impl ProjectRepository for FsProjectRepository {
    async fn create_project(&self, project: Project) -> odi_core::Result<Project> {
        let serialized = serde_json::to_vec(&project)
            .map_err(CoreError::Serialization)?;
        
        let hash = self.storage.store_object(ObjectType::Project, &serialized)
            .map_err(|e| CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
            
        // Create reference for easy lookup
        self.storage.create_ref(&format!("projects/{}", project.id.to_string()), &hash, ObjectType::Project)
            .map_err(|e| CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
        
        Ok(project)
    }
    
    async fn get_project(&self, id: &ProjectId) -> odi_core::Result<Option<Project>> {
        let ref_name = format!("projects/{}", id.to_string());
        
        let ref_obj = self.storage.get_ref(&ref_name)
            .map_err(|e| CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
            
        if let Some(obj_ref) = ref_obj {
            let storage_obj = self.storage.retrieve_object(&obj_ref.hash)
                .map_err(|e| CoreError::ValidationError { 
                    field: "storage".to_string(), 
                    message: e.to_string() 
                })?;
                
            if let Some(obj) = storage_obj {
                let project: Project = serde_json::from_slice(&obj.data)
                    .map_err(CoreError::Serialization)?;
                Ok(Some(project))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    
    async fn update_project(&self, id: &ProjectId, update: ProjectUpdate) -> odi_core::Result<Option<Project>> {
        if let Some(mut project) = self.get_project(id).await? {
            // Apply updates
            if let Some(name) = update.name {
                project.name = name;
            }
            if let Some(description) = update.description {
                project.description = description;
            }
            if let Some(teams) = update.teams {
                project.teams = teams;
            }
            if let Some(workspaces) = update.workspaces {
                project.workspaces = workspaces;
            }
            
            // Update timestamps
            project.updated_at = chrono::Utc::now();
            
            // Store updated project
            self.create_project(project.clone()).await?;
            Ok(Some(project))
        } else {
            Ok(None)
        }
    }
    
    async fn delete_project(&self, id: &ProjectId) -> odi_core::Result<bool> {
        let ref_name = format!("projects/{}", id.to_string());
        
        let ref_obj = self.storage.get_ref(&ref_name)
            .map_err(|e| CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
            
        if let Some(obj_ref) = ref_obj {
            // Delete the object
            let deleted_obj = self.storage.delete_object(&obj_ref.hash)
                .map_err(|e| CoreError::ValidationError { 
                    field: "storage".to_string(), 
                    message: e.to_string() 
                })?;
                
            // Delete the reference
            let deleted_ref = self.storage.delete_ref(&ref_name)
                .map_err(|e| CoreError::ValidationError { 
                    field: "storage".to_string(), 
                    message: e.to_string() 
                })?;
                
            Ok(deleted_obj && deleted_ref)
        } else {
            Ok(false)
        }
    }
    
    async fn list_projects(&self, query: ProjectQuery) -> odi_core::Result<Vec<Project>> {
        let mut projects = Vec::new();
        
        // Get all project objects
        let hashes = self.storage.list_objects(Some(ObjectType::Project))
            .map_err(|e| CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
            
        for hash in hashes {
            if let Some(storage_obj) = self.storage.retrieve_object(&hash)
                .map_err(|e| CoreError::ValidationError { 
                    field: "storage".to_string(), 
                    message: e.to_string() 
                })? {
                
                let project: Project = serde_json::from_slice(&storage_obj.data)
                    .map_err(CoreError::Serialization)?;
                    
                // Apply filters
                let mut include = true;
                
                if let Some(workspace_id) = &query.workspace_id {
                    if !project.workspaces.contains(workspace_id) {
                        include = false;
                    }
                }
                
                if let Some(team_id) = &query.team_id {
                    if !project.teams.contains(team_id) {
                        include = false;
                    }
                }
                
                if include {
                    projects.push(project);
                }
            }
        }
        
        // Apply limit and offset
        if let Some(offset) = query.offset {
            if offset >= projects.len() {
                projects.clear();
            } else {
                projects.drain(0..offset);
            }
        }
        
        if let Some(limit) = query.limit {
            projects.truncate(limit);
        }
        
        Ok(projects)
    }
    
    async fn count_projects(&self, query: ProjectQuery) -> odi_core::Result<usize> {
        let projects = self.list_projects(query).await?;
        Ok(projects.len())
    }
    
    async fn search_projects(&self, query: &str) -> odi_core::Result<Vec<Project>> {
        let mut projects = Vec::new();
        let all_projects = self.list_projects(ProjectQuery::default()).await?;
        
        for project in all_projects {
            if project.name.to_lowercase().contains(&query.to_lowercase()) 
                || project.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&query.to_lowercase())) {
                projects.push(project);
            }
        }
        
        Ok(projects)
    }
    
    async fn create_workspace(&self, workspace: Workspace) -> odi_core::Result<Workspace> {
        // For simplicity, we'll store workspaces as a special type of project data
        // In a real implementation, this might be separate storage
        let serialized = serde_json::to_vec(&workspace)
            .map_err(CoreError::Serialization)?;
        
        let hash = self.storage.store_object(ObjectType::Project, &serialized)
            .map_err(|e| CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
            
        // Create reference for easy lookup
        self.storage.create_ref(&format!("workspaces/{}", workspace.id.to_string()), &hash, ObjectType::Project)
            .map_err(|e| CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
        
        Ok(workspace)
    }
    
    async fn get_workspace(&self, id: &WorkspaceId) -> odi_core::Result<Option<Workspace>> {
        let ref_name = format!("workspaces/{}", id.to_string());
        
        let ref_obj = self.storage.get_ref(&ref_name)
            .map_err(|e| CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
            
        if let Some(obj_ref) = ref_obj {
            let storage_obj = self.storage.retrieve_object(&obj_ref.hash)
                .map_err(|e| CoreError::ValidationError { 
                    field: "storage".to_string(), 
                    message: e.to_string() 
                })?;
                
            if let Some(obj) = storage_obj {
                let workspace: Workspace = serde_json::from_slice(&obj.data)
                    .map_err(CoreError::Serialization)?;
                Ok(Some(workspace))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    
    async fn get_workspace_by_path(&self, path: &std::path::PathBuf) -> odi_core::Result<Option<Workspace>> {
        // Search through all workspaces to find one with matching path
        let workspaces = self.list_workspaces(WorkspaceQuery::default()).await?;
        for workspace in workspaces {
            if workspace.path == *path {
                return Ok(Some(workspace));
            }
        }
        Ok(None)
    }
    
    async fn update_workspace(&self, id: &WorkspaceId, update: WorkspaceUpdate) -> odi_core::Result<Option<Workspace>> {
        if let Some(mut workspace) = self.get_workspace(id).await? {
            // Apply updates
            if let Some(path) = update.path {
                workspace.path = path;
            }
            if let Some(projects) = update.projects {
                workspace.projects = projects;
            }
            
            // Store updated workspace
            self.create_workspace(workspace.clone()).await?;
            Ok(Some(workspace))
        } else {
            Ok(None)
        }
    }
    
    async fn delete_workspace(&self, id: &WorkspaceId) -> odi_core::Result<bool> {
        let ref_name = format!("workspaces/{}", id.to_string());
        
        let ref_obj = self.storage.get_ref(&ref_name)
            .map_err(|e| CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
            
        if let Some(obj_ref) = ref_obj {
            // Delete the object
            let deleted_obj = self.storage.delete_object(&obj_ref.hash)
                .map_err(|e| CoreError::ValidationError { 
                    field: "storage".to_string(), 
                    message: e.to_string() 
                })?;
                
            // Delete the reference
            let deleted_ref = self.storage.delete_ref(&ref_name)
                .map_err(|e| CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
                
            Ok(deleted_obj && deleted_ref)
        } else {
            Ok(false)
        }
    }
    
    async fn list_workspaces(&self, _query: WorkspaceQuery) -> odi_core::Result<Vec<Workspace>> {
        // TODO: Implement proper workspace filtering
        // For now, return empty list as workspaces are implicitly managed
        Ok(Vec::new())
    }
    
    async fn count_workspaces(&self, query: WorkspaceQuery) -> odi_core::Result<usize> {
        let workspaces = self.list_workspaces(query).await?;
        Ok(workspaces.len())
    }
    
    async fn get_project_workspaces(&self, project_id: &ProjectId) -> odi_core::Result<Vec<Workspace>> {
        let query = WorkspaceQuery {
            project_id: Some(project_id.clone()),
            ..Default::default()
        };
        self.list_workspaces(query).await
    }
    
    async fn get_workspace_projects(&self, workspace_id: &WorkspaceId) -> odi_core::Result<Vec<Project>> {
        let query = ProjectQuery {
            workspace_id: Some(workspace_id.clone()),
            ..Default::default()
        };
        self.list_projects(query).await
    }
    
    async fn link_project_workspace(&self, project_id: &ProjectId, workspace_id: &WorkspaceId) -> odi_core::Result<()> {
        // Get project and add workspace to it
        if let Some(mut project) = self.get_project(project_id).await? {
            if !project.workspaces.contains(workspace_id) {
                project.workspaces.push(workspace_id.clone());
                self.update_project(project_id, ProjectUpdate {
                    name: None,
                    description: None,
                    teams: None,
                    workspaces: Some(project.workspaces),
                }).await?;
            }
        }
        
        // Get workspace and add project to it
        if let Some(mut workspace) = self.get_workspace(workspace_id).await? {
            if !workspace.projects.contains(project_id) {
                workspace.projects.push(project_id.clone());
                self.update_workspace(workspace_id, WorkspaceUpdate {
                    path: None,
                    projects: Some(workspace.projects),
                }).await?;
            }
        }
        
        Ok(())
    }
    
    async fn unlink_project_workspace(&self, project_id: &ProjectId, workspace_id: &WorkspaceId) -> odi_core::Result<()> {
        // Remove workspace from project
        if let Some(mut project) = self.get_project(project_id).await? {
            project.workspaces.retain(|w| w != workspace_id);
            self.update_project(project_id, ProjectUpdate {
                name: None,
                description: None,
                teams: None,
                workspaces: Some(project.workspaces),
            }).await?;
        }
        
        // Remove project from workspace
        if let Some(mut workspace) = self.get_workspace(workspace_id).await? {
            workspace.projects.retain(|p| p != project_id);
            self.update_workspace(workspace_id, WorkspaceUpdate {
                path: None,
                projects: Some(workspace.projects),
            }).await?;
        }
        
        Ok(())
    }
    
    async fn create_label(&self, project_id: &ProjectId, label: Label) -> odi_core::Result<Label> {
        let serialized = serde_json::to_vec(&label)
            .map_err(CoreError::Serialization)?;
        
        let hash = self.storage.store_object(ObjectType::Label, &serialized)
            .map_err(|e| CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
            
        // Create reference for easy lookup
        self.storage.create_ref(&format!("labels/{}/{}", project_id.to_string(), label.id.to_string()), &hash, ObjectType::Label)
            .map_err(|e| CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
        
        Ok(label)
    }
    
    async fn get_label(&self, project_id: &ProjectId, label_id: &LabelId) -> odi_core::Result<Option<Label>> {
        let ref_name = format!("labels/{}/{}", project_id.to_string(), label_id.to_string());
        
        let ref_obj = self.storage.get_ref(&ref_name)
            .map_err(|e| CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
            
        if let Some(obj_ref) = ref_obj {
            let storage_obj = self.storage.retrieve_object(&obj_ref.hash)
                .map_err(|e| CoreError::ValidationError { 
                    field: "storage".to_string(), 
                    message: e.to_string() 
                })?;
                
            if let Some(obj) = storage_obj {
                let label: Label = serde_json::from_slice(&obj.data)
                    .map_err(CoreError::Serialization)?;
                Ok(Some(label))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    
    async fn update_label(&self, project_id: &ProjectId, label_id: &LabelId, label: Label) -> odi_core::Result<Option<Label>> {
        // Delete old label and create new one (labels are immutable objects)
        self.delete_label(project_id, label_id).await?;
        Ok(Some(self.create_label(project_id, label).await?))
    }
    
    async fn delete_label(&self, project_id: &ProjectId, label_id: &LabelId) -> odi_core::Result<bool> {
        let ref_name = format!("labels/{}/{}", project_id.to_string(), label_id.to_string());
        
        let ref_obj = self.storage.get_ref(&ref_name)
            .map_err(|e| CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
            
        if let Some(obj_ref) = ref_obj {
            // Delete the object
            let deleted_obj = self.storage.delete_object(&obj_ref.hash)
                .map_err(|e| CoreError::ValidationError { 
                    field: "storage".to_string(), 
                    message: e.to_string() 
                })?;
                
            // Delete the reference
            let deleted_ref = self.storage.delete_ref(&ref_name)
                .map_err(|e| CoreError::ValidationError { 
                    field: "storage".to_string(), 
                    message: e.to_string() 
                })?;
                
            Ok(deleted_obj && deleted_ref)
        } else {
            Ok(false)
        }
    }
    
    async fn list_labels(&self, project_id: &ProjectId, query: LabelQuery) -> odi_core::Result<Vec<Label>> {
        let mut labels = Vec::new();
        
        // Get all label objects for this project
        let hashes = self.storage.list_objects(Some(ObjectType::Label))
            .map_err(|e| CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
            
        for hash in hashes {
            if let Some(storage_obj) = self.storage.retrieve_object(&hash)
                .map_err(|e| CoreError::ValidationError { 
                    field: "storage".to_string(), 
                    message: e.to_string() 
                })? {
                
                let label: Label = serde_json::from_slice(&storage_obj.data)
                    .map_err(CoreError::Serialization)?;
                    
                // Apply filters
                let mut include = true;
                
                // Labels are project-scoped by storage location, not by field
                
                if let Some(color) = &query.color {
                    if &label.color != color {
                        include = false;
                    }
                }
                
                if include {
                    labels.push(label);
                }
            }
        }
        
        // Apply limit and offset
        if let Some(offset) = query.offset {
            if offset >= labels.len() {
                labels.clear();
            } else {
                labels.drain(0..offset);
            }
        }
        
        if let Some(limit) = query.limit {
            labels.truncate(limit);
        }
        
        Ok(labels)
    }
    
    async fn get_all_labels(&self, query: LabelQuery) -> odi_core::Result<Vec<Label>> {
        let mut labels = Vec::new();
        
        // Get all label objects
        let hashes = self.storage.list_objects(Some(ObjectType::Label))
            .map_err(|e| CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
            
        for hash in hashes {
            if let Some(storage_obj) = self.storage.retrieve_object(&hash)
                .map_err(|e| CoreError::ValidationError { 
                    field: "storage".to_string(), 
                    message: e.to_string() 
                })? {
                
                let label: Label = serde_json::from_slice(&storage_obj.data)
                    .map_err(CoreError::Serialization)?;
                    
                // Apply filters
                let mut include = true;
                
                // For get_all_labels, we can't filter by project since labels don't have project_id field
                // This would need to be implemented differently in a real system
                
                if let Some(color) = &query.color {
                    if &label.color != color {
                        include = false;
                    }
                }
                
                if include {
                    labels.push(label);
                }
            }
        }
        
        // Apply limit and offset
        if let Some(offset) = query.offset {
            if offset >= labels.len() {
                labels.clear();
            } else {
                labels.drain(0..offset);
            }
        }
        
        if let Some(limit) = query.limit {
            labels.truncate(limit);
        }
        
        Ok(labels)
    }
    
    async fn add_project_issue(&self, project_id: &ProjectId, issue_id: &IssueId) -> odi_core::Result<()> {
        // This is handled at the issue level - issues have a project_id field
        // No additional storage needed here
        Ok(())
    }
    
    async fn remove_project_issue(&self, project_id: &ProjectId, issue_id: &IssueId) -> odi_core::Result<()> {
        // This is handled at the issue level - issues have a project_id field
        // No additional storage needed here
        Ok(())
    }
    
    async fn add_project_team(&self, project_id: &ProjectId, team_id: &TeamId) -> odi_core::Result<()> {
        if let Some(mut project) = self.get_project(project_id).await? {
            if !project.teams.contains(team_id) {
                project.teams.push(team_id.clone());
                self.update_project(project_id, ProjectUpdate {
                    name: None,
                    description: None,
                    teams: Some(project.teams),
                    workspaces: None,
                }).await?;
            }
        }
        Ok(())
    }
    
    async fn remove_project_team(&self, project_id: &ProjectId, team_id: &TeamId) -> odi_core::Result<()> {
        if let Some(mut project) = self.get_project(project_id).await? {
            project.teams.retain(|t| t != team_id);
            self.update_project(project_id, ProjectUpdate {
                name: None,
                description: None,
                teams: Some(project.teams),
                workspaces: None,
            }).await?;
        }
        Ok(())
    }
}

/// User repository implementation using filesystem storage
pub struct FsUserRepository {
    storage: FileSystemStorage,
}

impl FsUserRepository {
    pub fn new(storage: FileSystemStorage) -> Self {
        Self { storage }
    }
    
    async fn load_team_by_ref(&self, obj_ref: &ObjectRef) -> odi_core::Result<Option<Team>> {
        if let Some(storage_obj) = self.storage.retrieve_object(&obj_ref.hash)
            .map_err(|e| CoreError::ValidationError { field: "storage".to_string(), message: e.to_string() })? {
            let team: Team = serde_json::from_slice(&storage_obj.data)
                .map_err(CoreError::Serialization)?;
            Ok(Some(team))
        } else {
            Ok(None)
        }
    }
    
    async fn load_user_by_ref(&self, obj_ref: &ObjectRef) -> odi_core::Result<Option<User>> {
        if let Some(storage_obj) = self.storage.retrieve_object(&obj_ref.hash)
            .map_err(|e| CoreError::ValidationError { field: "storage".to_string(), message: e.to_string() })? {
            let user: User = serde_json::from_slice(&storage_obj.data)
                .map_err(CoreError::Serialization)?;
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }
}

#[async_trait::async_trait]
impl UserRepository for FsUserRepository {
    async fn create_user(&self, user: User) -> odi_core::Result<User> {
        let serialized = serde_json::to_vec(&user).map_err(CoreError::Serialization)?;
        let hash = self.storage.store_object(ObjectType::User, &serialized)
            .map_err(|e| CoreError::ValidationError { field: "storage".to_string(), message: e.to_string() })?;
        self.storage.create_ref(&format!("users/{}", user.id.to_string()), &hash, ObjectType::User)
            .map_err(|e| CoreError::ValidationError { field: "storage".to_string(), message: e.to_string() })?;
        Ok(user)
    }
    
    async fn get_user(&self, id: &UserId) -> odi_core::Result<Option<User>> {
        let ref_name = format!("users/{}", id.to_string());
        let ref_obj = self.storage.get_ref(&ref_name)
            .map_err(|e| CoreError::ValidationError { field: "storage".to_string(), message: e.to_string() })?;
        if let Some(obj_ref) = ref_obj {
            if let Some(storage_obj) = self.storage.retrieve_object(&obj_ref.hash)
                .map_err(|e| CoreError::ValidationError { field: "storage".to_string(), message: e.to_string() })? {
                let user: User = serde_json::from_slice(&storage_obj.data).map_err(CoreError::Serialization)?;
                return Ok(Some(user));
            }
        }
        Ok(None)
    }
    
    async fn update_user(&self, id: &UserId, update: UserUpdate) -> odi_core::Result<Option<User>> {
        if let Some(mut user) = self.get_user(id).await? {
            if let Some(name) = update.name { user.name = name; }
            if let Some(email) = update.email { user.email = email; }
            if let Some(avatar) = update.avatar { user.avatar = avatar; }
            if let Some(teams) = update.teams { user.teams = teams; }
            self.create_user(user.clone()).await?;
            Ok(Some(user))
        } else { Ok(None) }
    }
    
    async fn delete_user(&self, id: &UserId) -> odi_core::Result<bool> {
        let ref_name = format!("users/{}", id.to_string());
        if let Some(obj_ref) = self.storage.get_ref(&ref_name)
            .map_err(|e| CoreError::ValidationError { field: "storage".to_string(), message: e.to_string() })? {
            let deleted_obj = self.storage.delete_object(&obj_ref.hash)
                .map_err(|e| CoreError::ValidationError { field: "storage".to_string(), message: e.to_string() })?;
            let deleted_ref = self.storage.delete_ref(&ref_name)
                .map_err(|e| CoreError::ValidationError { field: "storage".to_string(), message: e.to_string() })?;
            Ok(deleted_obj && deleted_ref)
        } else { Ok(false) }
    }
    
    async fn list_users(&self, _query: UserQuery) -> odi_core::Result<Vec<User>> {
        // Get all user refs and load them
        let refs = self.storage.list_refs()
            .map_err(|e| CoreError::ValidationError { field: "storage".to_string(), message: e.to_string() })?;
        
        let mut users = Vec::new();
        for obj_ref in refs {
            if obj_ref.name.starts_with("users/") {
                if let Some(user) = self.load_user_by_ref(&obj_ref).await? {
                    users.push(user);
                }
            }
        }
        Ok(users)
    }
    async fn count_users(&self, query: UserQuery) -> odi_core::Result<usize> { Ok(self.list_users(query).await?.len()) }
    async fn get_user_by_email(&self, email: &str) -> odi_core::Result<Option<User>> {
        // Search through all users to find one with matching email
        let users = self.list_users(UserQuery::default()).await?;
        Ok(users.into_iter().find(|user| user.email == email))
    }
    async fn search_users(&self, _query: &str) -> odi_core::Result<Vec<User>> { Ok(Vec::new()) }
    async fn get_user_teams(&self, user_id: &UserId) -> odi_core::Result<Vec<Team>> { Ok(Vec::new()) }
    async fn get_team_members(&self, team_id: &TeamId) -> odi_core::Result<Vec<User>> {
        // For simplicity, we'll check which users have this team in their teams list
        let users = self.list_users(UserQuery::default()).await?;
        let members: Vec<User> = users.into_iter()
            .filter(|user| user.teams.contains(team_id))
            .collect();
        Ok(members)
    }
    async fn add_team_member(&self, team_id: &TeamId, user_id: &UserId) -> odi_core::Result<()> {
        // Get the user and add the team to their teams list
        if let Some(mut user) = self.get_user(user_id).await? {
            if !user.teams.contains(team_id) {
                user.teams.push(team_id.clone());
                self.create_user(user).await?; // This will overwrite the existing user
            }
        }
        Ok(())
    }
    async fn remove_team_member(&self, team_id: &TeamId, user_id: &UserId) -> odi_core::Result<()> {
        // Get the user and remove the team from their teams list
        if let Some(mut user) = self.get_user(user_id).await? {
            if let Some(pos) = user.teams.iter().position(|x| x == team_id) {
                user.teams.remove(pos);
                self.create_user(user).await?; // This will overwrite the existing user
            }
        }
        Ok(())
    }
    
    async fn create_team(&self, team: Team) -> odi_core::Result<Team> {
        let serialized = serde_json::to_vec(&team).map_err(CoreError::Serialization)?;
        let hash = self.storage.store_object(ObjectType::Team, &serialized)
            .map_err(|e| CoreError::ValidationError { field: "storage".to_string(), message: e.to_string() })?;
        self.storage.create_ref(&format!("teams/{}", team.id.to_string()), &hash, ObjectType::Team)
            .map_err(|e| CoreError::ValidationError { field: "storage".to_string(), message: e.to_string() })?;
        Ok(team)
    }
    async fn get_team(&self, id: &TeamId) -> odi_core::Result<Option<Team>> {
        let ref_name = format!("teams/{}", id.to_string());
        if let Some(obj_ref) = self.storage.get_ref(&ref_name)
            .map_err(|e| CoreError::ValidationError { field: "storage".to_string(), message: e.to_string() })? {
            if let Some(storage_obj) = self.storage.retrieve_object(&obj_ref.hash)
                .map_err(|e| CoreError::ValidationError { field: "storage".to_string(), message: e.to_string() })? {
                let team: Team = serde_json::from_slice(&storage_obj.data).map_err(CoreError::Serialization)?;
                return Ok(Some(team));
            }
        }
        Ok(None)
    }
    async fn update_team(&self, id: &TeamId, update: TeamUpdate) -> odi_core::Result<Option<Team>> {
        if let Some(mut team) = self.get_team(id).await? {
            if let Some(name) = update.name { team.name = name; }
            if let Some(description) = update.description { team.description = description; }
            if let Some(members) = update.members { team.members = members; }
            self.create_team(team.clone()).await?;
            Ok(Some(team))
        } else { Ok(None) }
    }
    async fn delete_team(&self, id: &TeamId) -> odi_core::Result<bool> {
        let ref_name = format!("teams/{}", id.to_string());
        if let Some(obj_ref) = self.storage.get_ref(&ref_name)
            .map_err(|e| CoreError::ValidationError { field: "storage".to_string(), message: e.to_string() })? {
            let deleted_obj = self.storage.delete_object(&obj_ref.hash)
                .map_err(|e| CoreError::ValidationError { field: "storage".to_string(), message: e.to_string() })?;
            let deleted_ref = self.storage.delete_ref(&ref_name)
                .map_err(|e| CoreError::ValidationError { field: "storage".to_string(), message: e.to_string() })?;
            Ok(deleted_obj && deleted_ref)
        } else { Ok(false) }
    }
    async fn list_teams(&self, _query: TeamQuery) -> odi_core::Result<Vec<Team>> {
        // Get all team refs and load them
        let refs = self.storage.list_refs()
            .map_err(|e| CoreError::ValidationError { field: "storage".to_string(), message: e.to_string() })?;
        
        let mut teams = Vec::new();
        for obj_ref in refs {
            if obj_ref.name.starts_with("teams/") {
                if let Some(team) = self.load_team_by_ref(&obj_ref).await? {
                    teams.push(team);
                }
            }
        }
        Ok(teams)
    }
    async fn count_teams(&self, query: TeamQuery) -> odi_core::Result<usize> { Ok(self.list_teams(query).await?.len()) }
}

/// Remote repository implementation using filesystem storage
pub struct FsRemoteRepository {
    storage: FileSystemStorage,
}

impl FsRemoteRepository {
    pub fn new(storage: FileSystemStorage) -> Self {
        Self { storage }
    }
}

#[async_trait::async_trait]
impl RemoteRepository for FsRemoteRepository {
    async fn create(&self, remote: odi_core::Remote) -> odi_core::Result<odi_core::Remote> {
        let serialized = serde_json::to_vec(&remote)
            .map_err(odi_core::CoreError::Serialization)?;
        
        let hash = self.storage.store_object(ObjectType::Remote, &serialized)
            .map_err(|e| odi_core::CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
            
        // Create reference for easy lookup by ID
        self.storage.create_ref(&format!("remotes/{}", remote.id), &hash, ObjectType::Remote)
            .map_err(|e| odi_core::CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
        
        Ok(remote)
    }
    
    async fn get(&self, id: &odi_core::RemoteId) -> odi_core::Result<Option<odi_core::Remote>> {
        let ref_name = format!("remotes/{}", id);
        
        let ref_obj = self.storage.get_ref(&ref_name)
            .map_err(|e| odi_core::CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
            
        if let Some(obj_ref) = ref_obj {
            let storage_obj = self.storage.retrieve_object(&obj_ref.hash)
                .map_err(|e| odi_core::CoreError::ValidationError { 
                    field: "storage".to_string(), 
                    message: e.to_string() 
                })?;
                
            if let Some(obj) = storage_obj {
                let remote: odi_core::Remote = serde_json::from_slice(&obj.data)
                    .map_err(odi_core::CoreError::Serialization)?;
                Ok(Some(remote))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    
    async fn update(&self, id: &odi_core::RemoteId, remote: odi_core::Remote) -> odi_core::Result<Option<odi_core::Remote>> {
        // Check if remote exists first
        if self.get(id).await?.is_some() {
            // Store the updated remote (will overwrite existing)
            let updated = self.create(remote).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }
    
    async fn delete(&self, id: &odi_core::RemoteId) -> odi_core::Result<bool> {
        let ref_name = format!("remotes/{}", id);
        
        let ref_obj = self.storage.get_ref(&ref_name)
            .map_err(|e| odi_core::CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
            
        if let Some(obj_ref) = ref_obj {
            // Delete the object
            let deleted_obj = self.storage.delete_object(&obj_ref.hash)
                .map_err(|e| odi_core::CoreError::ValidationError { 
                    field: "storage".to_string(), 
                    message: e.to_string() 
                })?;
                
            // Delete the reference
            let deleted_ref = self.storage.delete_ref(&ref_name)
                .map_err(|e| odi_core::CoreError::ValidationError { 
                    field: "storage".to_string(), 
                    message: e.to_string() 
                })?;
                
            Ok(deleted_obj && deleted_ref)
        } else {
            Ok(false)
        }
    }
    
    async fn list(&self) -> odi_core::Result<Vec<odi_core::Remote>> {
        let mut remotes = Vec::new();
        
        // Get all remote objects
        let hashes = self.storage.list_objects(Some(ObjectType::Remote))
            .map_err(|e| odi_core::CoreError::ValidationError { 
                field: "storage".to_string(), 
                message: e.to_string() 
            })?;
            
        for hash in hashes {
            if let Some(storage_obj) = self.storage.retrieve_object(&hash)
                .map_err(|e| odi_core::CoreError::ValidationError { 
                    field: "storage".to_string(), 
                    message: e.to_string() 
                })? {
                
                let remote: odi_core::Remote = serde_json::from_slice(&storage_obj.data)
                    .map_err(odi_core::CoreError::Serialization)?;
                    
                remotes.push(remote);
            }
        }
        
        Ok(remotes)
    }
    
    async fn get_by_project(&self, project_id: &odi_core::ProjectId) -> odi_core::Result<Vec<odi_core::Remote>> {
        let mut remotes = Vec::new();
        let all_remotes = self.list().await?;
        
        for remote in all_remotes {
            if remote.has_project(project_id) {
                remotes.push(remote);
            }
        }
        
        Ok(remotes)
    }
    
    async fn exists(&self, name: &str) -> odi_core::Result<bool> {
        let remotes = self.list().await?;
        Ok(remotes.iter().any(|r| r.name == name))
    }
    
    async fn get_by_name(&self, name: &str) -> odi_core::Result<Option<odi_core::Remote>> {
        let remotes = self.list().await?;
        Ok(remotes.into_iter().find(|r| r.name == name))
    }
}