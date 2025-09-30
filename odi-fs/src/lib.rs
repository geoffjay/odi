//! ODI Filesystem operations crate
//!
//! Provides filesystem storage, configuration management, and Git integration
//! for the ODI distributed issue tracking system.

use thiserror::Error;

// Re-export main modules
pub mod config;
pub mod storage;
pub mod git;
pub mod repository;

// Re-export important types
pub use config::{Config, UserConfig, ProjectConfig, RemoteConfig, WorkspaceConfig, ConfigLoader, FileConfigLoader, load_config, save_config};
pub use storage::{ObjectType, ObjectHash, StorageObject, ObjectRef, StorageLock, Lock, ObjectStorage, StorageEngine, FileSystemStorage};
pub use repository::{FsIssueRepository, FsProjectRepository, FsUserRepository, FsRemoteRepository};

#[derive(Error, Debug)]
pub enum FsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {message}")]
    ConfigError { message: String },

    #[error("Storage error: {message}")]
    StorageError { message: String },

    #[error("Serialization error: {message}")]
    SerializationError { message: String },

    #[error("Lock error: {message}")]
    LockError { message: String },

    #[error("Git integration error: {message}")]
    GitError { message: String },

    #[error("TOML parsing error: {0}")]
    TomlError(#[from] toml::de::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, FsError>;
