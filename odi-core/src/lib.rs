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

// Issue entities and types
pub use issue::{
    Issue, IssueId, IssueStatus, Priority,
    IssueRepository, IssueQuery, IssueUpdate,
};

// User entities and types
pub use user::{
    User, UserId, Team, TeamId,
    UserRepository, UserQuery, UserUpdate, TeamQuery, TeamUpdate,
};

// Project entities and types
pub use project::{
    Project, ProjectId, Workspace, WorkspaceId, Label, LabelId,
    ProjectRepository, ProjectQuery, ProjectUpdate, WorkspaceQuery, WorkspaceUpdate, LabelQuery,
};

// Sync entities and types
pub use sync::{
    Remote, RemoteId, RemoteRepository, SyncEngine, SyncResult, Conflict, ConflictType, ConflictResolution,
    SyncOptions, SyncStats, BatchConflictStrategy, RemoteInfo, detect_conflict_type,
};

/// Current version of the ODI core library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");