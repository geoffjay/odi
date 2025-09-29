# Development Guide

This guide covers contributing to ODI, setting up the development environment, and extending the system with plugins and custom functionality.

## Getting Started with Development

### Prerequisites

- **Rust 1.75+**: Install via [rustup.rs](https://rustup.rs/)
- **Git**: For version control and Git integration features
- **Make** (optional): For build automation
- **Docker** (optional): For integration testing

### Development Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/your-org/odi.git
   cd odi
   ```

2. **Install development tools:**
   ```bash
   # Format checker
   rustup component add rustfmt
   
   # Linter  
   rustup component add clippy
   
   # Code coverage (optional)
   cargo install cargo-tarpaulin
   
   # Documentation generator
   cargo install cargo-docs
   ```

3. **Build the project:**
   ```bash
   cargo build --workspace
   ```

4. **Run tests:**
   ```bash
   cargo test --workspace
   ```

5. **Install locally:**
   ```bash
   cargo install --path odi
   ```

### Development Commands

```bash
# Build all crates
cargo build --workspace

# Build with optimizations
cargo build --release --workspace

# Run specific crate tests
cargo test -p odi-core
cargo test -p odi-fs
cargo test -p odi-net

# Run integration tests
cargo test --test integration

# Run contract tests  
cargo test contract

# Lint code
cargo clippy --workspace -- -D warnings

# Format code
cargo fmt --all

# Generate documentation
cargo doc --workspace --open

# Check for security vulnerabilities
cargo audit

# Measure test coverage
cargo tarpaulin --workspace
```

## Project Structure

Understanding ODI's structure is crucial for effective development:

```
odi/
├── Cargo.toml              # Workspace configuration
├── README.md               # Project overview
├── clippy.toml            # Linting configuration
├── rustfmt.toml           # Formatting configuration
│
├── odi/                   # Main binary crate
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs        # Entry point
│       ├── cli.rs         # Command-line interface
│       ├── commands/      # Command implementations
│       └── lib.rs         # Binary library code
│
├── odi-core/              # Domain logic crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs         # Public API
│       ├── error.rs       # Error definitions
│       ├── issue.rs       # Issue entity and logic
│       ├── project.rs     # Project management
│       ├── user.rs        # User and team management
│       └── sync.rs        # Synchronization engine
│
├── odi-fs/                # Filesystem operations
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs         # Public filesystem API
│       ├── storage.rs     # Object storage implementation
│       ├── config.rs      # Configuration management
│       └── index.rs       # Fast lookup indexing
│
├── odi-net/               # Network operations
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs         # Public network API
│       ├── protocols/     # Protocol implementations
│       │   ├── https.rs   # HTTPS protocol
│       │   └── ssh.rs     # SSH protocol
│       ├── auth.rs        # Authentication handling
│       └── sync.rs        # Remote synchronization
│
├── tests/                 # Integration tests
│   ├── Cargo.toml
│   ├── contract/          # Contract tests between crates
│   ├── integration/       # End-to-end integration tests
│   └── fixtures/          # Test data and utilities
│
├── docs/                  # Documentation
├── scripts/               # Development scripts
└── examples/              # Usage examples
```

## Coding Standards

### Rust Conventions

ODI follows Rust community conventions and additional project-specific guidelines:

#### Code Style
```rust
// Use explicit error handling
fn load_issue(id: IssueId) -> Result<Issue, StorageError> {
    // ... implementation
}

// Prefer ? operator over unwrap/expect in library code
let issue = storage.load_issue(id)?;

// Use descriptive variable names
let resolved_issues_count = issues.iter()
    .filter(|issue| issue.status == IssueStatus::Resolved)
    .count();

// Document public APIs
/// Create a new issue with the specified title and author.
///
/// # Arguments
/// * `title` - Issue title (1-100 characters)
/// * `author` - User ID of the issue creator
///
/// # Returns
/// New issue with default values for optional fields
///
/// # Examples
/// ```
/// let issue = Issue::new("Fix login bug".to_string(), user_id);
/// assert_eq!(issue.status, IssueStatus::Open);
/// ```
pub fn new(title: String, author: UserId) -> Self {
    // ... implementation
}
```

#### Error Handling
```rust
// Use thiserror for error definitions
#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("IO error: {message}")]
    Io { message: String },
    
    #[error("Object not found: {id}")]
    NotFound { id: String },
    
    #[error("Serialization failed")]
    Serialization(#[from] bincode::Error),
}

// Provide context in error propagation
fn save_issue(&self, issue: &Issue) -> Result<ObjectId, StorageError> {
    let bytes = bincode::serialize(issue)
        .map_err(StorageError::Serialization)?;
        
    self.write_object(&bytes)
        .map_err(|e| StorageError::Io { 
            message: format!("Failed to save issue {}: {}", issue.id, e) 
        })
}
```

#### Testing
```rust
// Unit tests in each module
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_issue_validation() {
        // Test valid case
        assert!(Issue::validate_title("Valid title"));
        
        // Test edge cases
        assert!(!Issue::validate_title(""));
        assert!(!Issue::validate_title(&"x".repeat(101)));
    }
    
    #[tokio::test]
    async fn test_async_operation() {
        let storage = MockStorage::new();
        let result = storage.save_issue(&sample_issue()).await;
        assert!(result.is_ok());
    }
}
```

### API Design Principles

#### Consistency
- Use consistent naming across all APIs
- Follow Rust naming conventions (snake_case, PascalCase)
- Maintain consistent error handling patterns

#### Safety
```rust
// Prefer owned types for public APIs
pub fn create_issue(&mut self, issue: Issue) -> Result<IssueId>;

// Use references for read-only operations  
pub fn get_issue(&self, id: IssueId) -> Result<&Issue>;

// Make impossible states unrepresentable
pub enum IssueStatus {
    Open { created_at: DateTime<Utc> },
    InProgress { started_at: DateTime<Utc>, assignee: UserId },
    Resolved { resolved_at: DateTime<Utc>, resolved_by: UserId },
    Closed { closed_at: DateTime<Utc>, reason: CloseReason },
}
```

#### Performance  
```rust
// Use iterators for data processing
pub fn filter_issues(&self, predicate: impl Fn(&Issue) -> bool) -> impl Iterator<Item = &Issue> {
    self.issues.values().filter(predicate)
}

// Provide lazy vs eager alternatives
pub fn list_issues(&self) -> impl Iterator<Item = &Issue>; // Lazy
pub fn collect_issues(&self) -> Vec<Issue>; // Eager
```

## Adding New Features

### 1. Planning

Before implementing new features:

1. **Create a specification** in the `specs/` directory
2. **Discuss the design** in GitHub issues
3. **Consider backward compatibility** implications
4. **Plan the testing strategy**

Example specification structure:
```markdown
# Feature: Advanced Search

## Overview
Implement advanced search capabilities for issues with filtering, sorting, and faceted search.

## Requirements
- [ ] Full-text search in titles and descriptions
- [ ] Filter by multiple criteria simultaneously  
- [ ] Sort by various fields
- [ ] Save and reuse search queries

## API Design
```rust
pub struct SearchQuery {
    pub text: Option<String>,
    pub filters: Vec<Filter>,
    pub sort: Vec<SortCriteria>,
}

pub trait SearchEngine {
    fn search(&self, query: &SearchQuery) -> SearchResult;
}
```

## Implementation Plan
1. Add search API to odi-core
2. Implement filesystem-based search in odi-fs
3. Add CLI commands in odi binary
4. Add comprehensive tests

## Testing Strategy
- Unit tests for search logic
- Integration tests for CLI commands  
- Performance tests with large datasets
```

### 2. Implementation

#### Adding a New Command

1. **Define command arguments:**
   ```rust
   // odi/src/commands/search.rs
   use clap::{Args, Subcommand};
   
   #[derive(Args)]
   pub struct SearchArgs {
       #[command(subcommand)]
       pub command: SearchSubcommand,
   }
   
   #[derive(Subcommand)]
   pub enum SearchSubcommand {
       /// Search issues by text and filters
       Issues {
           /// Search text
           query: Option<String>,
           /// Filter by status
           #[arg(long)]
           status: Option<IssueStatus>,
           /// Filter by assignee
           #[arg(long)]
           assignee: Option<String>,
       },
   }
   ```

2. **Implement command logic:**
   ```rust
   impl SearchArgs {
       pub async fn execute(&self) -> Result<()> {
           match &self.command {
               SearchSubcommand::Issues { query, status, assignee } => {
                   let workspace = Workspace::current()?;
                   let mut search = SearchQuery::new();
                   
                   if let Some(text) = query {
                       search = search.with_text(text);
                   }
                   
                   if let Some(status) = status {
                       search = search.with_status_filter(status);
                   }
                   
                   let results = workspace.search_issues(search).await?;
                   
                   for issue in results {
                       println!("{}: {}", issue.id, issue.title);
                   }
                   
                   Ok(())
               }
           }
       }
   }
   ```

3. **Add to CLI:**
   ```rust
   // odi/src/cli.rs
   #[derive(Subcommand)]
   pub enum Commands {
       // ... existing commands
       /// Advanced search functionality
       Search(SearchArgs),
   }
   
   impl Cli {
       pub async fn execute(&self) -> Result<()> {
           match &self.command {
               // ... existing handlers
               Commands::Search(args) => args.execute().await,
           }
       }
   }
   ```

#### Adding Core Functionality

1. **Define domain types:**
   ```rust
   // odi-core/src/search.rs
   #[derive(Debug, Clone)]
   pub struct SearchQuery {
       text: Option<String>,
       filters: Vec<Filter>,
       sort: Vec<SortCriteria>,
       limit: Option<usize>,
   }
   
   #[derive(Debug, Clone)]  
   pub enum Filter {
       Status(IssueStatus),
       Assignee(UserId),
       Label(LabelId),
       Priority(Priority),
       DateRange { start: DateTime<Utc>, end: DateTime<Utc> },
   }
   
   pub trait SearchEngine {
       fn search_issues(&self, query: &SearchQuery) -> Result<Vec<Issue>>;
   }
   ```

2. **Implement storage integration:**
   ```rust
   // odi-fs/src/search.rs
   impl SearchEngine for FileSystemStorage {
       fn search_issues(&self, query: &SearchQuery) -> Result<Vec<Issue>> {
           let mut results = Vec::new();
           
           // Load all issues (could be optimized with indexing)
           for issue_id in self.list_issue_ids()? {
               let issue = self.load_issue(issue_id)?;
               
               if self.matches_query(&issue, query) {
                   results.push(issue);
               }
           }
           
           // Apply sorting
           self.sort_results(&mut results, &query.sort);
           
           // Apply limit
           if let Some(limit) = query.limit {
               results.truncate(limit);
           }
           
           Ok(results)
       }
   }
   ```

### 3. Testing

#### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_search_query_builder() {
        let query = SearchQuery::new()
            .with_text("login bug")
            .with_status_filter(IssueStatus::Open)
            .with_limit(10);
            
        assert_eq!(query.text, Some("login bug".to_string()));
        assert_eq!(query.limit, Some(10));
    }
    
    #[test]
    fn test_filter_matching() {
        let issue = sample_issue_with_status(IssueStatus::Open);
        let filter = Filter::Status(IssueStatus::Open);
        
        assert!(filter.matches(&issue));
    }
}
```

#### Integration Tests
```rust
// tests/integration/search_test.rs
use odi::workspace::Workspace;

#[tokio::test]
async fn test_issue_search_integration() {
    let workspace = setup_test_workspace().await;
    
    // Create test issues
    let bug_issue = workspace.create_issue(Issue::new(
        "Login bug".to_string(), 
        "alice".to_string()
    )).await.unwrap();
    
    let feature_issue = workspace.create_issue(Issue::new(
        "New feature".to_string(),
        "bob".to_string() 
    )).await.unwrap();
    
    // Test text search
    let query = SearchQuery::new().with_text("bug");
    let results = workspace.search_issues(query).await.unwrap();
    
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, bug_issue);
}
```

## Plugin Development

ODI supports plugins for extending functionality without modifying core code.

### Plugin Architecture

```rust
// Plugin trait definition
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> Version;
    fn description(&self) -> &str;
    
    fn initialize(&mut self, context: &PluginContext) -> Result<(), PluginError>;
    fn shutdown(&mut self) -> Result<(), PluginError>;
}

pub struct PluginContext {
    workspace: Arc<Workspace>,
    config: PluginConfig,
}
```

### Example Plugin: Slack Integration

```rust
// plugins/odi-slack/src/lib.rs
use odi_plugin_api::*;

pub struct SlackPlugin {
    webhook_url: String,
    client: Option<reqwest::Client>,
}

impl Plugin for SlackPlugin {
    fn name(&self) -> &str { "slack-integration" }
    fn version(&self) -> Version { Version::new(1, 0, 0) }
    fn description(&self) -> &str { "Send notifications to Slack" }
    
    fn initialize(&mut self, context: &PluginContext) -> Result<(), PluginError> {
        // Get configuration
        self.webhook_url = context.config.get_string("webhook_url")?;
        self.client = Some(reqwest::Client::new());
        
        // Register event handlers
        context.register_handler("issue.created", Box::new(|event| {
            self.on_issue_created(event)
        }))?;
        
        Ok(())
    }
    
    fn shutdown(&mut self) -> Result<(), PluginError> {
        self.client = None;
        Ok(())
    }
}

impl SlackPlugin {
    async fn on_issue_created(&self, event: &Event) -> Result<(), PluginError> {
        if let Some(issue) = event.as_issue() {
            let message = format!("New issue created: {}", issue.title);
            self.send_slack_message(&message).await?;
        }
        Ok(())
    }
    
    async fn send_slack_message(&self, text: &str) -> Result<(), PluginError> {
        let payload = json!({
            "text": text,
            "username": "ODI Bot",
            "icon_emoji": ":bug:"
        });
        
        if let Some(client) = &self.client {
            client.post(&self.webhook_url)
                .json(&payload)
                .send()
                .await?;
        }
        
        Ok(())
    }
}

// Plugin registration
#[no_mangle]
pub extern "C" fn register_plugin() -> Box<dyn Plugin> {
    Box::new(SlackPlugin {
        webhook_url: String::new(),
        client: None,
    })
}
```

### Plugin Configuration

```toml
# .odi/config
[plugins.slack-integration]
enabled = true
webhook_url = "https://hooks.slack.com/services/..."
channels = ["#development", "#bugs"]

[plugins.jira-sync]
enabled = false
server = "https://company.atlassian.net"
username = "bot@company.com"
```

## Performance Optimization

### Profiling

Use Rust's built-in profiling tools:

```bash
# CPU profiling
cargo build --release
perf record --call-graph=dwarf ./target/release/odi issue list
perf report

# Memory profiling with valgrind
cargo build --release
valgrind --tool=massif ./target/release/odi issue list
```

### Benchmarking

ODI includes benchmarks for performance-critical operations:

```rust
// benches/storage_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use odi_fs::FileSystemStorage;

fn benchmark_issue_creation(c: &mut Criterion) {
    let storage = FileSystemStorage::new_temp().unwrap();
    
    c.bench_function("issue creation", |b| {
        b.iter(|| {
            let issue = sample_issue();
            storage.store_issue(black_box(&issue))
        })
    });
}

fn benchmark_issue_query(c: &mut Criterion) {
    let storage = setup_storage_with_issues(1000);
    
    c.bench_function("issue query", |b| {
        b.iter(|| {
            storage.filter_issues(black_box(|issue| {
                issue.status == IssueStatus::Open
            })).collect::<Vec<_>>()
        })
    });
}

criterion_group!(benches, benchmark_issue_creation, benchmark_issue_query);
criterion_main!(benches);
```

Run benchmarks:
```bash
cargo bench
```

### Memory Optimization

- **Use `Cow<str>`** for strings that might be borrowed or owned
- **Implement `Clone` efficiently** using `Arc` for large shared data
- **Use streaming for large datasets** instead of loading everything into memory
- **Profile memory usage** regularly with tools like `heaptrack`

### I/O Optimization

```rust
// Batch operations for better performance
impl FileSystemStorage {
    pub fn store_issues_batch(&self, issues: &[Issue]) -> Result<Vec<ObjectId>> {
        let mut results = Vec::with_capacity(issues.len());
        
        // Use buffered writer for better I/O performance
        let mut writer = BufWriter::new(File::create(&self.batch_file)?);
        
        for issue in issues {
            let id = self.write_issue_to_buffer(&mut writer, issue)?;
            results.push(id);
        }
        
        writer.flush()?;
        Ok(results)
    }
}
```

## Release Process

### Version Management

ODI uses semantic versioning (SemVer):

- **Major** (1.0.0): Breaking changes
- **Minor** (0.1.0): New features, backward compatible
- **Patch** (0.0.1): Bug fixes, backward compatible

### Release Checklist

1. **Update version numbers** in all `Cargo.toml` files
2. **Update CHANGELOG.md** with new features and fixes
3. **Run full test suite:**
   ```bash
   cargo test --workspace --release
   cargo clippy --workspace -- -D warnings
   cargo fmt --all --check
   ```
4. **Build release binaries:**
   ```bash
   cargo build --release --target x86_64-unknown-linux-gnu
   cargo build --release --target x86_64-pc-windows-gnu
   cargo build --release --target x86_64-apple-darwin
   ```
5. **Create Git tag:**
   ```bash
   git tag -a v1.0.0 -m "Release version 1.0.0"
   git push origin v1.0.0
   ```
6. **Publish to crates.io:**
   ```bash
   cargo publish -p odi-core
   cargo publish -p odi-fs  
   cargo publish -p odi-net
   cargo publish -p odi
   ```

### Continuous Integration

ODI uses GitHub Actions for CI/CD:

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        components: rustfmt, clippy
        
    - name: Format check
      run: cargo fmt --all -- --check
      
    - name: Lint
      run: cargo clippy --workspace -- -D warnings
      
    - name: Test
      run: cargo test --workspace
      
    - name: Integration tests  
      run: cargo test --test integration
```

## Contributing Guidelines

### Code Reviews

All changes require code review:

1. **Create feature branch** from main
2. **Make focused commits** with clear messages
3. **Write tests** for new functionality
4. **Update documentation** if needed
5. **Submit pull request** with detailed description
6. **Address review feedback** promptly

### Commit Messages

Follow conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix  
- `docs`: Documentation changes
- `style`: Formatting changes
- `refactor`: Code refactoring
- `test`: Test additions/changes
- `chore`: Build process or auxiliary tool changes

Examples:
```
feat(sync): add conflict resolution strategies

Add support for automatic conflict resolution using 
configurable strategies (ours, theirs, newer).

Closes #123

fix(storage): handle corrupted object files gracefully

Previously, corrupted objects would cause a panic. Now they 
are detected and reported as storage errors.

docs(api): update search API documentation

Add examples for the new advanced search functionality.
```

### Issue Management

When reporting issues:

1. **Use issue templates** for bugs and feature requests
2. **Provide minimal reproduction** case for bugs
3. **Include version information** and environment details
4. **Search existing issues** before creating new ones
5. **Use clear, descriptive titles**

### Documentation

Keep documentation up-to-date:

- **API documentation** using rustdoc
- **User guides** in `docs/` directory  
- **Code comments** for complex logic
- **Examples** for new features
- **Migration guides** for breaking changes

## Debugging and Troubleshooting

### Logging

Enable debug logging:

```bash
# Set log level
export RUST_LOG=odi=debug

# Run with verbose output
odi --verbose issue list

# Enable specific module logging
export RUST_LOG=odi_fs::storage=trace,odi_net=debug
```

### Debug Builds

Use debug builds for development:

```bash
# Build with debug symbols
cargo build

# Run with debugger
rust-gdb target/debug/odi
(gdb) run issue list
```

### Common Issues

**Build failures:**
```bash
# Clean and rebuild
cargo clean
cargo build --workspace

# Update dependencies
cargo update
```

**Test failures:**
```bash
# Run specific test with output
cargo test test_name -- --nocapture

# Run tests serially (avoid conflicts)
cargo test -- --test-threads=1
```

**Performance issues:**
```bash
# Profile with flamegraph
cargo install flamegraph
cargo flamegraph --bin odi -- issue list
```

This development guide provides a comprehensive foundation for contributing to ODI. As the project evolves, keep this guide updated with new conventions, tools, and best practices.