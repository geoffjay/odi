# Architecture Overview

This document provides a comprehensive overview of ODI's technical architecture, design decisions, and implementation details for developers and advanced users.

## System Architecture

ODI follows a modular, layered architecture built in Rust for performance, safety, and cross-platform compatibility.

### High-Level Architecture

```
┌─────────────────────────────────────────────┐
│                CLI Layer                    │
│  odi binary (clap-based command parsing)   │
└─────────────┬───────────────────────────────┘
              │
┌─────────────▼───────────────────────────────┐
│              Integration Layer              │
│     Command handlers, formatters, I/O      │
└─────┬───────────────┬─────────────────┬─────┘
      │               │                 │
┌─────▼─────┐ ┌───────▼───────┐ ┌───────▼──────┐
│ odi-core  │ │    odi-fs     │ │   odi-net    │
│  Domain   │ │  Filesystem   │ │  Network     │
│  Logic    │ │  Operations   │ │  Protocols   │
└───────────┘ └───────────────┘ └──────────────┘
```

### Crate Structure

ODI is implemented as a Rust workspace with multiple crates:

#### `odi/` - Binary Crate
- **Purpose**: CLI interface and user interaction
- **Dependencies**: clap, tokio, odi-core, odi-fs, odi-net
- **Responsibilities**:
  - Command parsing and validation
  - User interface and output formatting
  - Orchestrating operations across library crates
  - Error handling and user feedback

#### `odi-core/` - Domain Logic Crate
- **Purpose**: Core business logic and domain models
- **Dependencies**: serde, chrono, uuid, thiserror
- **Responsibilities**:
  - Issue, Project, User, Team entity definitions
  - Business rules and validation logic
  - Synchronization engine interfaces
  - Domain-specific error handling

#### `odi-fs/` - Filesystem Operations Crate  
- **Purpose**: Local storage and configuration management
- **Dependencies**: serde, toml, sha2, flate2
- **Responsibilities**:
  - Binary object storage (Git-like)
  - Configuration loading and validation
  - File system abstractions
  - Local data persistence

#### `odi-net/` - Network Operations Crate
- **Purpose**: Remote communication and synchronization
- **Dependencies**: reqwest, tokio, async-trait
- **Responsibilities**:
  - Protocol implementations (HTTPS, SSH)
  - Remote repository operations
  - Network error handling and retries
  - Authentication management

#### `tests/` - Integration Testing
- **Purpose**: Cross-crate integration and contract testing
- **Types**:
  - Contract tests: Validate interfaces between crates
  - Integration tests: End-to-end workflow testing
  - Performance tests: Benchmark critical operations

## Core Domain Model

### Entity Relationships

```
┌─────────────┐    ┌──────────────┐    ┌─────────────┐
│  Workspace  │◄──►│   Project    │◄──►│   Remote    │
│             │    │              │    │             │
└─────┬───────┘    └──────┬───────┘    └─────────────┘
      │                   │
      │ 1:N               │ 1:N
      ▼                   ▼
┌─────────────┐    ┌──────────────┐    ┌─────────────┐
│    Issue    │    │    Label     │    │    Team     │
│             │◄──►│              │◄──►│             │
└─────┬───────┘    └──────────────┘    └─────┬───────┘
      │                                      │
      │ N:M                                  │ N:M  
      ▼                                      ▼
┌─────────────┐                      ┌─────────────┐
│    User     │◄────────────────────►│TeamMembership│
│             │                      │             │
└─────────────┘                      └─────────────┘
```

### Key Design Decisions

#### Many-to-Many Project Relationships
Unlike traditional issue trackers, ODI supports:
- **Multiple projects per workspace**: A single working directory can manage multiple projects
- **Projects across workspaces**: The same project can exist in multiple working directories
- **Flexible project assignment**: Issues can move between projects easily

#### Binary Object Storage
Following Git's model, ODI stores data as binary objects:

```
.odi/objects/
├── issues/
│   ├── ab/
│   │   └── cdef1234... (compressed binary issue data)
│   └── 01/
│       └── 2345abcd... (compressed binary issue data)
├── projects/  
├── teams/
└── users/
```

**Benefits:**
- **Efficient storage**: Binary compression reduces disk usage
- **Integrity**: SHA-256 hashes ensure data integrity  
- **Performance**: Fast random access by object ID
- **Git compatibility**: Similar storage model enables integration

#### Single Configuration File
All configuration lives in `.odi/config` as TOML:

```toml
[user]
name = "John Doe"
email = "john@example.com"

[project]
name = "Main Project"
description = "Primary project"

[remotes.origin]  
url = "https://github.com/user/project-issues.git"

[sync]
auto_pull = true
conflict_strategy = "prompt"
```

This simplifies configuration management compared to multiple files.

## Storage Engine

### Object Storage Design

ODI implements a content-addressable storage system similar to Git:

```rust
// Object identification by hash
type ObjectId = [u8; 32]; // SHA-256 hash

// Generic object storage
trait ObjectStorage {
    fn store<T: Serialize>(&self, obj: &T) -> Result<ObjectId>;
    fn load<T: DeserializeOwned>(&self, id: ObjectId) -> Result<T>;
    fn exists(&self, id: ObjectId) -> bool;
    fn delete(&self, id: ObjectId) -> Result<()>;
}
```

### Storage Layout

```
.odi/
├── config                 # TOML configuration
├── objects/              # Content-addressable storage
│   ├── issues/           # Issue objects
│   ├── projects/         # Project objects  
│   ├── users/            # User objects
│   ├── teams/            # Team objects
│   └── metadata/         # Workspace metadata
├── refs/                 # Reference tracking
│   ├── heads/            # Local references
│   └── remotes/          # Remote tracking references  
├── index                 # Fast lookup index
└── locks/                # Concurrent access control
```

### Object Serialization

Objects are stored in a binary format using efficient serialization:

1. **Serialize** object to bytes (using `bincode`)
2. **Compress** with `flate2` (gzip compression)  
3. **Hash** compressed data with SHA-256
4. **Store** in `objects/{type}/{hash[0:2]}/{hash[2:]}`

```rust
impl FileSystemStorage {
    fn store_object<T: Serialize>(&self, obj: &T) -> Result<ObjectId> {
        // Serialize to binary
        let bytes = bincode::serialize(obj)?;
        
        // Compress
        let compressed = compress_gzip(&bytes)?;
        
        // Generate hash  
        let hash = sha256(&compressed);
        
        // Store with hash-based path
        let path = self.object_path(&hash);
        std::fs::write(path, compressed)?;
        
        Ok(hash)
    }
}
```

### Index and Lookups

For fast queries, ODI maintains an in-memory index:

```rust
struct WorkspaceIndex {
    // Fast lookups
    issues_by_project: HashMap<ProjectId, Vec<IssueId>>,
    issues_by_assignee: HashMap<UserId, Vec<IssueId>>,
    issues_by_label: HashMap<LabelId, Vec<IssueId>>,
    issues_by_status: HashMap<IssueStatus, Vec<IssueId>>,
    
    // Reverse mappings
    issue_to_object: HashMap<IssueId, ObjectId>,
    project_to_object: HashMap<ProjectId, ObjectId>,
    
    // Metadata
    last_updated: SystemTime,
}
```

## Synchronization Engine

### Distributed Sync Model

ODI's synchronization follows a distributed model without requiring a central authority:

```rust
trait SyncEngine {
    async fn push(&self, remote: &Remote) -> SyncResult;
    async fn pull(&self, remote: &Remote) -> SyncResult;  
    async fn merge(&self, changes: RemoteChanges) -> MergeResult;
    async fn resolve_conflicts(&self, conflicts: Vec<Conflict>) -> ResolutionResult;
}
```

### Change Tracking

Changes are tracked using a commit-like model:

```rust
#[derive(Serialize, Deserialize)]
struct ChangeSet {
    id: Uuid,
    parent: Option<Uuid>,
    timestamp: DateTime<Utc>, 
    author: UserId,
    changes: Vec<Change>,
    message: String,
}

#[derive(Serialize, Deserialize)]  
enum Change {
    IssueCreated { id: IssueId, object_id: ObjectId },
    IssueModified { id: IssueId, old: ObjectId, new: ObjectId },
    IssueDeleted { id: IssueId, object_id: ObjectId },
    ProjectCreated { id: ProjectId, object_id: ObjectId },
    // ... other change types
}
```

### Conflict Resolution

Conflicts are detected during merge operations:

```rust
#[derive(Debug)]
enum ConflictType {
    // Same object modified differently
    ContentConflict {
        local: ObjectId,
        remote: ObjectId,
        base: Option<ObjectId>,
    },
    
    // Object deleted locally but modified remotely  
    DeleteModifyConflict {
        deleted_locally: bool,
        modified_version: ObjectId,
    },
    
    // Structural conflicts
    StructuralConflict {
        description: String,
        local_state: Value,
        remote_state: Value,
    },
}
```

## Network Layer

### Protocol Abstraction

ODI supports multiple network protocols through a common interface:

```rust
#[async_trait]
trait Protocol {
    async fn push(&self, remote: &Remote, data: &[u8]) -> Result<()>;
    async fn pull(&self, remote: &Remote) -> Result<Vec<u8>>;
    async fn list_refs(&self, remote: &Remote) -> Result<Vec<RemoteRef>>;
    fn supports_auth(&self) -> Vec<AuthMethod>;
}

// Implementations
struct HttpsProtocol;
struct SshProtocol; 
struct S3Protocol; // Via plugin
```

### Authentication

Authentication is handled per-protocol:

```rust
enum AuthMethod {
    Token { token: String },
    Password { username: String, password: String },
    SshKey { key_path: PathBuf, passphrase: Option<String> },
    None,
}
```

### Network Resilience

The network layer includes resilience features:

- **Automatic retries** with exponential backoff
- **Connection pooling** for HTTPS requests
- **Timeout handling** with configurable limits
- **Bandwidth limiting** for large operations
- **Progress reporting** for long-running transfers

## Command Architecture  

### Command Pattern Implementation

Each ODI command is implemented using the Command pattern:

```rust
#[async_trait]
trait Command {
    async fn execute(&self) -> Result<()>;
    fn validate(&self) -> Result<()>;
}

// Example implementation
#[derive(Args)]
struct IssueCreateArgs {
    title: String,
    #[arg(long)]
    description: Option<String>,
    #[arg(long)] 
    priority: Option<Priority>,
}

#[async_trait]
impl Command for IssueCreateArgs {
    async fn execute(&self) -> Result<()> {
        // Load workspace
        let workspace = Workspace::current()?;
        
        // Create issue
        let mut issue = Issue::new(self.title.clone(), workspace.current_user()?);
        
        if let Some(desc) = &self.description {
            issue.description = Some(desc.clone());
        }
        
        // Store issue  
        let storage = workspace.storage();
        let issue_id = storage.store_issue(&issue)?;
        
        println!("Created issue: {}", issue_id);
        Ok(())
    }
    
    fn validate(&self) -> Result<()> {
        if self.title.is_empty() {
            return Err("Issue title cannot be empty".into());
        }
        Ok(())
    }
}
```

### Output Formatting

ODI supports multiple output formats through a formatting system:

```rust
trait Formatter {
    fn format_issue(&self, issue: &Issue) -> String;
    fn format_issue_list(&self, issues: &[Issue]) -> String;
    fn format_project(&self, project: &Project) -> String;
}

struct TableFormatter;
struct JsonFormatter;
struct CsvFormatter;

// Usage in commands
let formatter: Box<dyn Formatter> = match format {
    OutputFormat::Table => Box::new(TableFormatter),
    OutputFormat::Json => Box::new(JsonFormatter), 
    OutputFormat::Csv => Box::new(CsvFormatter),
};

println!("{}", formatter.format_issue_list(&issues));
```

## Error Handling

### Error Types

ODI uses structured error handling with `thiserror`:

```rust
#[derive(thiserror::Error, Debug)]
pub enum OdiError {
    #[error("IO error: {message}")]
    Io { message: String },
    
    #[error("Configuration error: {message}")]
    Config { message: String },
    
    #[error("Storage error: {message}")]  
    Storage { message: String },
    
    #[error("Network error: {message}")]
    Network { message: String },
    
    #[error("Sync conflict: {conflicts:?}")]
    SyncConflict { conflicts: Vec<Conflict> },
    
    #[error("Command error: {message}")]
    Command { message: String },
}
```

### Error Propagation

Errors propagate up the stack with context:

```rust
impl IssueStorage {
    fn load_issue(&self, id: IssueId) -> Result<Issue> {
        let object_id = self.issue_index.get(&id)
            .ok_or_else(|| OdiError::Storage { 
                message: format!("Issue {} not found", id) 
            })?;
            
        self.storage.load_object(*object_id)
            .map_err(|e| OdiError::Storage { 
                message: format!("Failed to load issue {}: {}", id, e) 
            })
    }
}
```

## Testing Strategy

### Unit Tests

Each crate includes comprehensive unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_issue_creation() {
        let issue = Issue::new("Test".to_string(), "author".to_string());
        assert_eq!(issue.title, "Test");
        assert_eq!(issue.status, IssueStatus::Open);
    }
    
    #[test]  
    fn test_status_transition() {
        let mut issue = Issue::new("Test".to_string(), "author".to_string());
        assert!(issue.update_status(IssueStatus::InProgress).is_ok());
        assert_eq!(issue.status, IssueStatus::InProgress);
    }
}
```

### Integration Tests

Cross-crate integration tests ensure proper interaction:

```rust
#[tokio::test]
async fn test_end_to_end_issue_workflow() {
    // Setup test workspace
    let temp_dir = tempfile::tempdir().unwrap();
    let workspace = Workspace::init(temp_dir.path()).await.unwrap();
    
    // Create issue
    let issue = Issue::new("Integration test".to_string(), "tester".to_string());
    let issue_id = workspace.create_issue(issue).await.unwrap();
    
    // Verify storage  
    let loaded = workspace.get_issue(issue_id).await.unwrap();
    assert_eq!(loaded.title, "Integration test");
    
    // Test sync (with mock remote)
    let mock_remote = MockRemote::new();
    workspace.add_remote("origin", mock_remote).await.unwrap();
    workspace.push("origin").await.unwrap();
}
```

### Contract Tests

Contract tests validate interfaces between crates:

```rust
#[test]
fn test_storage_contract() {
    let storage = FileSystemStorage::new_temp().unwrap();
    
    // Test all required operations
    storage_contract_tests(&storage);
}

fn storage_contract_tests<S: ObjectStorage>(storage: &S) {
    // Store and retrieve
    let issue = Issue::new("Contract test".to_string(), "tester".to_string());
    let id = storage.store(&issue).unwrap();
    let loaded: Issue = storage.load(id).unwrap();
    assert_eq!(issue.title, loaded.title);
    
    // Verify existence
    assert!(storage.exists(id));
    
    // Delete  
    storage.delete(id).unwrap();
    assert!(!storage.exists(id));
}
```

## Performance Considerations

### Scalability Design

ODI is designed to handle large-scale usage:

- **Lazy loading**: Objects loaded on-demand
- **Streaming operations**: Large datasets processed incrementally  
- **Index caching**: Fast queries without full scans
- **Parallel processing**: Multi-threaded operations where safe
- **Efficient serialization**: Binary formats minimize I/O

### Memory Management

Rust's ownership system ensures memory safety:

- **Zero-copy operations** where possible
- **Reference counting** for shared data
- **Bounded collections** prevent memory leaks
- **Streaming parsers** for large files

### Performance Monitoring

ODI includes built-in performance monitoring:

```rust
struct PerformanceMetrics {
    command_duration: Duration,
    objects_loaded: usize,
    bytes_transferred: usize,
    cache_hits: usize,
    cache_misses: usize,
}

// Usage
let _timer = perf_timer!("issue_create");
// ... operation ...
// Timer automatically records duration
```

## Extensibility

### Plugin Architecture

ODI supports plugins for extending functionality:

```rust
trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> Version;
    fn initialize(&mut self, context: &PluginContext) -> Result<()>;
}

// Example: Custom protocol plugin
struct S3SyncPlugin;

impl Plugin for S3SyncPlugin {
    fn name(&self) -> &str { "s3-sync" }
    
    fn initialize(&mut self, context: &PluginContext) -> Result<()> {
        context.register_protocol("s3", Box::new(S3Protocol::new()));
        Ok(())
    }
}
```

### Custom Formatters

New output formats can be added:

```rust
struct MarkdownFormatter;

impl Formatter for MarkdownFormatter {
    fn format_issue(&self, issue: &Issue) -> String {
        format!(
            "# {}\n\n**Status:** {}\n**Priority:** {}\n\n{}", 
            issue.title,
            issue.status,
            issue.priority,
            issue.description.as_deref().unwrap_or("")
        )
    }
}
```

### Hook System

ODI provides hooks for custom automation:

```toml
# .odi/config
[hooks]
pre_push = "scripts/validate.sh"
post_issue_create = "scripts/notify.py"
```

## Security Model

### Data Security

- **Local encryption** for sensitive workspaces
- **Content integrity** via SHA-256 hashing
- **Access controls** through filesystem permissions
- **Audit logging** for sensitive operations

### Network Security

- **TLS encryption** for HTTPS communications
- **SSH key authentication** for SSH protocol
- **Certificate validation** for all connections
- **Token rotation** support for long-running deployments

## Future Architecture

### Planned Enhancements

1. **Real-time collaboration**: WebSocket-based live updates
2. **Conflict-free replicated data types (CRDTs)**: Automatic conflict resolution
3. **Distributed consensus**: Byzantine fault tolerance for critical workspaces
4. **Plugin ecosystem**: Official plugin registry and management
5. **Performance optimizations**: Parallel sync, better caching
6. **Mobile support**: Cross-compilation for mobile platforms

### Backward Compatibility

ODI maintains backward compatibility through:

- **Versioned storage formats** with migration paths
- **API versioning** for plugin interfaces  
- **Configuration migration** tools
- **Legacy format support** for older workspaces