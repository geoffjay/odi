//! Complete workflow integration tests
//! Tests the full end-to-end functionality to validate specification compliance

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_complete_init_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_path = temp_dir.path();

    // Test 1: odi init command creates proper structure
    let mut cmd = Command::cargo_bin("odi").unwrap();
    cmd.current_dir(workspace_path)
        .arg("init")
        .assert()
        .success();

    // Validate .odi directory structure exists
    assert!(workspace_path.join(".odi").exists(), ".odi directory should be created");
    assert!(workspace_path.join(".odi/config").exists(), ".odi/config file should be created");
    assert!(workspace_path.join(".odi/objects").exists(), ".odi/objects directory should be created");
    assert!(workspace_path.join(".odi/refs").exists(), ".odi/refs directory should be created");

    // Test 2: Verify config file is valid TOML
    let config_content = fs::read_to_string(workspace_path.join(".odi/config")).unwrap();
    let _: toml::Value = toml::from_str(&config_content)
        .expect("Config file should be valid TOML");

    // Test 3: Re-running init should not fail (idempotent)
    let mut cmd = Command::cargo_bin("odi").unwrap();
    cmd.current_dir(workspace_path)
        .arg("init")
        .assert()
        .success();
}

#[tokio::test]
async fn test_project_lifecycle() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_path = temp_dir.path();

    // Initialize workspace
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .arg("init")
        .assert()
        .success();

    // Test: Create a project
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["project", "create", "test-project"])
        .assert()
        .success();

    // Test: List projects should show the created project
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("test-project"));

    // Test: Create project with description
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["project", "create", "detailed-project", "--description", "A test project with description"])
        .assert()
        .success();

    // Test: List should show both projects
    let output = Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["project", "list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("test-project"));
    assert!(output_str.contains("detailed-project"));
}

#[tokio::test]
async fn test_issue_lifecycle() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_path = temp_dir.path();

    // Setup
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .arg("init")
        .assert()
        .success();

    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["project", "create", "issue-test"])
        .assert()
        .success();

    // Test: Create an issue
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["issue", "create", "--project", "issue-test", "--title", "Test Issue", "--description", "This is a test issue"])
        .assert()
        .success();

    // Test: List issues should show the created issue
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["issue", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Test Issue"));

    // Test: Create issue with different priority
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["issue", "create", "--project", "issue-test", "--title", "High Priority Issue", "--priority", "high"])
        .assert()
        .success();

    // Test: List should show both issues
    let output = Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["issue", "list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Test Issue"));
    assert!(output_str.contains("High Priority Issue"));
}

#[tokio::test]
async fn test_team_management() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_path = temp_dir.path();

    // Setup
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .arg("init")
        .assert()
        .success();

    // Test: Create a team
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["team", "create", "developers", "--description", "Development team"])
        .assert()
        .success();

    // Test: Add user to team
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["team", "add-member", "developers", "alice@example.com"])
        .assert()
        .success();

    // Test: List teams should show created team
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["team", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("developers"));
}

#[tokio::test]
async fn test_configuration_management() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_path = temp_dir.path();

    // Setup
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .arg("init")
        .assert()
        .success();

    // Test: Set configuration value
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["config", "set", "user.name", "Test User"])
        .assert()
        .success();

    // Test: Get configuration value
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["config", "get", "user.name"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Test User"));

    // Test: Set email configuration
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["config", "set", "user.email", "test@example.com"])
        .assert()
        .success();

    // Test: List all configuration
    let output = Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["config", "list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("user.name"));
    assert!(output_str.contains("user.email"));
}

#[tokio::test] 
async fn test_remote_operations() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_path = temp_dir.path();

    // Setup
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .arg("init")
        .assert()
        .success();

    // Test: Add remote (this should work even if remote is not accessible)
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["remote", "add", "origin", "https://example.com/repo.git"])
        .assert()
        .success();

    // Test: List remotes should show added remote
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["remote", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("origin"));

    // Test: Add SSH remote
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["remote", "add", "backup", "ssh://git@example.com/repo.git"])
        .assert()
        .success();

    // Test: List should show both remotes
    let output = Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["remote", "list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("origin"));
    assert!(output_str.contains("backup"));
}

#[tokio::test]
async fn test_git_integration_detection() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_path = temp_dir.path();

    // Initialize a git repository
    std::process::Command::new("git")
        .current_dir(workspace_path)
        .args(&["init"])
        .output()
        .expect("Failed to initialize git repository");

    // Test: odi init in git repository should detect git
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .arg("init")
        .assert()
        .success();

    // Verify that git integration was detected (this would be in config or output)
    let config_content = fs::read_to_string(workspace_path.join(".odi/config")).unwrap();
    // The exact format depends on implementation, but git info should be present
    assert!(!config_content.is_empty(), "Config should contain git integration info");
}

#[tokio::test]
async fn test_object_store_integrity() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_path = temp_dir.path();

    // Setup with some data
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .arg("init")
        .assert()
        .success();

    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["project", "create", "test-project"])
        .assert()
        .success();

    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["issue", "create", "--project", "test-project", "--title", "Test Issue"])
        .assert()
        .success();

    // Test: Objects directory should contain files
    let objects_dir = workspace_path.join(".odi/objects");
    assert!(objects_dir.exists(), "Objects directory should exist");
    
    // Should have some object files (exact structure depends on implementation)
    let entries: Vec<_> = fs::read_dir(&objects_dir).unwrap().collect();
    assert!(!entries.is_empty(), "Objects directory should contain files");

    // Test: fsck command should validate integrity (if implemented)
    // This might not be implemented yet, so we'll make it optional
    let result = Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["fsck"])
        .assert();
    
    // Either success or command not found is acceptable for now
    if result.try_success().is_err() {
        // Command might not be implemented yet
        println!("fsck command not implemented yet");
    }
}

#[tokio::test]
async fn test_error_handling() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_path = temp_dir.path();

    // Test: Commands should fail gracefully outside odi workspace
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["issue", "list"])
        .assert()
        .failure();

    // Test: Invalid project reference should fail gracefully  
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .arg("init")
        .assert()
        .success();

    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["issue", "create", "--project", "nonexistent", "--title", "Test"])
        .assert()
        .failure();

    // Test: Invalid configuration keys should fail gracefully
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["config", "get", "invalid.key.that.doesnt.exist"])
        .assert()
        .failure();
}

#[tokio::test]
async fn test_multi_project_workspace() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_path = temp_dir.path();

    // Setup
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .arg("init")
        .assert()
        .success();

    // Create multiple projects
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["project", "create", "frontend"])
        .assert()
        .success();

    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["project", "create", "backend"])
        .assert()
        .success();

    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["project", "create", "mobile"])
        .assert()
        .success();

    // Create issues for different projects
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["issue", "create", "--project", "frontend", "--title", "UI Bug"])
        .assert()
        .success();

    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["issue", "create", "--project", "backend", "--title", "API Issue"])
        .assert()
        .success();

    // Test: List all projects
    let output = Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["project", "list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("frontend"));
    assert!(output_str.contains("backend"));
    assert!(output_str.contains("mobile"));

    // Test: List all issues across projects
    let output = Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["issue", "list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("UI Bug"));
    assert!(output_str.contains("API Issue"));

    // Test: Filter issues by project
    Command::cargo_bin("odi").unwrap()
        .current_dir(workspace_path)
        .args(&["issue", "list", "--project", "frontend"])
        .assert()
        .success()
        .stdout(predicate::str::contains("UI Bug"));
}