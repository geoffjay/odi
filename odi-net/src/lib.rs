//! ODI Network operations crate
//!
//! Provides networking capabilities for the ODI distributed issue tracking system.

use thiserror::Error;

// Re-export main modules
pub mod auth;
pub mod protocol;
pub mod sync;

// Re-export important types
pub use auth::{AuthToken, Credential};
pub use protocol::{Protocol, ProtocolHandler};
pub use sync::{RemoteSync, RemoteSyncState, SyncMetadata, IssueMetadata, SyncClient, DefaultRemoteSync};

#[derive(Error, Debug)]
pub enum NetError {
    #[error("Network error: {0}")]
    Network(#[from] std::io::Error),

    #[error("Authentication error: {message}")]
    Authentication { message: String },

    #[error("Protocol error: {message}")]
    Protocol { message: String },

    #[error("Sync error: {message}")]
    Sync { message: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, NetError>;
