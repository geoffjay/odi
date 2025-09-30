//! # ODI Filesystem Operations
//!
//! Provides filesystem storage, configuration management, and Git integration
//! for the ODI distributed issue tracking system.
//!
//! ## Architecture
//!
//! This crate implements the storage layer for ODI with:
//! - **Object Storage**: Git-like binary object storage in `.odi/objects/`
//! - **Configuration**: TOML-based configuration with hierarchy support
//! - **Repositories**: Filesystem implementations of domain repository traits
//! - **Git Integration**: Automatic detection and association with Git repositories
//!
//! ## Example Usage
//!
//! ### Object Storage
//! ```rust,ignore
//! use odi_fs::{FileSystemStorage, ObjectStorage};
//! use std::path::Path;
//!
//! // Initialize storage engine
//! let storage = FileSystemStorage::new(Path::new(".odi")).await?;
//!
//! // Store objects (issues, projects, etc.) as binary data
//! let object_data = bincode::serialize(&issue)?;
//! let hash = storage.store_object(&object_data).await?;
//!
//! // Retrieve objects by hash
//! let retrieved = storage.load_object(&hash).await?;
//! ```
//!
//! ### Configuration Management
//! ```rust,ignore
//! use odi_fs::{FileConfigLoader, Config};
//!
//! // Load configuration hierarchy: ~/.odiconfig then ./.odi/config
//! let global = FileConfigLoader::load_global()?;
//! let local = FileConfigLoader::load_local(Path::new("."))?;
//! let config = FileConfigLoader::merge(global, local);
//! ```
//!
//! ### Repository Pattern
//! ```rust,ignore
//! use odi_fs::FsIssueRepository;
//! use odi_core::IssueRepository;
//!
//! // Create filesystem-backed repository
//! let repo = FsIssueRepository::new(storage);
//! 
//! // Use repository trait methods
//! let issues = repo.list(Default::default()).await?;
//! ```
//!
//! ## Features
//!
//! - **Git-like Storage**: Content-addressable object store with integrity checking
//! - **Configuration Hierarchy**: Global and local config with merging and validation
//! - **Atomic Operations**: Filesystem locks prevent concurrent access issues
//! - **Git Detection**: Automatically finds and integrates with Git repositories
//! - **Performance**: Efficient binary serialization and indexed lookups

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
