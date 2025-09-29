//! # ODI Core
//!
//! Core domain logic for ODI distributed issue tracking system.
//! This crate provides the fundamental entities, traits, and business rules
//! for managing issues, users, teams, projects, and synchronization.

pub mod error;
pub mod issue;
pub mod project;
pub mod sync;
pub mod user;

// Re-exports for consumers
pub use error::{CoreError, Result};
pub use issue::{Issue, IssueId, IssueStatus, Priority};
pub use project::{Label, LabelId, Project, ProjectId, Workspace, WorkspaceId};
pub use sync::{ConflictResolution, ConflictType, Remote, RemoteId, SyncEngine, SyncResult};
pub use user::{Team, TeamId, User, UserId};

/// Current version of the ODI core library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");