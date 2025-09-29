//! Error types for ODI Filesystem

use thiserror::Error;

/// Result type for filesystem operations
pub type Result<T> = std::result::Result<T, FsError>;

/// Error type for filesystem operations
#[derive(Debug, Error)]
pub enum FsError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },

    #[error("Permission denied: {path}")]
    PermissionDenied { path: String },

    #[error("Invalid configuration: {message}")]
    ConfigError { message: String },

    #[error("Git error: {message}")]
    GitError { message: String },

    #[error("Object store error: {message}")]
    ObjectStoreError { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("Core error: {0}")]
    Core(#[from] odi_core::CoreError),
}