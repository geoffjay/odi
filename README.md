# ODI - Organized Distributed Issues

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

ODI (Organized Distributed Issues) is a Git-like distributed issue tracking system that brings version control concepts to issue management. Like Git manages source code history, ODI manages issue history with complete offline functionality, distributed synchronization, and conflict resolution.

## 🚀 Quick Start

```bash
# Initialize ODI workspace
odi init --project my-app

# Create your first issue
odi issue create "Fix login validation"

# List all issues
odi issue list

# Set up your identity
odi config set user.name "Your Name"
odi config set user.email "you@example.com"

# Add a remote repository
odi remote add origin https://github.com/username/repo-issues.git

# Push to remote
odi push origin
```

## 🎯 Key Features

- **Distributed Architecture**: Work offline, sync when connected
- **Git Integration**: Automatically detects and integrates with Git repositories
- **Team Collaboration**: Advanced user and team management
- **Project Organization**: Multiple projects per workspace with many-to-many relationships
- **Binary Storage**: Efficient object storage similar to Git's approach
- **Conflict Resolution**: Automatic merging with manual conflict resolution when needed
- **Cross-Platform**: Runs on Linux, macOS, and Windows

## 📁 Core Concepts

### Workspaces

An ODI workspace is initialized in any directory and manages issues, projects, and teams for that location. Workspaces automatically detect Git repositories and integrate with them.

### Projects

Projects are organizational units that can span multiple workspaces. Issues belong to projects, and projects can have multiple remotes for distributed collaboration.

### Issues

Issues are the core tracking unit with:

- **States**: Open, In Progress, Resolved, Closed
- **Priority**: Low, Medium, High, Critical
- **Assignees**: Multiple users can be assigned
- **Labels**: Flexible tagging system
- **Git Integration**: Link to commits, branches, and PRs

### Teams

Teams group users and can be assigned to issues collectively. Teams support nested hierarchies and role-based permissions.

## 🛠️ Installation

### From Source

```bash
git clone https://github.com/your-org/odi.git
cd odi
cargo install --path odi
```

### Pre-built Binaries

Download from [Releases](https://github.com/your-org/odi/releases)

## 📖 Documentation

- **[Getting Started Guide](docs/getting-started.md)** - Complete setup and first steps
- **[Command Reference](docs/commands.md)** - Detailed command documentation
- **[Configuration Guide](docs/configuration.md)** - Configuration options and formats
- **[Synchronization Guide](docs/sync.md)** - Remote repositories and conflict resolution
- **[Developer Guide](docs/development.md)** - Contributing and extending ODI
- **[Architecture Overview](docs/architecture.md)** - Technical design and implementation

## 🏗️ Architecture

ODI is built with a modular Rust architecture:

```
odi/                    # CLI binary crate
odi-core/              # Core domain logic
odi-fs/                # Filesystem operations
odi-net/               # Network protocols
tests/                 # Integration tests
```

Key technologies:

- **Rust 1.75+** - Memory safety and performance
- **Clap** - CLI parsing and user experience
- **Tokio** - Async runtime for network operations
- **TOML** - Human-readable configuration
- **Binary Objects** - Efficient storage like Git

## 🧪 Testing

ODI includes comprehensive testing infrastructure including Docker containers for testing remote operations.

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run specific test suites
cargo test --lib                    # Unit tests
cargo test --test integration       # Integration tests
cargo test contract                 # Contract tests
```

### Testing Remote Operations

ODI provides Docker containers for testing SSH and HTTPS remote operations:

```bash
# Start test containers
docker compose up -d

# Set up test repositories
./scripts/test-remote-setup.sh

# Run remote operation tests
cargo test --test integration -- remote

# Clean up
./scripts/test-remote-cleanup.sh
```

The test environment provides:

- **SSH Server** (port 2222): Test `ssh://` protocol operations
- **HTTPS Server** (ports 8080/8443): Test `https://` and `http://` protocol operations
- **Git Server** (port 9418): Reference implementation for comparison

See [.docker/README.md](.docker/README.md) for detailed testing instructions.

## 🤝 Contributing

We welcome contributions! See our [Contributing Guide](docs/contributing.md) for details.

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes and add tests
4. Run the test suite: `cargo test --workspace`
5. Test remote operations with Docker containers
6. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆚 Comparison

| Feature         | ODI       | GitHub Issues  | Jira           | Linear         |
| --------------- | --------- | -------------- | -------------- | -------------- |
| Offline Work    | ✅ Full   | ❌ No          | ❌ No          | ❌ No          |
| Distributed     | ✅ Yes    | ❌ Centralized | ❌ Centralized | ❌ Centralized |
| Git Integration | ✅ Native | ✅ Good        | ⚠️ Limited     | ⚠️ Limited     |
| Self-Hosted     | ✅ Always | ⚠️ Enterprise  | ⚠️ Complex     | ❌ No          |
| CLI First       | ✅ Yes    | ⚠️ Limited     | ❌ No          | ❌ No          |
| Open Source     | ✅ MIT    | ❌ Proprietary | ❌ Proprietary | ❌ Proprietary |

---

**ODI brings the power of distributed version control to issue tracking. Start tracking issues the Git way!**
