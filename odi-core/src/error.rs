//! Error types for ODI Core

use thiserror::Error;

/// Result type for ODI Core operations
pub type Result<T> = std::result::Result<T, CoreError>;

/// Error type for core domain operations
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Issue not found: {id}")]
    IssueNotFound { id: String },

    #[error("User not found: {id}")]
    UserNotFound { id: String },

    #[error("Project not found: {id}")]
    ProjectNotFound { id: String },

    #[error("Invalid issue status transition: {from} -> {to}")]
    InvalidStatusTransition { from: String, to: String },

    #[error("Permission denied: {operation}")]
    PermissionDenied { operation: String },

    #[error("Validation failed: {field}: {message}")]
    ValidationError { field: String, message: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("UUID error: {0}")]
    Uuid(#[from] uuid::Error),
}