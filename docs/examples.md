# ODI Examples

This document provides practical examples of using ODI for different scenarios and workflows.

## Basic Workflows

### Solo Developer Workflow

Setting up ODI for personal project management:

```bash
# Initialize workspace
cd ~/projects/my-app
odi init --project "Personal App"

# Configure identity
odi config set user.name "John Developer"
odi config set user.email "john@example.com"

# Create first issue
odi issue create "Setup CI/CD pipeline" \
  --description "Configure GitHub Actions for testing and deployment" \
  --priority high

# Work on the issue
odi issue update a1b2c3d4 --status in-progress --assignee john

# Create related issues
odi issue create "Add unit tests" --priority medium
odi issue create "Setup deployment" --priority medium

# Link to Git commits
git commit -m "Add initial CI config

Addresses: a1b2c3d4"

# Close completed issue
odi issue update a1b2c3d4 --status resolved
```

### Team Development Workflow

Collaborative issue tracking for a development team:

```bash
# Team lead sets up shared repository
odi init --project "Team Project" 
odi remote add origin https://github.com/team/project-issues.git

# Configure team settings
odi team create "developers" --description "Development team"
odi team create "qa" --description "QA team"

# Add team members
odi team add-member developers alice
odi team add-member developers bob  
odi team add-member qa charlie

# Create labels for organization
odi label create "bug" --color red --description "Something isn't working"
odi label create "feature" --color blue --description "New functionality"
odi label create "urgent" --color orange --description "Needs immediate attention"

# Create sprint project
odi project create "Sprint 1" --description "First development sprint"

# Create and assign issues
odi issue create "User authentication" \
  --project "Sprint 1" \
  --assignee alice \
  --label feature \
  --priority high

odi issue create "Database setup" \
  --project "Sprint 1" \
  --assignee bob \
  --label feature \
  --priority high

# Team members sync changes
odi push origin  # Push your changes
odi pull origin  # Get others' changes
```

## Advanced Use Cases

### Multi-Project Management

Managing multiple related projects in one workspace:

```bash
# Initialize workspace for multiple projects
odi init --project "Frontend"

# Add additional projects  
odi project create "Backend" --description "API and business logic"
odi project create "Mobile" --description "Mobile applications"
odi project create "DevOps" --description "Infrastructure and deployment"

# Create cross-project issues
odi issue create "Implement user API" \
  --project "Backend" \
  --label api,feature \
  --assignee alice

odi issue create "Integrate user API" \
  --project "Frontend" \
  --label integration,feature \
  --assignee bob

# Create epic that spans projects
odi issue create "User Management System" \
  --description "Complete user management across all platforms" \
  --priority critical \
  --label epic

# Link related issues (conceptually - full linking would be a feature)
# For now, reference in descriptions
odi issue update backend-issue-id \
  --description "Backend API for user management. Related to frontend integration issue."
```

### Release Management

Using ODI for release planning and tracking:

```bash
# Create release project
odi project create "Release 2.0" --description "Major feature release"

# Create release labels
odi label create "release-2.0" --color purple
odi label create "breaking-change" --color red
odi label create "documentation" --color green

# Plan release issues
odi issue create "API versioning" \
  --project "Release 2.0" \
  --label "breaking-change,release-2.0" \
  --priority critical

odi issue create "Migration guide" \
  --project "Release 2.0" \
  --label "documentation,release-2.0" \
  --priority high

odi issue create "Performance improvements" \
  --project "Release 2.0" \
  --label "enhancement,release-2.0" \
  --priority medium

# Track progress
odi issue list --project "Release 2.0" --status open
odi issue list --project "Release 2.0" --status resolved

# Generate release notes (conceptual - would need custom script)
odi issue list --project "Release 2.0" --format json | \
  jq -r '.[] | "- \(.title) (\(.id))"'
```

### Bug Triage Workflow

Systematic bug management process:

```bash
# Create triage labels
odi label create "triage" --color yellow --description "Needs investigation"
odi label create "confirmed" --color orange --description "Bug confirmed"
odi label create "wontfix" --color gray --description "Won't be fixed"

# Bug report comes in
odi issue create "App crashes on startup" \
  --description "Application crashes immediately when opened on iOS 15" \
  --priority high \
  --label bug,triage

# Triage process
odi issue update bug-id --add-label "confirmed" --remove-label "triage"
odi issue update bug-id --assignee alice
odi issue update bug-id --comment "Reproduced on iOS 15.4. Investigating root cause."

# Investigation updates
odi issue update bug-id --status in-progress
odi issue update bug-id --comment "Found issue in initialization code. Working on fix."

# Resolution
odi issue update bug-id --status resolved
odi issue update bug-id --comment "Fixed in commit abc123. Testing in QA."

# QA verification
odi issue update bug-id --assignee charlie  # QA team member
odi issue update bug-id --comment "QA: Verified fix works correctly. Ready for release."
odi issue update bug-id --status closed
```

## Integration Examples

### Git Integration

Using ODI with Git workflows:

```bash
# Setup Git integration
odi init --git-repo . --project "My App"

# Create feature branch workflow
git checkout -b feature/user-profiles
odi issue create "Add user profile pages" \
  --description "Allow users to view and edit their profiles" \
  --assignee $(odi config get user.name)

# Work on feature with linked commits
git add user-profile.js
git commit -m "Add user profile component

Addresses: issue-id
- Add profile display component  
- Add edit profile form
- Add profile image upload"

# Create PR and link
git push origin feature/user-profiles
# In GitHub PR description: "Fixes: issue-id"

# After merge, update issue
odi issue update issue-id --status resolved
```

### CI/CD Integration

Automating issue updates in CI/CD pipelines:

```bash
# .github/workflows/ci.yml script section
- name: Update issues on deployment
  run: |
    # Extract issue IDs from commit messages
    ISSUES=$(git log --oneline ${{ github.event.before }}..${{ github.sha }} | \
      grep -oE 'Addresses: [a-f0-9-]+' | \
      cut -d' ' -f2 | sort -u)
    
    # Update issues to deployed status
    for issue_id in $ISSUES; do
      odi issue update $issue_id --add-label "deployed" \
        --comment "Deployed to production in release ${{ github.ref_name }}"
    done
```

### Slack Integration (Conceptual Plugin)

Automated notifications to team chat:

```toml
# .odi/config
[plugins.slack-notifications]
enabled = true
webhook_url = "https://hooks.slack.com/services/..."

[plugins.slack-notifications.rules]
# Notify on high priority issues
high_priority_issues = { channel = "#urgent", priority = "high" }
# Notify on issue completion
resolved_issues = { channel = "#development", status = "resolved" }
```

## Synchronization Scenarios

### Distributed Team Sync

Multiple team members working across different locations:

```bash
# Team member 1 (Alice) - San Francisco
odi pull origin  # Get latest changes
odi issue create "Add payment processing" --assignee alice
odi issue update existing-issue --status in-progress
odi push origin  # Share changes

# Team member 2 (Bob) - London  
odi pull origin  # Gets Alice's changes
odi issue create "Update documentation" --assignee bob
odi issue update alice-issue --comment "I can help with testing this"
odi push origin

# Team member 3 (Charlie) - Tokyo
odi pull origin  # Gets both Alice's and Bob's changes
odi issue list --assignee alice  # See Alice's work
odi issue list --status open    # See all open work
```

### Conflict Resolution

Handling synchronization conflicts:

```bash
# Both Alice and Bob update the same issue
# Alice updates: status to "in-progress", adds comment "Working on this"
# Bob updates: priority to "critical", adds comment "This is urgent"

# When Bob tries to push:
odi push origin
# Error: Conflict detected for issue abc123

# View conflicts
odi conflicts list
# Shows: Issue abc123 has conflicting updates

# Resolve interactively
odi resolve abc123
# ODI presents conflict resolution UI:
# 1. Keep local changes (Bob's priority change)  
# 2. Keep remote changes (Alice's status change)
# 3. Merge both changes
# 4. Edit manually

# Choose option 3 (merge both)
# Result: Issue has both status=in-progress AND priority=critical
# Plus both comments are preserved

# Complete sync
odi push origin
```

### Backup and Recovery

Setting up multiple remotes for redundancy:

```bash
# Primary remote (GitHub)
odi remote add origin https://github.com/company/project-issues.git

# Backup remote (GitLab)  
odi remote add backup https://gitlab.com/company/project-issues.git

# Private backup (self-hosted)
odi remote add private git@backup.company.com:project-issues.git

# Push to all remotes
odi push origin
odi push backup  
odi push private

# Or push to all at once (conceptual feature)
odi push --all

# Recovery scenario - primary remote unavailable
odi remote remove origin
odi remote add origin backup  # Promote backup to primary
odi pull origin               # Restore from backup
```

## Automation Examples

### Issue Templates

Creating standardized issue templates:

```bash
# Create template directory
mkdir -p .odi/templates

# Bug report template
cat > .odi/templates/bug.md << 'EOF'
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

## Additional Context
EOF

# Feature request template  
cat > .odi/templates/feature.md << 'EOF'
# Feature Request

**Requested by:** {author}
**Priority:** {priority}

## Problem Statement
What problem does this solve?

## Proposed Solution  
How should this work?

## Alternatives Considered
What other approaches were considered?

## Acceptance Criteria
- [ ] Criterion 1
- [ ] Criterion 2  
- [ ] Criterion 3
EOF

# Use templates
odi issue create "Login bug" --template .odi/templates/bug.md
odi issue create "Dark mode" --template .odi/templates/feature.md
```

### Bulk Operations

Managing multiple issues efficiently:

```bash
# Create multiple related issues
for component in "auth" "database" "api" "frontend"; do
  odi issue create "Refactor $component module" \
    --project "Refactoring Sprint" \
    --label "refactoring,tech-debt" \
    --priority medium
done

# Bulk update issues
odi issue list --project "Refactoring Sprint" --format ids | \
  xargs -I {} odi issue update {} --add-label "ready-for-review"

# Bulk status updates  
odi issue list --label "ready-for-review" --format ids | \
  xargs -I {} odi issue update {} --status resolved

# Generate reports
echo "## Sprint Summary"
echo "Total issues: $(odi issue list --project 'Refactoring Sprint' | wc -l)"
echo "Completed: $(odi issue list --project 'Refactoring Sprint' --status resolved | wc -l)"
echo "In progress: $(odi issue list --project 'Refactoring Sprint' --status in-progress | wc -l)"
```

### Custom Workflows

Building custom workflows with scripts:

```bash
#!/bin/bash
# scripts/start-feature.sh - Start working on a new feature

set -e

feature_name="$1"
if [ -z "$feature_name" ]; then
  echo "Usage: $0 <feature-name>"
  exit 1
fi

echo "Starting work on feature: $feature_name"

# Create Git branch
git checkout -b "feature/$feature_name"

# Create ODI issue
issue_id=$(odi issue create "Implement $feature_name" \
  --assignee "$(odi config get user.name)" \
  --label "feature,in-development" \
  --priority medium \
  --format json | jq -r '.id')

echo "Created issue: $issue_id"

# Update issue to in-progress
odi issue update "$issue_id" --status in-progress

# Create initial commit with issue reference
git commit --allow-empty -m "Start feature: $feature_name

Addresses: $issue_id"

echo "Ready to start development!"
echo "Issue ID: $issue_id"
echo "Branch: feature/$feature_name"
```

```bash
#!/bin/bash  
# scripts/finish-feature.sh - Complete a feature

set -e

issue_id="$1"
if [ -z "$issue_id" ]; then
  echo "Usage: $0 <issue-id>"
  exit 1
fi

echo "Finishing feature for issue: $issue_id"

# Update issue status
odi issue update "$issue_id" --status resolved \
  --comment "Feature implementation complete. Ready for review."

# Push changes
git push origin HEAD

echo "Feature completed!"
echo "Don't forget to create a pull request."
```

## Integration with External Tools

### Jira Migration (Conceptual)

Migrating from Jira to ODI:

```bash
#!/bin/bash
# scripts/migrate-from-jira.sh

# Export Jira issues (using Jira API)
curl -u "$JIRA_USER:$JIRA_TOKEN" \
  "$JIRA_URL/rest/api/2/search?jql=project=$PROJECT" > jira-export.json

# Convert and import to ODI
jq -r '.issues[] | 
  "odi issue create \"" + .fields.summary + "\" " +
  "--description \"" + (.fields.description // "") + "\" " +  
  "--priority " + (.fields.priority.name | ascii_downcase) + " " +
  "--assignee \"" + (.fields.assignee.emailAddress // "") + "\""' \
  jira-export.json > import-commands.sh

# Execute import
bash import-commands.sh

echo "Jira migration complete!"
```

### Time Tracking Integration

Adding time tracking to issues:

```bash
# Custom time tracking (would need plugin support)
odi issue update abc123 --add-metadata "time_spent=2h30m"
odi issue update abc123 --add-metadata "time_estimate=4h"

# Query time data  
odi issue list --format json | \
  jq -r '.[] | select(.metadata.time_spent) | 
    "\(.title): \(.metadata.time_spent)"'
```

These examples demonstrate ODI's flexibility for various workflows and use cases. As ODI evolves, more automation and integration possibilities will become available through plugins and extended functionality.