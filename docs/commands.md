# Command Reference

Complete reference for all ODI commands and their options.

## Global Options

All ODI commands support these global options:

- `--verbose, -v`: Enable verbose output
- `--quiet, -q`: Suppress non-essential output
- `--config <path>`: Use custom configuration file
- `--help, -h`: Show help information
- `--version, -V`: Show version information

## `odi init`

Initialize a new ODI workspace in the current directory.

### Usage
```bash
odi init [OPTIONS]
```

### Options
- `--project <name>`: Create initial project with specified name
- `--git-repo <path>`: Associate with existing Git repository
- `--remote <url>`: Add initial remote repository
- `--config <path>`: Use custom configuration file

### Examples
```bash
# Basic initialization
odi init

# Initialize with project
odi init --project "My Application"

# Initialize with Git integration
odi init --git-repo /path/to/git/repo

# Initialize with remote
odi init --remote https://github.com/user/repo-issues.git
```

### Output Structure
```
.odi/
├── config          # Workspace configuration
├── objects/         # Binary object storage
│   ├── issues/     # Issue objects
│   ├── projects/   # Project objects
│   ├── users/      # User objects
│   └── teams/      # Team objects
└── refs/           # Reference tracking
    ├── heads/      # Local references
    └── remotes/    # Remote references
```

## `odi issue`

Manage issues within the workspace.

### `odi issue create`

Create a new issue.

#### Usage
```bash
odi issue create <title> [OPTIONS]
```

#### Options
- `--description, -d <text>`: Issue description
- `--priority, -p <level>`: Priority (low, medium, high, critical)
- `--assignee, -a <user>`: Assign to user
- `--project <name>`: Assign to project
- `--label, -l <label>`: Add labels (comma-separated)
- `--template, -t <path>`: Use issue template

#### Examples
```bash
# Simple issue
odi issue create "Fix login bug"

# Detailed issue
odi issue create "Improve error handling" \
  --description "Add better error messages for API failures" \
  --priority high \
  --assignee alice \
  --project "Backend" \
  --label "bug,api,urgent"

# Using template
odi issue create "Bug report" --template .odi/templates/bug.md
```

### `odi issue list`

List issues with filtering and formatting options.

#### Usage
```bash
odi issue list [OPTIONS]
```

#### Options
- `--status, -s <status>`: Filter by status (open, in-progress, resolved, closed)
- `--assignee, -a <user>`: Filter by assignee
- `--project, -p <project>`: Filter by project
- `--label, -l <label>`: Filter by label
- `--author <user>`: Filter by author
- `--priority <level>`: Filter by priority
- `--format, -f <format>`: Output format (table, json, csv, ids)
- `--limit, -n <count>`: Limit number of results
- `--sort <field>`: Sort by field (created, updated, priority, status)
- `--order <direction>`: Sort order (asc, desc)

#### Examples
```bash
# List all issues
odi issue list

# Open issues assigned to alice
odi issue list --status open --assignee alice

# High priority bugs
odi issue list --priority high --label bug

# JSON output for scripting
odi issue list --format json --status open

# Just issue IDs
odi issue list --format ids --assignee bob
```

### `odi issue show`

Display detailed information about a specific issue.

#### Usage
```bash
odi issue show <issue-id> [OPTIONS]
```

#### Options
- `--format, -f <format>`: Output format (default, json, yaml)
- `--refs`: Show Git references
- `--history`: Show change history
- `--comments`: Show comments

#### Examples
```bash
# Basic issue details
odi issue show a1b2c3d4

# With Git references
odi issue show a1b2c3d4 --refs

# JSON format
odi issue show a1b2c3d4 --format json
```

### `odi issue update`

Update an existing issue.

#### Usage
```bash
odi issue update <issue-id> [OPTIONS]
```

#### Options
- `--title, -t <title>`: Update title
- `--description, -d <text>`: Update description
- `--status, -s <status>`: Change status
- `--priority, -p <level>`: Change priority
- `--assignee, -a <user>`: Change assignee
- `--add-assignee <user>`: Add additional assignee
- `--remove-assignee <user>`: Remove assignee
- `--project <name>`: Move to project
- `--add-label <label>`: Add labels
- `--remove-label <label>`: Remove labels
- `--comment, -c <text>`: Add comment

#### Examples
```bash
# Update status
odi issue update a1b2c3d4 --status in-progress

# Add assignee and labels
odi issue update a1b2c3d4 --add-assignee bob --add-label "in-review"

# Add comment
odi issue update a1b2c3d4 --comment "Working on this now"
```

### `odi issue close`

Close an issue.

#### Usage
```bash
odi issue close <issue-id> [OPTIONS]
```

#### Options
- `--reason, -r <reason>`: Closure reason (fixed, duplicate, invalid, wontfix)
- `--comment, -c <text>`: Closing comment

#### Examples
```bash
# Simple close
odi issue close a1b2c3d4

# Close with reason
odi issue close a1b2c3d4 --reason fixed --comment "Fixed in commit abc123"
```

## `odi project`

Manage projects within the workspace.

### `odi project create`

Create a new project.

#### Usage
```bash
odi project create <name> [OPTIONS]
```

#### Options
- `--description, -d <text>`: Project description
- `--default-branch <branch>`: Default Git branch
- `--template, -t <template>`: Use project template

#### Examples
```bash
# Simple project
odi project create "Frontend"

# Detailed project
odi project create "Backend API" \
  --description "REST API and business logic" \
  --default-branch main
```

### `odi project list`

List all projects.

#### Usage
```bash
odi project list [OPTIONS]
```

#### Options
- `--format, -f <format>`: Output format (table, json, csv)
- `--active`: Show only active projects

### `odi project show`

Show project details.

#### Usage
```bash
odi project show <project-name> [OPTIONS]
```

#### Options
- `--issues`: Include issue count and summary
- `--format, -f <format>`: Output format

## `odi team`

Manage teams and team membership.

### `odi team create`

Create a new team.

#### Usage
```bash
odi team create <name> [OPTIONS]
```

#### Options
- `--description, -d <text>`: Team description
- `--parent, -p <team>`: Parent team for hierarchy

### `odi team add-member`

Add a user to a team.

#### Usage
```bash
odi team add-member <team> <user> [OPTIONS]
```

#### Options
- `--role, -r <role>`: Member role (member, lead, admin)

### `odi team list`

List teams.

#### Usage
```bash
odi team list [OPTIONS]
```

#### Options
- `--format, -f <format>`: Output format
- `--members`: Include member counts

## `odi remote`

Manage remote repositories for synchronization.

### `odi remote add`

Add a remote repository.

#### Usage
```bash
odi remote add <name> <url> [OPTIONS]
```

#### Options
- `--protocol, -p <protocol>`: Force protocol (https, ssh)
- `--auth, -a <method>`: Authentication method

#### Examples
```bash
# HTTPS remote
odi remote add origin https://github.com/user/repo-issues.git

# SSH remote  
odi remote add origin git@github.com:user/repo-issues.git
```

### `odi remote list`

List configured remotes.

#### Usage
```bash
odi remote list [OPTIONS]
```

#### Options
- `--verbose, -v`: Show URLs and protocols

### `odi remote remove`

Remove a remote.

#### Usage
```bash
odi remote remove <name>
```

## `odi push`

Push changes to a remote repository.

### Usage
```bash
odi push <remote> [OPTIONS]
```

### Options
- `--force, -f`: Force push (overwrites remote)
- `--dry-run`: Show what would be pushed without pushing
- `--all`: Push all projects

### Examples
```bash
# Push to origin
odi push origin

# Force push
odi push origin --force

# Dry run
odi push origin --dry-run
```

## `odi pull`

Pull changes from a remote repository.

### Usage
```bash
odi pull <remote> [OPTIONS]
```

### Options
- `--merge`: Attempt automatic merge
- `--strategy <strategy>`: Merge strategy (auto, manual, ours, theirs)
- `--dry-run`: Show what would be pulled

### Examples
```bash
# Pull from origin
odi pull origin

# Pull with automatic merge
odi pull origin --merge

# Pull with strategy
odi pull origin --strategy auto
```

## `odi sync`

Advanced synchronization commands.

### `odi sync status`

Show sync status and pending changes.

### `odi sync resolve`

Resolve merge conflicts.

#### Usage
```bash
odi sync resolve [OPTIONS]
```

#### Options
- `--all`: Resolve all conflicts
- `--file <path>`: Resolve specific conflict file
- `--strategy <strategy>`: Resolution strategy

## `odi config`

Configuration management commands.

### `odi config get`

Get configuration value.

#### Usage
```bash
odi config get <key>
```

#### Examples
```bash
odi config get user.name
odi config get remotes.origin.url
```

### `odi config set`

Set configuration value.

#### Usage
```bash
odi config set <key> <value>
```

#### Examples
```bash
odi config set user.name "John Doe"
odi config set user.email "john@example.com"
```

### `odi config list`

List all configuration values.

#### Usage
```bash
odi config list [OPTIONS]
```

#### Options
- `--global`: Show global config only
- `--local`: Show workspace config only
- `--format, -f <format>`: Output format

### `odi config check`

Validate configuration.

#### Usage
```bash
odi config check
```

## `odi label`

Manage labels for categorizing issues.

### `odi label create`

Create a new label.

#### Usage
```bash
odi label create <name> [OPTIONS]
```

#### Options
- `--color, -c <color>`: Label color (name or hex)
- `--description, -d <text>`: Label description

### `odi label list`

List all labels.

#### Usage
```bash
odi label list [OPTIONS]
```

### `odi label delete`

Delete a label.

#### Usage
```bash
odi label delete <name>
```

## Exit Codes

ODI uses standard exit codes:

- `0`: Success
- `1`: General error
- `2`: Command usage error
- `3`: Configuration error
- `4`: Network error
- `5`: Merge conflict (manual resolution required)
- `6`: Permission denied
- `7`: Not found (workspace, issue, etc.)
- `8`: Already exists

## Environment Variables

ODI respects these environment variables:

- `ODI_CONFIG`: Override config file location
- `ODI_WORKSPACE`: Override workspace location
- `ODI_EDITOR`: Text editor for editing descriptions
- `ODI_PAGER`: Pager for long output
- `ODI_NO_COLOR`: Disable colored output
- `ODI_QUIET`: Enable quiet mode by default

## Configuration Files

ODI loads configuration from multiple sources in this order:

1. Command-line options (highest priority)
2. `.odi/config` (workspace-specific)
3. `~/.odiconfig` (user global)
4. Built-in defaults (lowest priority)

## Output Formats

Many commands support multiple output formats:

- `table`: Human-readable table (default)
- `json`: JSON format for scripting
- `csv`: Comma-separated values
- `yaml`: YAML format
- `ids`: Just IDs, one per line