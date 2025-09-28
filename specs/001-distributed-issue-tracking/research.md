# Phase 0: Research & Architecture Decisions

**Prerequisites**: plan.md technical context analysis
**Scope**: Resolve architecture decisions for Rust workspace CLI application

## Research Tasks Completed

### 1. Rust Workspace Best Practices for CLI Applications

**Decision**: Multi-crate workspace with binary + library separation
**Rationale**: 
- Enables code reuse across multiple binaries and external consumers
- Provides clear separation of concerns (CLI logic vs domain logic vs infrastructure)  
- Supports incremental compilation and testing of individual components
- Follows Rust ecosystem patterns used by Git, Cargo, and other CLI tools

**Alternatives Considered**:
- Single crate with modules: Rejected due to poor extensibility for external developers
- Separate repositories: Rejected due to version synchronization complexity

### 2. Clap CLI Design Patterns and Git-like UX

**Decision**: Subcommand-based architecture with clap derive macros
**Rationale**:
- Git-familiar command structure (`odi init`, `odi issue create`, `odi remote add`)
- Type-safe argument parsing with compile-time validation
- Automatic help generation and shell completion support
- Consistent error messages and validation patterns

**Alternatives Considered**:
- Builder pattern clap: Rejected due to verbosity and maintenance overhead
- Custom argument parsing: Rejected due to reinventing well-tested solutions

### 3. Distributed System Conflict Resolution Strategies

**Decision**: Three-way merge with manual conflict resolution (Git-like)
**Rationale**:
- Users already familiar with Git conflict resolution workflows
- Preserves data integrity by requiring explicit user decisions
- Supports complex conflict scenarios without data loss
- Enables offline-first operation with eventual consistency

**Alternatives Considered**:
- Automatic merge strategies: Rejected due to potential data loss in edge cases
- Last-writer-wins: Rejected due to loss of user work and poor collaboration experience
- Operational transforms: Rejected due to implementation complexity for this use case

### 4. TOML Configuration Hierarchy in Rust

**Decision**: Layered configuration with serde and config crate
**Rationale**:
- Native TOML support with type-safe deserialization
- Clear precedence rules: CLI args → local config → global config → defaults
- Validation at load time with helpful error messages
- Environment variable override support for CI/deployment scenarios

**Alternatives Considered**:
- JSON configuration: Rejected due to lack of comments and less human-friendly syntax
- YAML configuration: Rejected due to indentation sensitivity and security concerns
- Custom format: Rejected due to user learning curve and tooling limitations

### 5. Filesystem Storage Formats for Structured Data

**Decision**: Git-like object store with content-addressed binary storage
**Rationale**:
- Content-addressed storage using SHA-256 hashes ensures data integrity
- Binary format with compression optimizes storage efficiency and performance
- Git-like structure provides familiar patterns for distributed version control
- Object deduplication across multiple projects reduces storage overhead
- Atomic operations with hash verification prevent corruption

**Alternatives Considered**:
- JSON files in directories: Rejected due to storage inefficiency and lack of integrity checking
- SQLite embedded database: Rejected due to dependency complexity and file locking issues across distributed systems
- TOML for all data: Rejected due to performance issues with large datasets and binary content limitations

### 6. Configuration Management Strategy

**Decision**: Single unified TOML configuration file (`.odi/config`)
**Rationale**:
- Simplified configuration management with single source of truth
- TOML sections provide clear organization (user, workspace, project.*, remote.*)
- No file extension follows Git conventions (.git/config, not .git/config.toml)
- Hierarchical structure supports multiple projects in single workspace
- Easy to edit, version control, and merge across different environments

**Alternatives Considered**:
- Multiple specialized config files: Rejected due to complexity and synchronization issues
- JSON configuration: Rejected due to lack of comments and less human-friendly syntax
- Environment variables only: Rejected due to poor organization for complex nested configurations

## Architecture Decisions Summary

### Crate Structure
```
odi/                    # Binary crate - CLI entry point and command routing
odi-core/              # Domain logic - entities, business rules, validation  
odi-fs/                # Filesystem operations - storage, config, Git integration
odi-net/               # Network operations - protocols, sync, authentication
```

### Data Storage Strategy
```
.odi/
├── config             # Unified TOML configuration (no extension)
├── objects/           # Content-addressed object store
│   ├── {hash[0:2]}/   # First 2 chars as directory (Git pattern)
│   │   └── {hash[2:]} # Remaining hash as compressed binary object
│   └── pack/          # Packed objects (future optimization)
├── refs/              # Reference pointers to objects
│   ├── issues/        # Issue ID → object hash mappings
│   ├── projects/      # Project ID → object hash mappings
│   └── remotes/       # Remote sync state
├── HEAD               # Current workspace state pointer
└── locks/             # Concurrent access control
```

### CLI Command Architecture
```
odi
├── init              # Initialize new ODI repository
├── issue             # Issue management subcommands
│   ├── create        # Create new issue
│   ├── assign        # Assign issue to user/team
│   ├── label         # Add/remove labels
│   └── list          # List and filter issues
├── remote            # Remote repository management
│   ├── add           # Add remote repository
│   ├── push          # Push changes to remote
│   └── pull          # Pull changes from remote
├── team              # Team management
│   ├── create        # Create new team
│   └── add           # Add user to team
└── config            # Configuration management
    ├── get           # Get configuration value
    └── set           # Set configuration value
```

### Error Handling Strategy
- Result-based error propagation with custom error types
- User-friendly error messages with actionable suggestions
- Detailed error context for debugging without exposing internals
- Consistent exit codes following Unix conventions

### Performance Considerations
- Content-addressed storage enables efficient deduplication across projects
- Binary format with zlib compression minimizes I/O overhead
- Object references allow lazy loading of large datasets
- Incremental synchronization using object hashes minimizes network traffic
- Memory-mapped files for large read operations on packed objects

### Security & Authentication
- SSH key-based authentication for SSH protocol
- OAuth/token-based authentication for HTTPS protocol
- Local credential storage with OS keyring integration
- Permission model based on Git repository access patterns

---

**Research Phase Complete**: All architecture decisions documented with rationale and alternatives. Ready for Phase 1 design and contract generation.