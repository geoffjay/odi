//! Error types for ODI CLI

use thiserror::Error;

/// Result type for CLI operations
pub type Result<T> = std::result::Result<T, OdiError>;

/// Error type for CLI operations
#[derive(Debug, Error)]
pub enum OdiError {
    #[error("Core error: {0}")]
    Core(#[from] odi_core::CoreError),

    #[error("Filesystem error: {0}")]
    Filesystem(#[from] odi_fs::FsError),

    #[error("Network error: {0}")]
    Network(#[from] odi_net::NetError),

    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Command error: {message}")]
    Command { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}