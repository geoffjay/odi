//! T016: Contract test for `odi init` command
//!
//! Tests the CLI contract for initializing ODI workspace.
//! This test MUST FAIL initially as per Constitutional Principle I (TDD).

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_odi_init_basic() {
    // Test basic `odi init` command
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");

    cmd.current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialized ODI workspace"));

    // Verify .odi directory was created
    assert!(temp_dir.path().join(".odi").exists());
    assert!(temp_dir.path().join(".odi/config").exists());
    assert!(temp_dir.path().join(".odi/objects").exists());
}

#[test]
fn test_odi_init_with_project_name() {
    // Test `odi init --project <name>` command
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");

    cmd.current_dir(temp_dir.path())
        .args(["init", "--project", "test-project"])
        .assert()
        .success()
        .stdout(predicate::str::contains("test-project"));

    // Verify project configuration
    let config_content = fs::read_to_string(temp_dir.path().join(".odi/config"))
        .expect("Should read config file");
    assert!(config_content.contains("test-project"));
}

#[test]
fn test_odi_init_existing_workspace() {
    // Test init in directory with existing .odi workspace
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    fs::create_dir_all(temp_dir.path().join(".odi")).expect("Should create .odi dir");
    fs::write(temp_dir.path().join(".odi/config"), "[remotes]").expect("Should create config");

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");

    cmd.current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .failure()
        .stderr(predicate::str::contains("ODI workspace already exists"));
}

#[test]
fn test_odi_init_with_git_repo() {
    // Test init in directory with Git repository
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    
    // Initialize git repository
    Command::new("git")
        .args(["init"])
        .current_dir(temp_dir.path())
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");

    cmd.current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Git repository detected"));

    // Verify Git integration is configured
    let config_content = fs::read_to_string(temp_dir.path().join(".odi/config"))
        .expect("Should read config file");
    assert!(config_content.contains("git"));
}

#[test]
fn test_odi_init_force_flag() {
    // Test `odi init --force` to overwrite existing workspace
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    fs::create_dir_all(temp_dir.path().join(".odi")).expect("Should create .odi dir");
    fs::write(temp_dir.path().join(".odi/config"), "old_config").expect("Should create config");

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");

    cmd.current_dir(temp_dir.path())
        .args(["init", "--force"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Overwriting existing workspace"));

    // Verify config was recreated
    let config_content = fs::read_to_string(temp_dir.path().join(".odi/config"))
        .expect("Should read config file");
    assert!(!config_content.contains("old_config"));
}

#[test]
fn test_odi_init_help() {
    // Test `odi init --help` command
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");

    cmd.args(["init", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialize a new ODI workspace"))
        .stdout(predicate::str::contains("--project"))
        .stdout(predicate::str::contains("--force"));
}

#[test]
fn test_odi_init_invalid_project_name() {
    // Test init with invalid project name
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");

    cmd.current_dir(temp_dir.path())
        .args(["init", "--project", ""])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid project name"));
}

#[test]
fn test_odi_init_permissions_error() {
    // Test init in directory without write permissions
    // This test will be platform-specific and may need conditional compilation
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut perms = fs::metadata(temp_dir.path()).unwrap().permissions();
        perms.set_mode(0o444); // Read-only
        fs::set_permissions(temp_dir.path(), perms).expect("Should set permissions");

        let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");

        cmd.current_dir(temp_dir.path())
            .arg("init")
            .assert()
            .failure()
            .stderr(predicate::str::contains("Permission denied"));
    }
}