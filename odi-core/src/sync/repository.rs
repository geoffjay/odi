//! Remote repository trait definition

use async_trait::async_trait;
use crate::{Result, project::ProjectId, sync::{Remote, RemoteId}};

/// Repository trait for remote management operations
#[async_trait]
pub trait RemoteRepository: Send + Sync {
    /// Create a new remote
    async fn create(&self, remote: Remote) -> Result<Remote>;
    
    /// Get a remote by ID
    async fn get(&self, id: &RemoteId) -> Result<Option<Remote>>;
    
    /// Update a remote
    async fn update(&self, id: &RemoteId, remote: Remote) -> Result<Option<Remote>>;
    
    /// Delete a remote
    async fn delete(&self, id: &RemoteId) -> Result<bool>;
    
    /// List all remotes
    async fn list(&self) -> Result<Vec<Remote>>;
    
    /// Get remotes for a specific project
    async fn get_by_project(&self, project_id: &ProjectId) -> Result<Vec<Remote>>;
    
    /// Check if a remote exists by name
    async fn exists(&self, name: &str) -> Result<bool>;
    
    /// Find remote by name
    async fn get_by_name(&self, name: &str) -> Result<Option<Remote>>;
}