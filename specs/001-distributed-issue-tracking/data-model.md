# Data Model: Distributed Issue Tracking System

**Prerequisites**: research.md architecture decisions
**Scope**: Entity definitions, relationships, and validation rules

## Core Entities

### Issue
**Purpose**: Primary tracking unit for work items, bugs, and tasks

**Fields**:
- `id`: String - Unique identifier (UUID v4 format)
- `title`: String - Brief description (1-100 characters)
- `description`: Option<String> - Detailed description (markdown supported)
- `status`: IssueStatus - Current state (Open, InProgress, Resolved, Closed)
- `priority`: Priority - Urgency level (Low, Medium, High, Critical)
- `assignees`: Vec<UserId> - Users assigned to this issue
- `author`: UserId - User who created the issue
- `co_authors`: Vec<UserId> - Additional authors/contributors
- `labels`: Vec<LabelId> - Categorization tags
- `project_id`: Option<ProjectId> - Associated project
- `created_at`: DateTime<Utc> - Creation timestamp
- `updated_at`: DateTime<Utc> - Last modification timestamp
- `closed_at`: Option<DateTime<Utc>> - Resolution timestamp
- `git_refs`: Vec<GitRef> - Associated Git commits/branches

**Validation Rules**:
- Title must be non-empty and ≤ 100 characters
- Author must be valid user ID
- Assignees must be valid user IDs
- Status transitions must follow valid state machine
- Git refs must be valid commit SHA or branch name format

**State Transitions**:
```
Open → InProgress → Resolved → Closed
  ↓         ↓          ↓        ↑
  └────────→ Closed ←──────────┘
```

### User
**Purpose**: Individual identity with authentication and team membership

**Fields**:
- `id`: UserId - Unique identifier (username format)
- `name`: String - Display name (1-50 characters)
- `email`: String - Contact email (valid email format)
- `avatar`: Option<String> - Profile image URL or path
- `teams`: Vec<TeamId> - Team memberships
- `roles`: Vec<Role> - Project-specific roles and permissions
- `ssh_keys`: Vec<SshKey> - Public keys for authentication
- `created_at`: DateTime<Utc> - Account creation timestamp
- `last_active`: DateTime<Utc> - Last activity timestamp

**Validation Rules**:
- ID must match regex `^[a-zA-Z0-9_-]+$` (3-30 characters)
- Name must be non-empty and ≤ 50 characters  
- Email must be valid format and unique within project
- SSH keys must be valid public key format

### Team
**Purpose**: Group of users with shared permissions and project access

**Fields**:
- `id`: TeamId - Unique identifier (team name format)
- `name`: String - Display name (1-50 characters)  
- `description`: Option<String> - Team purpose description
- `members`: Vec<UserId> - Team member user IDs
- `permissions`: Vec<Permission> - Team-level permissions
- `projects`: Vec<ProjectId> - Accessible projects
- `created_at`: DateTime<Utc> - Creation timestamp
- `updated_at`: DateTime<Utc> - Last modification timestamp

**Validation Rules**:
- ID must match regex `^[a-zA-Z0-9_-]+$` (3-30 characters)
- Name must be non-empty and ≤ 50 characters
- Members must be valid user IDs
- Must have at least one member (creator)

### Project
**Purpose**: Container for related issues with configuration and team assignments

**Fields**:
- `id`: ProjectId - Unique identifier (project name format)
- `name`: String - Display name (1-100 characters)
- `description`: Option<String> - Project purpose and goals
- `issues`: Vec<IssueId> - Associated issue IDs
- `teams`: Vec<TeamId> - Teams with project access
- `labels`: Vec<Label> - Available labels for issues
- `git_repository`: Option<GitRepository> - Associated Git repo
- `remotes`: Vec<Remote> - Remote repository connections
- `settings`: ProjectSettings - Project-specific configuration
- `created_at`: DateTime<Utc> - Creation timestamp  
- `updated_at`: DateTime<Utc> - Last modification timestamp

**Validation Rules**:
- ID must match regex `^[a-zA-Z0-9_.-]+$` (3-100 characters)
- Name must be non-empty and ≤ 100 characters
- Teams must be valid team IDs
- Git repository path must be valid if specified

### Label
**Purpose**: Categorization tag with visual representation for issue organization

**Fields**:
- `id`: LabelId - Unique identifier within project
- `name`: String - Display name (1-30 characters)
- `description`: Option<String> - Label purpose description
- `color`: String - Hex color code (#RRGGBB format)
- `created_at`: DateTime<Utc> - Creation timestamp

**Validation Rules**:
- ID must be unique within project scope
- Name must be non-empty and ≤ 30 characters
- Color must be valid hex format (#[0-9A-Fa-f]{6})
- Name must be unique within project

### Remote
**Purpose**: External repository connection with protocol and synchronization state

**Fields**:
- `id`: RemoteId - Unique identifier (remote name)
- `name`: String - Display name (typically 'origin', 'upstream')
- `url`: String - Repository URL (SSH or HTTPS)
- `protocol`: Protocol - Connection type (SSH, HTTPS)
- `auth`: AuthConfig - Authentication configuration
- `last_sync`: Option<DateTime<Utc>> - Last successful sync
- `sync_state`: SyncState - Current synchronization status
- `created_at`: DateTime<Utc> - Creation timestamp

**Validation Rules**:
- ID must match regex `^[a-zA-Z0-9_-]+$` (1-50 characters)
- URL must be valid SSH or HTTPS format
- Protocol must match URL scheme
- Auth config must be valid for protocol type

### Config
**Purpose**: Hierarchical settings with global and local scopes

**Fields**:
- `user`: UserConfig - User identity and preferences
- `project`: ProjectConfig - Project-specific settings
- `remote`: RemoteConfig - Default remote settings
- `ui`: UiConfig - Interface preferences
- `sync`: SyncConfig - Synchronization behavior

**Validation Rules**:
- All nested configs must pass individual validation
- Required fields must be present (user.name, user.email)
- Optional fields have sensible defaults

## Supporting Types

### Enumerations
```rust
pub enum IssueStatus { Open, InProgress, Resolved, Closed }
pub enum Priority { Low, Medium, High, Critical }  
pub enum Protocol { SSH, HTTPS }
pub enum SyncState { Clean, Dirty, Syncing, Conflict }
pub enum Permission { Read, Write, Admin }
```

### Complex Types
```rust
pub struct GitRef {
    pub ref_type: GitRefType, // Commit, Branch, Tag
    pub sha: String,          // Git commit SHA
    pub message: Option<String>, // Commit message if available
}

pub struct GitRepository {
    pub path: PathBuf,        // Local repository path
    pub remote_url: Option<String>, // Origin remote URL
    pub branch: Option<String>, // Current branch
}

pub struct AuthConfig {
    pub method: AuthMethod,   // SSHKey, Token, OAuth
    pub credential: String,   // Key path, token value, etc.
}
```

## Relationships

### Entity Relationships
- **User** ←→ **Team**: Many-to-many (users belong to multiple teams)
- **Team** ←→ **Project**: Many-to-many (teams access multiple projects)  
- **Project** ←→ **Issue**: One-to-many (project contains multiple issues)
- **Issue** ←→ **User**: Many-to-many (assignees), One-to-many (author)
- **Issue** ←→ **Label**: Many-to-many (issues have multiple labels)
- **Project** ←→ **Remote**: One-to-many (project has multiple remotes)

### Data Dependencies
- Issues depend on Users (author, assignees) and Projects
- Teams depend on Users (members) and Projects (access)
- Remotes depend on Projects (ownership) and Auth (credentials)
- Labels depend on Projects (scoped definition)

## Storage Implementation

### File Structure
```
.odi/
├── config.toml           # Local project configuration
├── issues/
│   ├── {issue-id}.json   # Individual issue files
│   └── index.toml        # Issue index and metadata
├── users.toml            # User definitions and team memberships
├── projects.toml         # Project metadata and settings
├── labels.toml           # Label definitions per project
├── remotes.toml          # Remote repository configuration
└── state/
    ├── sync.toml         # Synchronization state
    └── locks/            # File locking for concurrent access
```

### Serialization Format
- **TOML**: Configuration files, metadata, small structured data
- **JSON**: Issue content, large structured data, cross-language compatibility
- **Compression**: Optional gzip for large issue collections

### Atomic Operations
- Use temporary files with atomic rename for consistency
- File locking to prevent concurrent modification conflicts
- Backup creation before destructive operations
- Transaction-like semantics for multi-file operations

---

**Data Model Complete**: All entities defined with validation rules and relationships. Ready for contract generation.