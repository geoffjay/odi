//! Filesystem check and repair commands

use clap::{Parser, Subcommand};
use odi_core::{IssueRepository, ProjectRepository, UserRepository};
use crate::{Result, AppContext};

/// Filesystem check and repair commands
#[derive(Parser)]
pub struct FsckArgs {
    #[command(subcommand)]
    pub command: FsckCommand,
}

#[derive(Subcommand)]
pub enum FsckCommand {
    /// Check object store integrity
    Check {
        /// Only report issues, don't fix them
        #[arg(long)]
        dry_run: bool,
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Repair corrupted objects
    Repair {
        /// Force repair even if backups exist  
        #[arg(long)]
        force: bool,
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Show storage statistics
    Stats {
        /// Include detailed breakdown
        #[arg(long)]
        detailed: bool,
    },
}

impl FsckArgs {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.command {
            FsckCommand::Check { dry_run, verbose } => {
                check_integrity(ctx, *dry_run, *verbose).await
            },
            FsckCommand::Repair { force, verbose } => {
                repair_objects(ctx, *force, *verbose).await
            },
            FsckCommand::Stats { detailed } => {
                show_storage_stats(ctx, *detailed).await
            },
        }
    }
}

async fn check_integrity(ctx: &AppContext, dry_run: bool, verbose: bool) -> Result<()> {
    println!("🔍 Checking ODI object store integrity...");
    
    let issue_repo = ctx.issue_repository();
    let project_repo = ctx.project_repository();
    let user_repo = ctx.user_repository();
    
    let mut issues_found = 0;
    let mut objects_checked = 0;
    
    // T079: Enhanced object store integrity checking
    println!("🗄️  Checking object store structure...");
    check_object_store_structure(ctx, &mut issues_found, &mut objects_checked, verbose).await?;
    
    // Check issues
    println!("📋 Checking issues...");
    match issue_repo.list(odi_core::issue::IssueQuery::default()).await {
        Ok(issues) => {
            for issue in issues {
                objects_checked += 1;
                
                if verbose {
                    println!("  ✓ Issue: {} ({})", issue.title, issue.id);
                }
                
                // Validate issue fields
                if issue.title.trim().is_empty() {
                    issues_found += 1;
                    println!("  ⚠️  Issue {} has empty title", issue.id);
                }
                
                // Check if created_at is in the future (corrupted timestamp)
                if issue.created_at > chrono::Utc::now() {
                    issues_found += 1;
                    println!("  ⚠️  Issue {} has future creation date: {}", issue.id, issue.created_at);
                }
                
                // Check if updated_at is before created_at
                if issue.updated_at < issue.created_at {
                    issues_found += 1;
                    println!("  ⚠️  Issue {} has update date before creation date", issue.id);
                }
            }
            println!("  📊 Checked {} issues", objects_checked);
        },
        Err(e) => {
            issues_found += 1;
            println!("  ✗ Failed to list issues: {}", e);
        }
    }
    
    // Check projects  
    println!("📁 Checking projects...");
    let mut project_count = 0;
    match project_repo.list_projects(odi_core::project::ProjectQuery::default()).await {
        Ok(projects) => {
            for project in projects {
                project_count += 1;
                objects_checked += 1;
                
                if verbose {
                    println!("  ✓ Project: {} ({})", project.name, project.id);
                }
                
                // Validate project fields
                if project.name.trim().is_empty() {
                    issues_found += 1;
                    println!("  ⚠️  Project {} has empty name", project.id);
                }
                
                // Check timestamps
                if project.created_at > chrono::Utc::now() {
                    issues_found += 1;
                    println!("  ⚠️  Project {} has future creation date: {}", project.id, project.created_at);
                }
            }
            println!("  📊 Checked {} projects", project_count);
        },
        Err(e) => {
            issues_found += 1;
            println!("  ✗ Failed to list projects: {}", e);
        }
    }
    
    // Check users and teams
    println!("👥 Checking users and teams...");
    let mut user_count = 0;
    match user_repo.list_users(odi_core::user::UserQuery::default()).await {
        Ok(users) => {
            for user in users {
                user_count += 1;
                objects_checked += 1;
                
                if verbose {
                    println!("  ✓ User: {} ({})", user.name, user.id);
                }
                
                // Validate user fields
                if user.name.trim().is_empty() {
                    issues_found += 1;
                    println!("  ⚠️  User {} has empty name", user.id);
                }
                
                if !user.email.contains('@') {
                    issues_found += 1;
                    println!("  ⚠️  User {} has invalid email: {}", user.id, user.email);
                }
            }
            println!("  📊 Checked {} users", user_count);
        },
        Err(e) => {
            issues_found += 1;
            println!("  ✗ Failed to list users: {}", e);
        }
    }
    
    let mut team_count = 0;
    match user_repo.list_teams(odi_core::user::TeamQuery::default()).await {
        Ok(teams) => {
            for team in teams {
                team_count += 1;
                objects_checked += 1;
                
                if verbose {
                    println!("  ✓ Team: {} ({})", team.name, team.id);
                }
                
                // Validate team fields
                if team.name.trim().is_empty() {
                    issues_found += 1;
                    println!("  ⚠️  Team {} has empty name", team.id);
                }
            }
            println!("  📊 Checked {} teams", team_count);
        },
        Err(e) => {
            issues_found += 1;
            println!("  ✗ Failed to list teams: {}", e);
        }
    }
    
    // Summary
    println!("\n📋 Integrity Check Summary:");
    println!("  Objects checked: {}", objects_checked);
    println!("  Issues found: {}", issues_found);
    
    if issues_found == 0 {
        println!("✅ Object store is healthy!");
    } else {
        println!("⚠️  Found {} integrity issues", issues_found);
        if !dry_run {
            println!("💡 Run 'odi fsck repair' to attempt automatic fixes");
        }
    }
    
    if issues_found > 0 {
        return Err(crate::OdiError::Validation { 
            message: format!("Found {} integrity issues", issues_found) 
        });
    }
    
    Ok(())
}

/// T079: Check object store structure and integrity
async fn check_object_store_structure(
    ctx: &AppContext, 
    issues_found: &mut u32, 
    objects_checked: &mut u32, 
    verbose: bool
) -> Result<()> {
    use std::path::Path;
    
    
    let storage = ctx.storage();
    
    // Check .odi directory structure
    let odi_path = Path::new(".odi");
    if !odi_path.exists() {
        *issues_found += 1;
        println!("  ✗ .odi directory does not exist");
        return Ok(());
    }
    
    // Check objects directory
    let objects_path = odi_path.join("objects");
    if !objects_path.exists() {
        *issues_found += 1;
        println!("  ✗ .odi/objects directory does not exist");
        return Ok(());
    }
    
    // Check config file
    let config_path = odi_path.join("config");
    if !config_path.exists() {
        *issues_found += 1;
        println!("  ⚠️  .odi/config file does not exist");
    }
    
    // Scan objects directory for .bin files
    if let Ok(entries) = std::fs::read_dir(&objects_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("bin") {
                    *objects_checked += 1;
                    
                    // Check if file can be read
                    match std::fs::read(&path) {
                        Ok(data) => {
                            // Check if data is valid (not empty, reasonable size)
                            if data.is_empty() {
                                *issues_found += 1;
                                println!("  ⚠️  Empty object file: {:?}", path);
                            } else if data.len() > 10 * 1024 * 1024 { // 10MB limit
                                *issues_found += 1;
                                println!("  ⚠️  Unusually large object file: {:?} ({} bytes)", path, data.len());
                            } else {
                                // Try to deserialize as different object types
                                let mut valid_object = false;
                                
                                // Try Issue
                                if let Ok(_) = bincode::deserialize::<odi_core::Issue>(&data) {
                                    valid_object = true;
                                    if verbose {
                                        println!("    ✓ Valid Issue object: {:?}", path);
                                    }
                                }
                                // Try Project 
                                else if let Ok(_) = bincode::deserialize::<odi_core::Project>(&data) {
                                    valid_object = true;
                                    if verbose {
                                        println!("    ✓ Valid Project object: {:?}", path);
                                    }
                                }
                                // Try User
                                else if let Ok(_) = bincode::deserialize::<odi_core::User>(&data) {
                                    valid_object = true;
                                    if verbose {
                                        println!("    ✓ Valid User object: {:?}", path);
                                    }
                                }
                                // Try Remote
                                else if let Ok(_) = bincode::deserialize::<odi_core::Remote>(&data) {
                                    valid_object = true;
                                    if verbose {
                                        println!("    ✓ Valid Remote object: {:?}", path);
                                    }
                                }
                                
                                if !valid_object {
                                    *issues_found += 1;
                                    println!("  ⚠️  Corrupted or unknown object type: {:?}", path);
                                }
                            }
                        },
                        Err(e) => {
                            *issues_found += 1;
                            println!("  ✗ Cannot read object file {:?}: {}", path, e);
                        }
                    }
                }
            }
        }
    } else {
        *issues_found += 1;
        println!("  ✗ Cannot read objects directory");
    }
    
    // Check for orphaned files (non-.bin files in objects directory)
    if let Ok(entries) = std::fs::read_dir(&objects_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|ext| ext.to_str()) != Some("bin") {
                    *issues_found += 1;
                    println!("  ⚠️  Orphaned file in objects directory: {:?}", path);
                }
            }
        }
    }
    
    println!("  📊 Scanned objects directory: {} objects", *objects_checked);
    
    Ok(())
}

async fn repair_objects(_ctx: &AppContext, _force: bool, _verbose: bool) -> Result<()> {
    println!("🔧 Object repair functionality not yet implemented");
    println!("💡 This feature will automatically fix common object store issues");
    println!("   - Orphaned references");
    println!("   - Corrupted timestamps");  
    println!("   - Missing object files");
    println!("   - Invalid field values");
    
    Err(crate::OdiError::Command { 
        message: "Repair functionality is not yet implemented".to_string() 
    })
}

async fn show_storage_stats(ctx: &AppContext, detailed: bool) -> Result<()> {
    println!("📊 ODI Storage Statistics");
    println!();
    
    let issue_repo = ctx.issue_repository();
    let project_repo = ctx.project_repository();
    let user_repo = ctx.user_repository();
    
    // Count objects by type
    let issue_count = match issue_repo.list(odi_core::issue::IssueQuery::default()).await {
        Ok(issues) => issues.len(),
        Err(_) => 0,
    };
    
    let project_count = match project_repo.list_projects(odi_core::project::ProjectQuery::default()).await {
        Ok(projects) => projects.len(),
        Err(_) => 0,
    };
    
    let user_count = match user_repo.list_users(odi_core::user::UserQuery::default()).await {
        Ok(users) => users.len(),
        Err(_) => 0,
    };
    
    let team_count = match user_repo.list_teams(odi_core::user::TeamQuery::default()).await {
        Ok(teams) => teams.len(),
        Err(_) => 0,
    };
    
    println!("📈 Object Counts:");
    println!("  Issues:   {}", issue_count);
    println!("  Projects: {}", project_count);
    println!("  Users:    {}", user_count);
    println!("  Teams:    {}", team_count);
    println!("  Total:    {}", issue_count + project_count + user_count + team_count);
    
    if detailed {
        println!("\n📋 Issue Breakdown:");
        let issues = issue_repo.list(odi_core::issue::IssueQuery::default()).await
            .unwrap_or_default();
        
        let open_count = issues.iter().filter(|i| i.status == odi_core::IssueStatus::Open).count();
        let closed_count = issues.iter().filter(|i| i.status == odi_core::IssueStatus::Closed).count();
        let in_progress_count = issues.iter().filter(|i| i.status == odi_core::IssueStatus::InProgress).count();
        
        println!("  Open:        {}", open_count);
        println!("  In Progress: {}", in_progress_count); 
        println!("  Closed:      {}", closed_count);
        
        let high_priority = issues.iter().filter(|i| i.priority == odi_core::Priority::High).count();
        let medium_priority = issues.iter().filter(|i| i.priority == odi_core::Priority::Medium).count();
        let low_priority = issues.iter().filter(|i| i.priority == odi_core::Priority::Low).count();
        
        println!("\n🎯 Priority Breakdown:");
        println!("  High:   {}", high_priority);
        println!("  Medium: {}", medium_priority);
        println!("  Low:    {}", low_priority);
    }
    
    // Try to estimate disk usage (this is a rough approximation)
    println!("\n💾 Estimated Storage:");
    let avg_issue_size = 250; // Rough estimate based on our binary serialization
    let avg_project_size = 150;
    let avg_user_size = 120;
    let avg_team_size = 100;
    
    let total_bytes = (issue_count * avg_issue_size) + 
                     (project_count * avg_project_size) +
                     (user_count * avg_user_size) +
                     (team_count * avg_team_size);
    
    if total_bytes < 1024 {
        println!("  Size: {} bytes", total_bytes);
    } else if total_bytes < 1024 * 1024 {
        println!("  Size: {:.1} KB", total_bytes as f64 / 1024.0);
    } else {
        println!("  Size: {:.1} MB", total_bytes as f64 / (1024.0 * 1024.0));
    }
    
    Ok(())
}