//! # ODI Network Operations
//!
//! Provides networking capabilities for the ODI distributed issue tracking system.
//! This crate implements distributed synchronization via SSH and HTTPS protocols,
//! enabling Git-like push/pull workflows for issue collaboration.
//!
//! ## Architecture
//!
//! The networking layer consists of:
//! - **Protocol Handlers**: SSH and HTTPS transport implementations
//! - **Authentication**: Key-based SSH and token-based HTTPS auth
//! - **Synchronization**: Bidirectional sync with conflict detection
//! - **Remote Management**: Connection pooling and session management
//!
//! ## Example Usage
//!
//! ### Remote Synchronization
//! ```rust,ignore
//! use odi_net::{DefaultRemoteSync, RemoteSync};
//! use odi_core::Remote;
//!
//! let sync = DefaultRemoteSync::new();
//! let remote = Remote::new("origin", "ssh://git@server/repo.odi");
//!
//! // Connect to remote
//! let client = sync.connect(&remote).await?;
//!
//! // Push local issues
//! for issue in local_issues {
//!     sync.upload_issue(&client, &issue).await?;
//! }
//!
//! // Pull remote issues  
//! let remote_issues = sync.list_issues(&client).await?;
//! for metadata in remote_issues {
//!     let issue = sync.download_issue(&client, &metadata.id).await?;
//!     // Merge with local state...
//! }
//! ```
//!
//! ### Protocol Handlers
//! ```rust,ignore
//! use odi_net::{Protocol, ProtocolHandler, SshHandler, HttpsHandler};
//!
//! // SSH protocol for secure key-based access
//! let ssh = SshHandler::new();
//! let objects = ssh.list_objects("ssh://server/repo", "issues").await?;
//!
//! // HTTPS protocol for token-based access  
//! let https = HttpsHandler::new();
//! let data = https.download_object("https://server/repo", "issues/123.bin").await?;
//! ```
//!
//! ### Authentication
//! ```rust,ignore
//! use odi_net::{Credential, AuthToken};
//! use std::path::PathBuf;
//!
//! // SSH key authentication
//! let ssh_key = Credential::SshKey {
//!     path: PathBuf::from("~/.ssh/id_rsa"),
//!     passphrase: None,
//! };
//!
//! // Token authentication for HTTPS
//! let token = Credential::Token {
//!     value: "github_pat_...".to_string(),
//! };
//! ```
//!
//! ## Features
//!
//! - **Multi-Protocol**: SSH and HTTPS transport with authentication
//! - **Conflict Detection**: Timestamp-based merge conflict identification  
//! - **Binary Transfer**: Efficient object serialization for network transport
//! - **Session Management**: Connection pooling and credential caching
//! - **Error Recovery**: Automatic retry and graceful error handling

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
