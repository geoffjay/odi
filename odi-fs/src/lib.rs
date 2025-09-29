//! # ODI Filesystem
//!
//! Filesystem operations for ODI distributed issue tracking system.
//! This crate provides storage, configuration, and Git integration capabilities.

pub mod config;
pub mod error;
pub mod git;
pub mod storage;

// Re-exports for consumers  
pub use config::{Config, ConfigLoader, DefaultConfigLoader};
pub use error::{FsError, Result};
pub use git::{DefaultGitIntegration, GitIntegration, GitRef, GitRepository};
pub use storage::{FilesystemStorage, ObjectHash, ObjectType, StorageEngine};

/// Current version of the ODI filesystem library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");