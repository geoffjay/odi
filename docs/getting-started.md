# Getting Started with ODI

This guide will walk you through setting up ODI and creating your first issues and projects.

## Installation

### Prerequisites
- Rust 1.75 or higher
- Git (optional, for Git integration)

### Install from Source
```bash
git clone https://github.com/your-org/odi.git
cd odi
cargo install --path odi
```

Verify installation:
```bash
odi --version
```

## First Time Setup

### 1. Initialize a Workspace
Navigate to your project directory and initialize ODI:

```bash
cd /path/to/your/project
odi init --project "My Project"
```

This creates a `.odi` directory with:
```
.odi/
├── config          # Configuration file
├── objects/         # Binary object storage
└── refs/           # Reference tracking
```

### 2. Configure Your Identity
Set your name and email for issue authorship:

```bash
odi config set user.name "John Doe"
odi config set user.email "john@example.com"
```

### 3. Verify Configuration
Check your current configuration:

```bash
odi config list
```

## Creating Your First Issue

### Basic Issue Creation
```bash
odi issue create "Fix login validation bug"
```

### Issue with Details
```bash
odi issue create "Improve error handling" \
  --description "Add better error messages for API failures" \
  --priority high \
  --assignee alice \
  --label bug,api
```

### List Issues
```bash
# List all issues
odi issue list

# List issues by status
odi issue list --status open

# List issues by assignee
odi issue list --assignee alice
```

## Working with Projects

### Create Additional Projects
```bash
odi project create "Frontend" --description "UI and UX components"
odi project create "Backend" --description "API and business logic"
```

### List Projects
```bash
odi project list
```

### Assign Issues to Projects
```bash
odi issue update <issue-id> --project Frontend
```

## Team Management

### Create Teams
```bash
odi team create "developers" --description "Development team"
odi team create "qa" --description "Quality assurance team"
```

### Add Users to Teams
```bash
odi team add-member developers alice
odi team add-member developers bob
odi team add-member qa charlie
```

### Assign Issues to Teams
```bash
odi issue update <issue-id> --assignee-team developers
```

## Working with Labels

### Create Labels
```bash
odi label create "bug" --color red --description "Something isn't working"
odi label create "feature" --color blue --description "New functionality"
odi label create "urgent" --color orange --description "Needs immediate attention"
```

### Apply Labels to Issues
```bash
odi issue update <issue-id> --add-label bug,urgent
```

## Git Integration

If ODI detects a Git repository, it automatically integrates with it:

### Link Issues to Commits
```bash
# Reference issues in commit messages
git commit -m "Fix validation logic

Fixes: <issue-id>"
```

### View Git References
```bash
odi issue show <issue-id> --refs
```

## Remote Synchronization

### Add a Remote Repository
```bash
# HTTPS
odi remote add origin https://github.com/username/project-issues.git

# SSH
odi remote add origin git@github.com:username/project-issues.git
```

### Push Changes
```bash
odi push origin
```

### Pull Changes
```bash
odi pull origin
```

### Handle Conflicts
When conflicts occur during sync:

```bash
# ODI will show conflict details
odi status

# Resolve conflicts manually by editing .odi/conflicts/
# Then mark as resolved
odi resolve --all

# Complete the sync
odi sync continue
```

## Workflow Examples

### Daily Workflow
```bash
# Start your day - pull latest changes
odi pull origin

# Create a new issue
odi issue create "Add user avatar support"

# Work on an existing issue
odi issue update <issue-id> --status in-progress --assignee $(odi config get user.name)

# Complete the issue
odi issue update <issue-id> --status resolved

# Push your changes
odi push origin
```

### Project Management Workflow
```bash
# Create project structure
odi project create "Q1 2024 Sprint"
odi label create "q1-sprint" --color purple

# Bulk create issues for sprint
odi issue create "User authentication" --project "Q1 2024 Sprint" --label q1-sprint
odi issue create "Database migration" --project "Q1 2024 Sprint" --label q1-sprint
odi issue create "API documentation" --project "Q1 2024 Sprint" --label q1-sprint

# Assign to team members
odi issue list --project "Q1 2024 Sprint" --format ids | \
  xargs -I {} odi issue update {} --assignee alice

# Track progress
odi issue list --project "Q1 2024 Sprint" --status in-progress
```

## Configuration Tips

### Global Configuration
Edit `~/.odiconfig` for settings that apply to all ODI workspaces:

```toml
[user]
name = "John Doe"
email = "john@example.com"

[defaults]
priority = "medium"
auto_assign = true
```

### Workspace Configuration
Edit `.odi/config` for workspace-specific settings:

```toml
[project]
name = "My Project"
default_branch = "main"

[sync]
auto_push = true
conflict_strategy = "prompt"

[remotes.origin]
url = "https://github.com/username/project-issues.git"
protocol = "https"
```

## Next Steps

- Read the [Command Reference](commands.md) for detailed command documentation
- Learn about [Configuration](configuration.md) options
- Understand [Synchronization](sync.md) and conflict resolution
- Explore the [Architecture](architecture.md) for advanced usage

## Troubleshooting

### Common Issues

**ODI workspace already initialized**
```bash
# If you need to reinitialize, remove the .odi directory first
rm -rf .odi
odi init
```

**Permission denied on push**
```bash
# Check your remote URL and credentials
odi remote -v
odi config get user.email

# For SSH, ensure your key is added to ssh-agent
ssh-add ~/.ssh/id_rsa
```

**Merge conflicts**
```bash
# View conflict status
odi status

# List conflicted files
ls .odi/conflicts/

# After manual resolution
odi resolve --all
```

### Getting Help
```bash
# Command help
odi --help
odi issue --help
odi issue create --help

# Version information
odi --version

# Configuration diagnostics
odi config check
```