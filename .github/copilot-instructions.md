# ODI Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-09-28

## Active Technologies

**Language/Version**: Rust 1.75+ (stable channel)
**Primary Dependencies**: clap (CLI), serde (serialization), toml (configuration), tokio (async runtime)
**Storage**: Local filesystem with structured data files (.odi directory)
**Testing**: cargo test with integration and contract test suites
**Target Platform**: Cross-platform (Linux, macOS, Windows) CLI application

## Project Structure
```
Cargo.toml               # Workspace root configuration
odi/
├── Cargo.toml          # Binary crate configuration  
├── src/
│   ├── main.rs         # CLI entry point
│   ├── cli/            # Command definitions and parsing
│   ├── commands/       # Command implementations
│   └── lib.rs          # Binary crate library

odi-core/
├── Cargo.toml          # Core domain logic
└── src/
    ├── lib.rs          # Public API
    ├── issue/          # Issue entity and operations
    ├── project/        # Project management
    ├── user/           # User and team management
    └── sync/           # Distributed synchronization

odi-fs/
├── Cargo.toml          # Filesystem operations crate
└── src/
    ├── lib.rs          # Public filesystem API
    ├── storage/        # Local storage implementation
    ├── config/         # Configuration management
    └── git/            # Git integration

odi-net/
├── Cargo.toml          # Network operations crate
└── src/
    ├── lib.rs          # Public network API
    ├── protocols/      # SSH/HTTPS protocol handlers
    ├── sync/           # Remote synchronization
    └── auth/           # Authentication handling

tests/
├── contract/           # Cross-crate contract tests
├── integration/        # Full system integration tests
└── fixtures/           # Test data and utilities
```

## Commands

**Development**:
```bash
cargo build --workspace                    # Build all crates
cargo test --workspace                     # Run all tests
cargo clippy --workspace -- -D warnings    # Lint code
cargo fmt --all                           # Format code
```

**Testing**:
```bash
cargo test --lib                          # Unit tests only
cargo test --test integration             # Integration tests only  
cargo test contract                       # Contract tests only
cargo test --release                      # Performance tests
```

**Binary**:
```bash
cargo run --bin odi -- init               # Run CLI command
cargo install --path odi                  # Install binary locally
cargo build --release --bin odi           # Release build
```

## Code Style

**Rust Conventions**:
- Use `cargo fmt` and `cargo clippy` for formatting and linting
- Follow Rust API Guidelines for public interfaces
- Use `thiserror` for error types with descriptive messages
- Implement `Debug`, `Clone`, `Serialize`, `Deserialize` for data types
- Use `async/await` for I/O operations with `tokio` runtime
- Prefer `Result<T>` over panics for error handling
- Use type aliases for complex generic types (e.g., `type Result<T> = std::result::Result<T, Error>`)

**Architecture Patterns**:
- Repository pattern for data access (traits with implementations)
- Command pattern for CLI subcommands with clap derive macros
- Builder pattern for complex configuration objects
- Error propagation with `?` operator and custom error types

## Recent Changes

1. **001-distributed-issue-tracking**: Added comprehensive specification for Git-like distributed issue tracking system with Rust workspace architecture, CLI interface using clap, and modular crate design for extensibility.

<!-- MANUAL ADDITIONS START -->
<!-- Add any manual customizations for GitHub Copilot here -->
<!-- MANUAL ADDITIONS END -->