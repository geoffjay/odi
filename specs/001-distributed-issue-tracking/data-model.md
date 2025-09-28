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
**Purpose**: Logical container for related issues, independent of working directory location

**Fields**:
- `id`: ProjectId - Unique identifier (project name format)
- `name`: String - Display name (1-100 characters)
- `description`: Option<String> - Project purpose and goals
- `issues`: Vec<IssueId> - Associated issue IDs
- `teams`: Vec<TeamId> - Teams with project access
- `labels`: Vec<Label> - Available labels for issues
- `workspaces`: Vec<WorkspaceId> - Working directories using this project
- `settings`: ProjectSettings - Project-specific configuration
- `created_at`: DateTime<Utc> - Creation timestamp  
- `updated_at`: DateTime<Utc> - Last modification timestamp

**Validation Rules**:
- ID must match regex `^[a-zA-Z0-9_.-]+$` (3-100 characters)
- Name must be non-empty and ≤ 100 characters
- Teams must be valid team IDs
- Must be associated with at least one workspace

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

### Workspace
**Purpose**: Working directory that can contain multiple projects and their remotes

**Fields**:
- `id`: WorkspaceId - Unique identifier (path-based hash)
- `path`: PathBuf - Absolute path to working directory
- `projects`: Vec<ProjectId> - Projects active in this workspace
- `remotes`: Vec<Remote> - Remote repository connections for projects
- `git_repository`: Option<GitRepository> - Associated Git repo
- `created_at`: DateTime<Utc> - Creation timestamp
- `updated_at`: DateTime<Utc> - Last modification timestamp

**Validation Rules**:
- Path must be absolute and accessible
- Projects must be valid project IDs
- Git repository path must be valid if specified
- At least one project must be active

### Remote
**Purpose**: External repository connection with protocol and synchronization state

**Fields**:
- `id`: RemoteId - Unique identifier (remote name)
- `name`: String - Display name (typically 'origin', 'upstream')
- `url`: String - Repository URL (SSH or HTTPS)
- `protocol`: Protocol - Connection type (SSH, HTTPS)
- `auth`: AuthConfig - Authentication configuration
- `projects`: Vec<ProjectId> - Projects synchronized via this remote
- `last_sync`: Option<DateTime<Utc>> - Last successful sync
- `sync_state`: SyncState - Current synchronization status
- `created_at`: DateTime<Utc> - Creation timestamp

**Validation Rules**:
- ID must match regex `^[a-zA-Z0-9_-]+$` (1-50 characters)
- URL must be valid SSH or HTTPS format
- Protocol must match URL scheme
- Auth config must be valid for protocol type
- Must be associated with at least one project

### Config
**Purpose**: Unified configuration file with all settings sections

**TOML Structure**:
```toml
[user]
name = "John Developer"  
email = "john@example.com"
ssh_key = "~/.ssh/id_rsa"

[workspace]
active_projects = ["project1", "project2"]
default_assignee = "@john"

[project.project1] 
name = "Main Project"
default_labels = ["bug", "feature"]
git_integration = true

[project.project2]
name = "Documentation"  
default_labels = ["docs"]
git_integration = false

[remote.origin]
url = "https://issues.example.com/repo.git"
protocol = "https"
projects = ["project1", "project2"]

[ui]
color = "auto"
pager = true
editor = "vim"

[sync]
auto_pull = false
conflict_strategy = "manual"
compress_objects = true
```

**Validation Rules**:
- Required: user.name, user.email
- Projects referenced in workspace.active_projects must exist in [project.*]
- Remotes referenced must have valid URLs and protocols
- All file paths must be accessible

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
- **Project** ←→ **Workspace**: Many-to-many (projects can exist in multiple workspaces)
- **Workspace** ←→ **Remote**: One-to-many (workspace has multiple remotes)
- **Remote** ←→ **Project**: Many-to-many (remote can sync multiple projects)
- **Issue** ←→ **User**: Many-to-many (assignees), One-to-many (author)
- **Issue** ←→ **Label**: Many-to-many (issues have multiple labels)

### Data Dependencies
- Issues depend on Users (author, assignees) and Projects
- Teams depend on Users (members) and Projects (access)
- Workspaces depend on Projects (active projects) and Remotes (connections)
- Remotes depend on Projects (synchronization scope) and Auth (credentials)
- Labels depend on Projects (scoped definition)

## Storage Implementation

### File Structure (Git-like Object Store)
```
.odi/
├── config                # Single TOML configuration file (no extension)
├── objects/              # Content-addressed object store
│   ├── {hash[0:2]}/      # First 2 chars of hash as directory
│   │   └── {hash[2:]}    # Remaining hash as filename
│   └── pack/             # Packed object files (future optimization)
├── refs/                 # Reference storage
│   ├── issues/           # Issue references
│   ├── projects/         # Project references  
│   └── remotes/          # Remote references
├── HEAD                  # Current workspace state
└── locks/                # File locking for concurrent access
```

### Object Storage Format
- **Binary Format**: All objects stored as compressed binary data
- **Content Addressing**: SHA-256 hash of object content as identifier  
- **Object Types**: Issue, User, Team, Project, Workspace, Remote, Label
- **Compression**: zlib compression for all stored objects
- **Integrity**: Hash verification on read operations

### Atomic Operations
- Use temporary files with atomic rename for consistency
- File locking to prevent concurrent modification conflicts
- Backup creation before destructive operations
- Transaction-like semantics for multi-file operations

---

**Data Model Complete**: All entities defined with validation rules and relationships. Ready for contract generation.