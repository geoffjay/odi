//! Object storage implementation

use std::path::Path;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::Result;

/// Object hash type (SHA-256)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObjectHash(pub String);

/// Object type enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectType {
    Issue,
    User,
    Team,
    Project,
    Workspace,
    Remote,
    Label,
}

/// Storage engine trait
#[async_trait::async_trait]
pub trait StorageEngine {
    async fn initialize(&self, path: &Path) -> Result<()>;
    
    // Object store operations (Git-like)
    async fn write_object<T: Serialize + Send>(&self, obj: &T) -> Result<ObjectHash>;
    async fn read_object<T: for<'de> Deserialize<'de>>(&self, hash: &ObjectHash) -> Result<Option<T>>;
    async fn delete_object(&self, hash: &ObjectHash) -> Result<()>;
    async fn list_objects(&self, object_type: ObjectType) -> Result<Vec<ObjectHash>>;
    
    // Reference operations
    async fn write_ref(&self, name: &str, hash: &ObjectHash) -> Result<()>;
    async fn read_ref(&self, name: &str) -> Result<Option<ObjectHash>>;
    async fn list_refs(&self, prefix: &str) -> Result<Vec<String>>;
    
    // Locking operations
    async fn lock(&self, resource: &str) -> Result<Lock>;
    async fn unlock(&self, lock: Lock) -> Result<()>;
}

/// File lock for concurrent access
#[derive(Debug)]
pub struct Lock {
    pub resource: String,
    pub acquired_at: DateTime<Utc>,
    pub lock_file: std::path::PathBuf,
}

impl std::fmt::Display for ObjectHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}