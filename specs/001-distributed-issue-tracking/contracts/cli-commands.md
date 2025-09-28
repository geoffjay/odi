# CLI Command Contracts

**Scope**: Command-line interface contracts for ODI distributed issue tracking
**Format**: Input validation, output specification, error handling

## Core Commands

### `odi init`
**Purpose**: Initialize ODI issue tracking in current directory

**Usage**: `odi init [OPTIONS]`

**Arguments**:
- `--git-repo <PATH>`: Associate with existing Git repository (optional)
- `--remote <URL>`: Add initial remote repository (optional)
- `--config <PATH>`: Use alternative config file (optional)

**Input Validation**:
- Directory must be writable
- Git repository path must exist if specified
- Remote URL must be valid SSH or HTTPS format
- Config file must be valid TOML if specified

**Output Contract**:
```
SUCCESS (exit 0):
Initialized ODI repository in /path/to/project
Git repository detected: /path/to/project/.git
Created configuration: .odi/config.toml

ERROR (exit 1-127):
Error: Directory not writable: /path/to/project
Error: Invalid remote URL: invalid-url
Error: Git repository not found: /invalid/path
```

**Side Effects**:
- Creates `.odi/` directory structure
- Generates initial configuration files
- Associates with Git repository if detected/specified

---

### `odi issue create`
**Purpose**: Create new issue in current project

**Usage**: `odi issue create <TITLE> [OPTIONS]`

**Arguments**:
- `<TITLE>`: Issue title (required, 1-100 characters)
- `--description <TEXT>`: Issue description (optional)
- `--assignee <USER>`: Assign to user (optional, multiple allowed)
- `--label <LABEL>`: Add label (optional, multiple allowed)
- `--priority <LEVEL>`: Set priority (low|medium|high|critical)

**Input Validation**:
- Title must be 1-100 characters, non-empty
- Assignees must be valid user IDs
- Labels must exist in current project
- Priority must be valid enum value

**Output Contract**:
```
SUCCESS (exit 0):
Created issue #a1b2c3d4: Fix login validation bug
Assigned to: @john, @sarah
Labels: bug, frontend
Priority: high

ERROR (exit 1):
Error: Issue title cannot be empty
Error: User not found: @invalid-user  
Error: Label not found: invalid-label
Error: Not in ODI repository (run 'odi init' first)
```

**Side Effects**:
- Creates new issue file in `.odi/issues/`
- Updates issue index
- Triggers hooks if configured

---

### `odi issue assign`
**Purpose**: Assign issue to users or teams

**Usage**: `odi issue assign <ISSUE> <ASSIGNEE...>`

**Arguments**:
- `<ISSUE>`: Issue identifier (#id or partial ID)
- `<ASSIGNEE...>`: One or more users (@user) or teams (@team/name)

**Input Validation**:
- Issue must exist and be accessible
- Assignees must be valid users or teams
- User must have permission to modify issue

**Output Contract**:
```
SUCCESS (exit 0):
Issue #a1b2c3d4 assigned to @john, @team/backend
Previous assignees: @sarah (removed)

ERROR (exit 1):
Error: Issue not found: #invalid
Error: User not found: @invalid-user
Error: Permission denied: cannot assign issue
Error: Ambiguous issue ID: #a1b (matches multiple issues)
```

---

### `odi issue list`
**Purpose**: List and filter issues

**Usage**: `odi issue list [OPTIONS]`

**Arguments**:
- `--assignee <USER>`: Filter by assignee (optional)
- `--label <LABEL>`: Filter by label (optional, multiple allowed)
- `--status <STATUS>`: Filter by status (optional)
- `--format <FORMAT>`: Output format (table|json|csv)
- `--limit <N>`: Maximum number of results

**Input Validation**:
- Assignee must be valid user if specified
- Labels must exist if specified
- Status must be valid enum value
- Format must be supported output type
- Limit must be positive integer

**Output Contract**:
```
SUCCESS (exit 0):
ID       TITLE                    STATUS      ASSIGNEE    LABELS
a1b2c3d4 Fix login validation     Open        @john       bug, frontend
e5f6g7h8 Add user registration    InProgress  @sarah      feature, backend
i9j0k1l2 Update documentation     Resolved    @team/docs  docs

JSON FORMAT:
[
  {
    "id": "a1b2c3d4",
    "title": "Fix login validation bug", 
    "status": "Open",
    "assignees": ["john"],
    "labels": ["bug", "frontend"],
    "created_at": "2025-09-28T10:30:00Z"
  }
]
```

---

### `odi remote add`
**Purpose**: Add remote repository connection

**Usage**: `odi remote add <NAME> <URL> [OPTIONS]`

**Arguments**:
- `<NAME>`: Remote name identifier (origin, upstream, etc.)
- `<URL>`: Remote repository URL (SSH or HTTPS)
- `--auth <METHOD>`: Authentication method (ssh-key|token|oauth)

**Input Validation**:
- Name must be valid identifier format
- URL must be valid SSH or HTTPS format
- Auth method must be supported
- Remote name must not already exist

**Output Contract**:
```
SUCCESS (exit 0):
Added remote 'origin': https://issues.example.com/project.git
Authentication: SSH key (~/.ssh/id_rsa)

ERROR (exit 1):
Error: Invalid remote name: 'invalid name'
Error: Invalid URL format: not-a-url
Error: Remote already exists: origin
Error: Authentication failed: invalid SSH key
```

---

### `odi remote pull`
**Purpose**: Pull changes from remote repository

**Usage**: `odi remote pull [REMOTE] [OPTIONS]`

**Arguments**:
- `[REMOTE]`: Remote name (defaults to 'origin')
- `--force`: Force pull with potential data loss
- `--dry-run`: Show what would be pulled without applying

**Input Validation**:
- Remote must exist and be configured
- Authentication must be valid
- Network connectivity required

**Output Contract**:
```
SUCCESS (exit 0):
Pulling from origin (https://issues.example.com/project.git)
Downloaded 3 new issues, 2 updated issues
Conflicts detected in issue #a1b2c3d4 - manual resolution required

CONFLICT RESOLUTION REQUIRED (exit 2):
Conflict in issue #a1b2c3d4:
<<< LOCAL
Title: Fix login bug
>>> REMOTE  
Title: Resolve authentication issue
====
Run 'odi resolve #a1b2c3d4' to resolve conflicts

ERROR (exit 1):
Error: Remote not found: invalid-remote
Error: Authentication failed
Error: Network error: connection timeout
```

---

### `odi remote push` 
**Purpose**: Push local changes to remote repository

**Usage**: `odi remote push [REMOTE] [OPTIONS]`

**Arguments**:
- `[REMOTE]`: Remote name (defaults to 'origin')
- `--force`: Force push with potential remote data loss
- `--dry-run`: Show what would be pushed without sending

**Input Validation**:
- Remote must exist and be configured
- Authentication must be valid and have write access
- No unresolved conflicts in local repository

**Output Contract**:
```
SUCCESS (exit 0):
Pushing to origin (https://issues.example.com/project.git)
Uploaded 2 new issues, 1 updated issue
Push completed successfully

ERROR (exit 1):
Error: Unresolved conflicts - resolve before pushing
Error: Permission denied: no write access to remote
Error: Push rejected: remote has newer changes (pull first)
Error: Network error: connection failed
```

## Exit Code Standards

**Standard Exit Codes**:
- `0`: Success - operation completed successfully
- `1`: General error - invalid arguments, file not found, etc.
- `2`: Conflict resolution required - user intervention needed
- `126`: Permission denied - insufficient access rights
- `127`: Command not found - invalid subcommand or missing dependency
- `128+N`: Signal termination - interrupted by signal N

## Error Message Format

**Structure**: `Error: <description>` followed by optional suggestion
**Examples**:
```
Error: Issue not found: #invalid
Suggestion: Use 'odi issue list' to see available issues

Error: Not in ODI repository
Suggestion: Run 'odi init' to initialize issue tracking
```

## Output Formatting

**Human-readable**: Default format with colors and tables
**JSON**: Machine-readable format for scripting (`--format json`)
**CSV**: Spreadsheet-compatible format for data export

---

**CLI Contracts Complete**: All major commands defined with input/output specifications and error handling.