# Configuration Guide

ODI uses TOML configuration files for workspace and global settings. This guide covers all configuration options and their usage.

## Configuration Hierarchy

ODI loads configuration from multiple sources in this order (highest to lowest priority):

1. **Command-line options** - Override any config setting
2. **Workspace config** - `.odi/config` in the current workspace
3. **Global config** - `~/.odiconfig` in user home directory  
4. **Built-in defaults** - Hardcoded fallback values

## Configuration Files

### Workspace Configuration (`.odi/config`)

Located in the ODI workspace root, this file contains workspace-specific settings:

```toml
# User configuration for this workspace
[user]
name = "John Doe"
email = "john@example.com"

# Project settings
[project]
name = "My Application"
description = "A sample application"
default_branch = "main"

# Default values for new issues
[defaults]
priority = "medium"
auto_assign = true
auto_close_resolved = false

# Synchronization settings
[sync]
auto_push = false
auto_pull = true
conflict_strategy = "prompt"
merge_strategy = "recursive"

# Remote repositories
[remotes.origin]
url = "https://github.com/username/project-issues.git"
protocol = "https"
auth_method = "token"

[remotes.backup]
url = "git@gitlab.com:username/project-issues.git"  
protocol = "ssh"
auth_method = "key"

# Git integration
[git]
enabled = true
repo_path = "/path/to/git/repo"
auto_link_commits = true
commit_message_template = "Addresses: {issue_id}"

# Display preferences
[display]
default_format = "table"
show_colors = true
pager = "less -F"
editor = "vim"

# Issue templates
[templates]
bug = ".odi/templates/bug.md"
feature = ".odi/templates/feature.md"
```

### Global Configuration (`~/.odiconfig`)

Located in the user's home directory, this provides defaults for all ODI workspaces:

```toml
# Global user identity
[user]
name = "John Doe"  
email = "john@example.com"
gpg_key = "ABC123"

# Global defaults
[defaults]
priority = "medium"
auto_assign = false
editor = "code"
pager = "less -F"

# Network settings
[network]
timeout = 30
retries = 3
proxy = "http://proxy.company.com:8080"

# Security settings  
[security]
verify_ssl = true
trusted_hosts = ["github.com", "gitlab.com"]
```

## Configuration Sections

### `[user]` - User Identity

Defines the user identity for authorship and assignment.

```toml
[user]
name = "John Doe"           # Display name
email = "john@example.com"  # Email address
gpg_key = "ABC123"         # GPG key ID for signing (optional)
```

**Command equivalents:**
```bash
odi config set user.name "John Doe"
odi config set user.email "john@example.com"
```

### `[project]` - Project Settings

Workspace-specific project configuration.

```toml
[project]
name = "My Application"                    # Project name
description = "A sample application"       # Project description  
default_branch = "main"                   # Git default branch
auto_create_labels = true                 # Auto-create common labels
issue_number_format = "sequential"        # Issue numbering (sequential, uuid)
```

### `[defaults]` - Default Values

Default values for new issues and operations.

```toml
[defaults]
priority = "medium"              # Default issue priority
status = "open"                  # Default issue status
auto_assign = false              # Auto-assign to creator
auto_close_resolved = false      # Auto-close resolved issues
editor = "vim"                   # Text editor for descriptions
format = "table"                 # Default output format
```

**Valid priority values:** `low`, `medium`, `high`, `critical`
**Valid status values:** `open`, `in-progress`, `resolved`, `closed`

### `[sync]` - Synchronization Settings

Controls how ODI synchronizes with remote repositories.

```toml
[sync]
auto_push = false               # Automatically push after changes
auto_pull = true               # Automatically pull before operations
conflict_strategy = "prompt"    # How to handle conflicts
merge_strategy = "recursive"    # Merge algorithm
timeout = 60                   # Network timeout in seconds
```

**Conflict strategies:**
- `prompt` - Ask user for each conflict
- `auto` - Attempt automatic resolution
- `ours` - Prefer local changes
- `theirs` - Prefer remote changes
- `abort` - Abort on conflicts

**Merge strategies:**
- `recursive` - Git-like recursive merge
- `ours` - Keep all local changes
- `theirs` - Accept all remote changes

### `[remotes.<name>]` - Remote Repositories

Define remote repositories for synchronization.

```toml
[remotes.origin]
url = "https://github.com/username/project-issues.git"
protocol = "https"              # https, ssh, or auto-detect
auth_method = "token"           # token, password, key
branch = "main"                 # Remote branch
timeout = 30                    # Connection timeout

[remotes.backup]
url = "git@gitlab.com:username/project-issues.git"
protocol = "ssh"
auth_method = "key"
key_file = "~/.ssh/id_rsa"     # SSH key file
```

**Authentication methods:**
- `token` - GitHub/GitLab personal access token
- `password` - Username/password authentication
- `key` - SSH key authentication
- `auto` - Auto-detect based on URL

### `[git]` - Git Integration

Configure integration with Git repositories.

```toml
[git]
enabled = true                              # Enable Git integration
repo_path = "/path/to/git/repo"            # Git repository path
auto_link_commits = true                   # Link commits to issues
commit_message_template = "Fixes: {issue_id}"  # Template for commits
branch_naming = "issue-{issue_id}"         # Branch naming pattern
```

**Template variables:**
- `{issue_id}` - Issue identifier
- `{issue_title}` - Issue title (sanitized)
- `{project}` - Project name
- `{priority}` - Issue priority

### `[display]` - Display Preferences

Control output formatting and appearance.

```toml
[display]
default_format = "table"        # Default output format
show_colors = true              # Enable colored output
pager = "less -F"              # Pager for long output
editor = "vim"                 # Default text editor
date_format = "%Y-%m-%d %H:%M" # Date display format
```

**Output formats:**
- `table` - Human-readable table
- `json` - JSON format
- `csv` - Comma-separated values
- `yaml` - YAML format

### `[templates]` - Issue Templates

Define paths to issue templates for different types.

```toml
[templates]
bug = ".odi/templates/bug.md"
feature = ".odi/templates/feature.md"
task = ".odi/templates/task.md"
```

Template files can use variables:

```markdown
# Bug Report

**Issue ID:** {issue_id}
**Reporter:** {author}
**Date:** {created_at}

## Description
{description}

## Steps to Reproduce
1. 
2. 
3. 

## Expected Behavior

## Actual Behavior

## Environment
- OS: 
- Browser: 
- Version: 
```

### `[network]` - Network Settings

Configure network behavior for remote operations.

```toml
[network]
timeout = 30                    # Connection timeout in seconds
retries = 3                    # Number of retry attempts
retry_delay = 5                # Delay between retries
proxy = "http://proxy:8080"    # HTTP proxy (optional)
user_agent = "ODI/1.0"        # Custom User-Agent header
```

### `[security]` - Security Settings

Security and trust configuration.

```toml
[security]
verify_ssl = true                              # Verify SSL certificates
trusted_hosts = ["github.com", "gitlab.com"]  # Trusted hostnames
max_file_size = "10MB"                        # Maximum file size
allow_shell = false                           # Allow shell commands in hooks
```

## Environment Variable Overrides

Configuration values can be overridden using environment variables:

```bash
# User configuration
export ODI_USER_NAME="John Doe"
export ODI_USER_EMAIL="john@example.com"

# Display settings
export ODI_DEFAULT_FORMAT="json"
export ODI_NO_COLOR="1"            # Disable colors
export ODI_QUIET="1"               # Enable quiet mode

# Editor and pager
export ODI_EDITOR="code"
export ODI_PAGER="bat"

# Network settings
export ODI_TIMEOUT="60"
export ODI_HTTP_PROXY="http://proxy:8080"

# Custom config file
export ODI_CONFIG="/path/to/custom/config"
```

## Command-Line Overrides

Most configuration values can be overridden on the command line:

```bash
# Override user settings
odi issue create "Bug" --author "jane@example.com"

# Override display format
odi issue list --format json

# Override configuration file
odi --config /path/to/config init

# Override editor
odi issue create "Feature" --editor nano
```

## Configuration Management Commands

### View Configuration
```bash
# List all configuration
odi config list

# List only global config
odi config list --global

# List only workspace config
odi config list --local

# Get specific value
odi config get user.name
odi config get remotes.origin.url
```

### Modify Configuration
```bash
# Set values
odi config set user.name "John Doe"
odi config set defaults.priority high
odi config set sync.auto_push true

# Unset values  
odi config unset defaults.editor

# Edit configuration file directly
odi config edit           # Edit workspace config
odi config edit --global  # Edit global config
```

### Validate Configuration
```bash
# Check configuration validity
odi config check

# Validate specific file
odi config check --file /path/to/config
```

## Configuration Examples

### Team Development Setup

**Global config (`~/.odiconfig`):**
```toml
[user]
name = "Team Member"
email = "member@company.com"

[defaults]
auto_assign = false
format = "table"

[network]
timeout = 30
proxy = "http://proxy.company.com:8080"
```

**Workspace config (`.odi/config`):**
```toml
[project]
name = "Company Product"
default_branch = "develop"

[remotes.origin]
url = "git@company.git:product/issues.git"
protocol = "ssh"

[sync]
auto_pull = true
conflict_strategy = "prompt"

[git]
enabled = true
auto_link_commits = true
```

### Personal Project Setup

```toml
[user]
name = "Solo Developer"
email = "dev@personal.com"

[project]
name = "Personal App"

[defaults]
auto_assign = true
priority = "medium"

[remotes.origin]
url = "https://github.com/username/app-issues.git"
protocol = "https"

[sync]
auto_push = true
auto_pull = true
```

### CI/CD Integration

```toml
[user]
name = "CI Bot"
email = "ci@company.com"

[defaults]
format = "json"
auto_assign = false

[display]
show_colors = false

[sync]
timeout = 120
retries = 5
```

## Security Best Practices

1. **Use SSH keys** for authentication when possible
2. **Store tokens securely** in environment variables, not config files
3. **Verify SSL certificates** in production environments
4. **Limit trusted hosts** to only necessary domains
5. **Use specific branches** for remotes instead of default
6. **Regular credential rotation** for long-running deployments

## Troubleshooting Configuration

### Common Issues

**Configuration not loading:**
```bash
# Check config file locations
odi config list --debug

# Validate syntax
odi config check
```

**Authentication failures:**
```bash
# Check remote configuration
odi remote list --verbose

# Test connectivity
odi remote test origin
```

**Merge conflicts:**
```bash
# Check sync settings
odi config get sync.conflict_strategy

# Update conflict handling
odi config set sync.conflict_strategy prompt
```