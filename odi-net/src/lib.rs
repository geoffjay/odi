//! # ODI Network
//!
//! Network operations for ODI distributed issue tracking system.
//! This crate provides remote synchronization, protocol handlers, and authentication.

pub mod auth;
pub mod error;
pub mod protocols;
pub mod sync;

// Re-exports for consumers
pub use auth::{AuthToken, Authentication, Credential};
pub use error::{NetError, Result};
pub use protocols::{Protocol, ProtocolHandler};
pub use sync::{RemoteSync, SyncClient};

/// Current version of the ODI network library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");