//! # ODI Core
//!
//! Core domain logic for ODI distributed issue tracking system.
//! This crate provides the fundamental entities, traits, and business rules
//! for managing issues, users, teams, projects, and synchronization.
//!
//! ## Architecture
//!
//! ODI Core follows Domain-Driven Design principles with clear separation between:
//! - **Entities**: Core business objects (Issue, User, Project, etc.)
//! - **Repositories**: Data access trait definitions
//! - **Services**: Business logic and workflow coordination
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use odi_core::{Issue, IssueStatus, Priority};
//! 
//! // Create a new issue
//! let issue = Issue::new(
//!     "Fix login bug".to_string(),
//!     "Users cannot login with special characters".to_string(),
//! );
//! 
//! // Issues have unique IDs and timestamps
//! println!("Issue ID: {}", issue.id);
//! println!("Created: {}", issue.created_at);
//!
//! // Repository pattern for data access  
//! async fn create_issue(repo: &dyn IssueRepository, issue: Issue) -> Result<()> {
//!     repo.create(issue).await
//! }
//! ```
//!
//! ## Features
//!
//! - **Type Safety**: Strong typing with newtype patterns (IssueId, UserId, etc.)
//! - **Async Support**: All repository operations are async-first
//! - **Serialization**: All entities support serde for storage/network
//! - **Validation**: Input validation and business rule enforcement
//! - **Error Handling**: Comprehensive error types with context

pub mod error;
pub mod issue;
pub mod project;
pub mod sync;
pub mod user;

// Re-exports for consumers
pub use error::{CoreError, Result};

// Legacy alias for Error (backward compatibility)
pub use CoreError as Error;

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