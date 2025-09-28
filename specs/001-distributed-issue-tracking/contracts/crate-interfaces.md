# Crate Interface Contracts

**Scope**: API contracts between ODI crates for modular architecture
**Format**: Public API definitions, error types, trait boundaries

## odi-core Crate

### Public API Surface
```rust
// Re-exports for consumers
pub use issue::{Issue, IssueId, IssueStatus, Priority};
pub use user::{User, UserId, Team, TeamId};
pub use project::{Project, ProjectId, Label, LabelId};
pub use sync::{SyncEngine, SyncResult, ConflictResolution};

// Core result type for all operations
pub type Result<T> = std::result::Result<T, CoreError>;

// Error type for domain operations
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Issue not found: {id}")]
    IssueNotFound { id: IssueId },
    
    #[error("User not found: {id}")]
    UserNotFound { id: UserId },
    
    #[error("Invalid issue status transition: {from} -> {to}")]
    InvalidStatusTransition { from: IssueStatus, to: IssueStatus },
    
    #[error("Permission denied: {operation}")]
    PermissionDenied { operation: String },
    
    #[error("Validation failed: {field}: {message}")]
    ValidationError { field: String, message: String },
}
```

### Issue Management Contract
```rust
pub trait IssueRepository {
    async fn create(&self, issue: CreateIssueRequest) -> Result<Issue>;
    async fn get(&self, id: &IssueId) -> Result<Option<Issue>>;
    async fn update(&self, id: &IssueId, updates: UpdateIssueRequest) -> Result<Issue>;
    async fn delete(&self, id: &IssueId) -> Result<()>;
    async fn list(&self, filters: IssueFilters) -> Result<Vec<Issue>>;
}

pub struct CreateIssueRequest {
    pub title: String,
    pub description: Option<String>,
    pub assignees: Vec<UserId>,
    pub labels: Vec<LabelId>,
    pub priority: Priority,
}

pub struct UpdateIssueRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<IssueStatus>,
    pub assignees: Option<Vec<UserId>>,
    pub labels: Option<Vec<LabelId>>,
    pub priority: Option<Priority>,
}

pub struct IssueFilters {
    pub assignee: Option<UserId>,
    pub labels: Vec<LabelId>,
    pub status: Option<IssueStatus>,
    pub limit: Option<usize>,
}
```

### User Management Contract
```rust
pub trait UserRepository {
    async fn create(&self, user: CreateUserRequest) -> Result<User>;
    async fn get(&self, id: &UserId) -> Result<Option<User>>;
    async fn update(&self, id: &UserId, updates: UpdateUserRequest) -> Result<User>;
    async fn delete(&self, id: &UserId) -> Result<()>;
    async fn list(&self) -> Result<Vec<User>>;
}

pub trait TeamRepository {
    async fn create(&self, team: CreateTeamRequest) -> Result<Team>;
    async fn add_member(&self, team_id: &TeamId, user_id: &UserId) -> Result<()>;
    async fn remove_member(&self, team_id: &TeamId, user_id: &UserId) -> Result<()>;
    async fn get(&self, id: &TeamId) -> Result<Option<Team>>;
    async fn list(&self) -> Result<Vec<Team>>;
}
```

### Synchronization Contract
```rust
pub trait SyncEngine {
    async fn pull(&self, remote: &Remote) -> Result<SyncResult>;
    async fn push(&self, remote: &Remote) -> Result<SyncResult>;
    async fn resolve_conflict(&self, conflict: &Conflict, resolution: ConflictResolution) -> Result<()>;
    async fn status(&self) -> Result<SyncStatus>;
}

#[derive(Debug, Clone)]
pub struct SyncResult {
    pub pulled_issues: Vec<IssueId>,
    pub pushed_issues: Vec<IssueId>,
    pub conflicts: Vec<Conflict>,
    pub errors: Vec<SyncError>,
}

#[derive(Debug, Clone)]
pub struct Conflict {
    pub issue_id: IssueId,
    pub local_version: Issue,
    pub remote_version: Issue,
    pub conflict_type: ConflictType,
}

pub enum ConflictResolution {
    AcceptLocal,
    AcceptRemote,
    Manual(Issue), // User-resolved version
}
```

---

## odi-fs Crate

### Public API Surface
```rust
// Re-exports for filesystem operations
pub use storage::{Storage, StorageEngine};
pub use config::{Config, ConfigLoader, ConfigError};
pub use git::{GitIntegration, GitRepository, GitRef};

// Result type for filesystem operations
pub type Result<T> = std::result::Result<T, FsError>;

// Error type for filesystem operations  
#[derive(Debug, thiserror::Error)]
pub enum FsError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("Permission denied: {path}")]
    PermissionDenied { path: String },
    
    #[error("Invalid configuration: {message}")]
    ConfigError { message: String },
    
    #[error("Git error: {message}")]
    GitError { message: String },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
```

### Storage Contract
```rust
pub trait StorageEngine {
    async fn initialize(&self, path: &Path) -> Result<()>;
    
    // Object store operations (Git-like)
    async fn write_object<T: Serialize>(&self, obj: &T) -> Result<ObjectHash>;
    async fn read_object<T: DeserializeOwned>(&self, hash: &ObjectHash) -> Result<Option<T>>;
    async fn delete_object(&self, hash: &ObjectHash) -> Result<()>;
    async fn list_objects(&self, object_type: ObjectType) -> Result<Vec<ObjectHash>>;
    
    // Reference operations
    async fn write_ref(&self, name: &str, hash: &ObjectHash) -> Result<()>;
    async fn read_ref(&self, name: &str) -> Result<Option<ObjectHash>>;
    async fn list_refs(&self, prefix: &str) -> Result<Vec<String>>;
    
    // Configuration operations
    async fn read_config(&self) -> Result<Config>;
    async fn write_config(&self, config: &Config) -> Result<()>;
    
    // Locking operations
    async fn lock(&self, resource: &str) -> Result<Lock>;
    async fn unlock(&self, lock: Lock) -> Result<()>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectHash(String); // SHA-256 hex string

#[derive(Debug, Clone)]
pub enum ObjectType {
    Issue,
    User,
    Team,
    Project,
    Workspace,
    Remote,
    Label,
}

pub struct Lock {
    resource: String,
    acquired_at: DateTime<Utc>,
    lock_file: PathBuf,
}
```

### Configuration Contract
```rust
pub trait ConfigLoader {
    fn load_global() -> Result<Option<Config>>;
    fn load_local(workspace_path: &Path) -> Result<Option<Config>>;
    fn merge(global: Option<Config>, local: Option<Config>) -> Config;
    fn validate(config: &Config) -> Result<()>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub user: UserConfig,
    pub workspace: Option<WorkspaceConfig>,
    pub project: HashMap<String, ProjectConfig>,
    pub remote: HashMap<String, RemoteConfig>,
    pub ui: UiConfig,
    pub sync: SyncConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]  
pub struct UserConfig {
    pub name: String,
    pub email: String,
    pub ssh_key: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub active_projects: Vec<String>,
    pub default_assignee: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub default_labels: Vec<String>,
    pub git_integration: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteConfig {
    pub url: String,
    pub protocol: String,
    pub projects: Vec<String>,
}
```

### Git Integration Contract
```rust
pub trait GitIntegration {
    fn detect_repository(path: &Path) -> Result<Option<GitRepository>>;
    fn get_current_branch(&self) -> Result<Option<String>>;
    fn get_remote_url(&self, remote: &str) -> Result<Option<String>>;
    fn list_commits(&self, branch: &str) -> Result<Vec<GitRef>>;
    fn associate_issue(&self, issue_id: &IssueId, git_ref: &GitRef) -> Result<()>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitRepository {
    pub path: PathBuf,
    pub remotes: HashMap<String, String>,
    pub current_branch: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitRef {
    pub sha: String,
    pub message: Option<String>,
    pub author: Option<String>,
    pub timestamp: DateTime<Utc>,
}
```

---

## odi-net Crate

### Public API Surface
```rust
// Re-exports for network operations
pub use protocols::{Protocol, ProtocolHandler};
pub use sync::{RemoteSync, SyncClient};
pub use auth::{Authentication, Credential};

// Result type for network operations
pub type Result<T> = std::result::Result<T, NetError>;

// Error type for network operations
#[derive(Debug, thiserror::Error)]
pub enum NetError {
    #[error("Connection failed: {url}")]
    ConnectionFailed { url: String },
    
    #[error("Authentication failed: {method}")]
    AuthenticationFailed { method: String },
    
    #[error("Protocol error: {message}")]
    ProtocolError { message: String },
    
    #[error("Timeout: operation took longer than {seconds}s")]
    Timeout { seconds: u64 },
    
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("TLS error: {0}")]
    Tls(String),
}
```

### Remote Synchronization Contract
```rust
pub trait RemoteSync {
    async fn connect(&self, remote: &Remote) -> Result<SyncClient>;
    async fn list_issues(&self, client: &SyncClient) -> Result<Vec<IssueMetadata>>;
    async fn download_issue(&self, client: &SyncClient, id: &IssueId) -> Result<Issue>;
    async fn upload_issue(&self, client: &SyncClient, issue: &Issue) -> Result<()>;
    async fn get_sync_state(&self, client: &SyncClient) -> Result<RemoteSyncState>;
}

pub struct SyncClient {
    protocol: Box<dyn ProtocolHandler>,
    authenticated: bool,
    base_url: String,
}

#[derive(Debug, Clone)]
pub struct IssueMetadata {
    pub id: IssueId,
    pub last_modified: DateTime<Utc>,
    pub checksum: String,
}

#[derive(Debug, Clone)]
pub struct RemoteSyncState {
    pub last_sync: Option<DateTime<Utc>>,
    pub total_issues: usize,
    pub pending_changes: usize,
}
```

### Protocol Handler Contract
```rust
pub trait ProtocolHandler: Send + Sync {
    async fn authenticate(&self, credential: &Credential) -> Result<AuthToken>;
    async fn get(&self, path: &str, auth: &AuthToken) -> Result<Vec<u8>>;
    async fn post(&self, path: &str, data: &[u8], auth: &AuthToken) -> Result<Vec<u8>>;
    async fn put(&self, path: &str, data: &[u8], auth: &AuthToken) -> Result<Vec<u8>>;
    async fn delete(&self, path: &str, auth: &AuthToken) -> Result<()>;
}

pub struct AuthToken {
    token: String,
    expires_at: Option<DateTime<Utc>>,
    refresh_token: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Credential {
    SshKey { path: PathBuf, passphrase: Option<String> },
    Token { value: String },
    OAuth { client_id: String, refresh_token: String },
}
```

### Authentication Contract
```rust
pub trait Authentication {
    async fn validate_credential(&self, credential: &Credential) -> Result<bool>;
    async fn refresh_token(&self, auth: &AuthToken) -> Result<AuthToken>;
    async fn revoke_token(&self, auth: &AuthToken) -> Result<()>;
    
    fn load_credential(&self, remote: &Remote) -> Result<Option<Credential>>;
    fn store_credential(&self, remote: &Remote, credential: &Credential) -> Result<()>;
    fn remove_credential(&self, remote: &Remote) -> Result<()>;
}
```

---

## Cross-Crate Dependencies

### Dependency Graph
```
odi (binary)
├── odi-core (domain logic)
├── odi-fs (filesystem operations)  
└── odi-net (network operations)

odi-fs
└── odi-core (entities and types)

odi-net  
└── odi-core (entities and types)
```

### Shared Types
- All crates depend on `odi-core` for shared entities
- Error types are crate-specific but convertible
- Async traits used throughout for future async CLI support
- Serde traits derived for all data structures

### Testing Contracts
- Each crate provides mock implementations for testing
- Contract tests validate cross-crate boundaries
- Integration tests use real implementations

---

**Crate Interface Contracts Complete**: All public APIs defined with error handling and trait boundaries for modular architecture.