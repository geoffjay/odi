# Synchronization Guide

ODI's distributed synchronization system allows teams to collaborate on issue tracking while maintaining full offline functionality. This guide covers remote repositories, conflict resolution, and advanced synchronization scenarios.

## Overview

ODI synchronization works similarly to Git but is designed specifically for issue data:

- **Distributed**: No central server required, any repository can be a remote
- **Offline-first**: Full functionality without network connectivity
- **Conflict resolution**: Automatic merging with manual resolution when needed
- **Multi-protocol**: Support for HTTPS, SSH, and custom protocols

## Remote Repositories

### Adding Remotes

Remote repositories store and share issue data between ODI workspaces.

```bash
# Add HTTPS remote
odi remote add origin https://github.com/username/project-issues.git

# Add SSH remote  
odi remote add origin git@github.com:username/project-issues.git

# Add multiple remotes
odi remote add upstream https://github.com/upstream/project-issues.git
odi remote add backup git@backup-server.com:project-issues.git
```

### Remote Configuration

Remotes can be configured in `.odi/config`:

```toml
[remotes.origin]
url = "https://github.com/username/project-issues.git"
protocol = "https"
auth_method = "token"
branch = "main"
timeout = 30

[remotes.upstream]  
url = "git@github.com:upstream/project-issues.git"
protocol = "ssh"
auth_method = "key"
key_file = "~/.ssh/id_rsa_odi"
```

### Managing Remotes

```bash
# List remotes
odi remote list

# Show detailed remote information
odi remote list --verbose

# Remove a remote
odi remote remove backup

# Update remote URL
odi remote set-url origin https://new-server.com/project-issues.git

# Test remote connectivity
odi remote test origin
```

## Push and Pull Operations

### Pushing Changes

Push your local changes to a remote repository:

```bash
# Push to default remote (origin)
odi push

# Push to specific remote
odi push origin

# Push specific projects
odi push origin --project "Backend"

# Force push (overwrites remote)
odi push origin --force

# Dry run (show what would be pushed)
odi push origin --dry-run
```

### Pulling Changes

Pull changes from a remote repository:

```bash
# Pull from default remote
odi pull

# Pull from specific remote  
odi pull upstream

# Pull with automatic merge
odi pull origin --merge

# Pull with specific merge strategy
odi pull origin --strategy ours

# Dry run (show what would be pulled)
odi pull origin --dry-run
```

## Synchronization States

ODI tracks synchronization state for each remote:

### Checking Sync Status

```bash
# Show overall sync status
odi status

# Show status for specific remote
odi status origin

# Show detailed change information
odi status --verbose
```

Example output:
```
ODI Workspace Status

Local changes:
  Modified issues: 3
  New issues: 1  
  Deleted issues: 0

Remote 'origin':
  Ahead by: 2 commits
  Behind by: 1 commit
  Status: Needs sync

Remote 'upstream':
  Ahead by: 0 commits
  Behind by: 5 commits
  Status: Can pull

Conflicts: None
```

### Sync States

- **In sync**: Local and remote are identical
- **Ahead**: Local has changes not on remote (need to push)
- **Behind**: Remote has changes not local (need to pull)  
- **Diverged**: Both have unique changes (need merge)
- **Conflict**: Automatic merge failed (need manual resolution)

## Conflict Resolution

When ODI cannot automatically merge changes, manual conflict resolution is required.

### Understanding Conflicts

Conflicts occur when:
- Same issue modified differently in local and remote
- Issue deleted locally but modified remotely (or vice versa)
- Project or team configurations conflict
- Structural changes conflict (e.g., project restructuring)

### Conflict Types

#### Issue Content Conflicts
When the same issue is modified differently:

```bash
# Show conflicts
odi status

# List conflicted items
odi conflicts list

# Show specific conflict
odi conflicts show <issue-id>
```

Example conflict output:
```
Issue: Fix login validation (a1b2c3d4)
Conflict: Content modification

Local version:
  Title: Fix login validation  
  Status: in-progress
  Assignee: alice
  Updated: 2024-01-15 10:30:00

Remote version:
  Title: Fix login validation bug
  Status: resolved  
  Assignee: bob
  Updated: 2024-01-15 11:45:00

Resolution required for:
  - title: "Fix login validation" vs "Fix login validation bug"
  - status: in-progress vs resolved
  - assignee: alice vs bob
```

#### Structural Conflicts
When organizational structure conflicts:

```bash
# Project conflicts
odi conflicts list --type project

# Team conflicts  
odi conflicts list --type team

# Configuration conflicts
odi conflicts list --type config
```

### Manual Resolution

ODI provides several tools for resolving conflicts:

#### Interactive Resolution
```bash
# Start interactive conflict resolution
odi resolve

# Resolve specific issue
odi resolve <issue-id>

# Resolve all issues of a type
odi resolve --type issue
```

Interactive resolution presents options:
```
Conflict: Issue a1b2c3d4 (Fix login validation)

Choose resolution:
1. Keep local version (status: in-progress, assignee: alice)
2. Keep remote version (status: resolved, assignee: bob)  
3. Merge versions (combine changes)
4. Edit manually
5. Skip (resolve later)

Selection [1-5]: 3

Merge options:
- Title: Keep "Fix login validation bug" (remote)? [y/N]: y
- Status: Keep "resolved" (remote)? [y/N]: n  
- Assignee: Keep both alice and bob? [Y/n]: y

Result:
  Title: Fix login validation bug
  Status: in-progress
  Assignees: alice, bob
  
Confirm merge [Y/n]: y
```

#### Resolution Strategies
```bash
# Use automatic resolution strategy
odi resolve --strategy auto

# Prefer local changes
odi resolve --strategy ours

# Prefer remote changes  
odi resolve --strategy theirs

# Use timestamp-based resolution (newer wins)
odi resolve --strategy newer
```

#### Manual Editing
```bash
# Edit conflict files directly
odi conflicts edit <issue-id>

# This opens conflict files in your configured editor
# Files are in .odi/conflicts/ directory
```

Conflict files use a standard format:
```yaml
# .odi/conflicts/issues/a1b2c3d4.conflict
conflict_type: content
item_type: issue
item_id: a1b2c3d4

local:
  title: "Fix login validation"
  status: in-progress
  assignee: alice
  updated_at: "2024-01-15T10:30:00Z"

remote:
  title: "Fix login validation bug" 
  status: resolved
  assignee: bob
  updated_at: "2024-01-15T11:45:00Z"

# Edit the resolution section:
resolution:
  title: "Fix login validation bug"
  status: in-progress  
  assignees: [alice, bob]
  # ODI will merge other fields automatically
```

#### Completing Resolution
```bash
# Mark conflicts as resolved
odi resolve --complete

# Or mark specific items  
odi resolve <issue-id> --complete

# Verify resolution
odi conflicts list  # Should show no conflicts

# Complete the sync
odi sync continue
```

## Advanced Synchronization

### Selective Synchronization

Control what gets synchronized:

```bash
# Sync only specific projects
odi push origin --project "Backend,Frontend"

# Exclude specific projects
odi push origin --exclude-project "Archive"

# Sync only issues (no teams/projects)
odi push origin --issues-only

# Sync only metadata (no issue content)
odi push origin --metadata-only
```

### Branching and Merging

ODI supports basic branching for different work streams:

```bash
# Create a branch for experimental changes
odi branch create experiment

# Switch branches
odi branch checkout experiment  

# List branches
odi branch list

# Merge branch back to main
odi branch checkout main
odi merge experiment
```

### Synchronization Hooks

Configure automatic actions during sync operations:

```toml
# .odi/config
[hooks]
pre_push = "scripts/validate_issues.sh"
post_push = "scripts/notify_team.sh"
pre_pull = "scripts/backup_local.sh"  
post_pull = "scripts/update_dashboard.sh"
conflict = "scripts/notify_conflicts.sh"
```

Hook scripts receive context as environment variables:
```bash
#!/bin/bash
# scripts/validate_issues.sh

echo "Validating issues before push..."
echo "Remote: $ODI_REMOTE"
echo "Issues to push: $ODI_PUSH_ISSUES"

# Validate issue titles are not empty
for issue_id in $ODI_PUSH_ISSUES; do
    title=$(odi issue show $issue_id --format json | jq -r '.title')
    if [ -z "$title" ]; then
        echo "Error: Issue $issue_id has empty title"
        exit 1
    fi
done

echo "Validation passed"
```

### Multi-Remote Synchronization

Synchronize with multiple remotes for redundancy:

```bash
# Push to all remotes
odi push --all

# Pull from multiple remotes and merge
odi pull upstream  
odi pull origin --merge

# Sync with specific remotes in sequence
odi sync --remotes "upstream,origin,backup"
```

### Synchronization Scheduling

Automate synchronization with cron jobs or systemd timers:

```bash
# Crontab entry for hourly sync
0 * * * * cd /path/to/project && odi pull origin --merge >/dev/null 2>&1

# Daily push at end of day
0 18 * * * cd /path/to/project && odi push origin >/dev/null 2>&1
```

## Network Protocols

### HTTPS Authentication

```bash
# Using personal access tokens
export ODI_GITHUB_TOKEN="ghp_xxxxxxxxxxxx"
odi remote add origin https://github.com/username/repo-issues.git

# Using username/password (less secure)
odi remote add origin https://username:password@github.com/username/repo-issues.git
```

### SSH Authentication

```bash
# Standard SSH key
odi remote add origin git@github.com:username/repo-issues.git

# Custom SSH key
odi remote add origin git@github.com:username/repo-issues.git --key ~/.ssh/odi_key

# SSH with custom port
odi remote add origin ssh://git@server.com:2222/repo-issues.git
```

### Custom Protocols

ODI supports custom synchronization protocols via plugins:

```toml
# .odi/config
[protocols.s3]
plugin = "odi-s3-sync"
bucket = "company-issues"
region = "us-west-2"

[remotes.s3backup]
url = "s3://company-issues/project-issues"
protocol = "s3"
```

## Best Practices

### Team Workflows

#### Centralized Workflow
```bash
# Team lead maintains authoritative remote
# Developers push/pull from central remote

# Developer workflow:
odi pull origin          # Get latest changes
# ... work on issues ...
odi push origin          # Push changes
```

#### Fork-and-Pull Workflow  
```bash
# Developers work on forks, submit pull requests

# Fork workflow:
odi remote add upstream https://github.com/company/project-issues.git
odi remote add origin https://github.com/developer/project-issues.git

odi pull upstream        # Get upstream changes
# ... work on issues ...
odi push origin         # Push to fork
# Submit pull request via GitHub/GitLab
```

#### Feature Branch Workflow
```bash
# Use branches for different work streams

odi branch create feature/user-management
odi branch checkout feature/user-management
# ... work on user management issues ...
odi push origin feature/user-management

# Later merge back:
odi branch checkout main  
odi pull origin
odi merge feature/user-management
odi push origin
```

### Conflict Prevention

1. **Frequent syncing**: Pull changes regularly
2. **Clear ownership**: Assign issues to specific team members  
3. **Communication**: Coordinate major structural changes
4. **Atomic changes**: Keep modifications focused and small
5. **Standard workflows**: Establish team conventions for updates

### Performance Optimization

```bash
# Use shallow pulls for large histories
odi pull origin --depth 10

# Compress network traffic  
odi config set network.compression true

# Parallel synchronization
odi push origin --jobs 4

# Cache credentials
odi config set auth.cache_tokens true
```

### Security Considerations

1. **Use SSH keys** instead of passwords when possible
2. **Rotate credentials** regularly  
3. **Verify SSL certificates** for HTTPS connections
4. **Audit remote access** with `odi remote audit`
5. **Encrypt sensitive data** in issue descriptions
6. **Use branch protection** for important remotes

## Troubleshooting

### Common Sync Issues

**Network timeouts:**
```bash
# Increase timeout
odi config set network.timeout 120

# Use retry settings
odi config set network.retries 5
```

**Authentication failures:**
```bash
# Test credentials
odi remote test origin

# Update authentication
odi remote set-auth origin --method token --token $NEW_TOKEN
```

**Large repository performance:**
```bash
# Use shallow clone
odi clone https://server.com/repo.git --depth 50

# Compress objects
odi maintenance gc

# Prune old data
odi maintenance prune --older-than 90d
```

**Stuck synchronization:**
```bash
# Reset sync state
odi sync reset

# Force clean sync  
odi sync --reset-remote origin

# Recover from corruption
odi maintenance verify --repair
```