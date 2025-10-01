//! T019: Contract test for `odi remote` commands
//!
//! Tests the CLI contract for remote management commands.
//! This test MUST FAIL initially as per Constitutional Principle I (TDD).

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn setup_odi_workspace(temp_dir: &TempDir) {
    // Initialize ODI workspace for testing
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();
}

#[test]
fn test_remote_add_https() {
    // Test adding HTTPS remote
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args([
            "remote", "add", "origin",
            "https://github.com/user/repo.git"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added remote: origin"));

    // Verify remote was added
    let mut list_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    list_cmd.current_dir(temp_dir.path())
        .args(["remote", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("origin"))
        .stdout(predicate::str::contains("https://github.com/user/repo.git"));
}

#[test]
fn test_remote_add_ssh() {
    // Test adding SSH remote
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args([
            "remote", "add", "upstream",
            "ssh://git@github.com/user/repo.git"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added remote: upstream"));
}

#[test]
fn test_remote_list_empty() {
    // Test listing remotes when none exist
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["remote", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No remotes configured"));
}

#[test]
fn test_remote_list_multiple() {
    // Test listing multiple remotes
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Add multiple remotes
    let remotes = [
        ("origin", "https://github.com/user/repo.git"),
        ("upstream", "ssh://git@github.com/upstream/repo.git"),
        ("fork", "https://github.com/fork/repo.git"),
    ];

    for (name, url) in remotes.iter() {
        let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
        cmd.current_dir(temp_dir.path())
            .args(["remote", "add", name, url])
            .assert()
            .success();
    }

    // List all remotes
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["remote", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("origin"))
        .stdout(predicate::str::contains("upstream"))
        .stdout(predicate::str::contains("fork"));
}

#[test]
fn test_remote_show_details() {
    // Test showing remote details
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Add remote
    let mut add_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    add_cmd.current_dir(temp_dir.path())
        .args([
            "remote", "add", "detailed",
            "https://github.com/user/repo.git"
        ])
        .assert()
        .success();

    // Show remote details
    let mut show_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    show_cmd.current_dir(temp_dir.path())
        .args(["remote", "show", "detailed"])
        .assert()
        .success()
        .stdout(predicate::str::contains("detailed"))
        .stdout(predicate::str::contains("https://github.com/user/repo.git"))
        .stdout(predicate::str::contains("Protocol: HTTPS"));
}

#[test]
fn test_remote_remove() {
    // Test removing remote
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Add and then remove remote
    let mut add_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    add_cmd.current_dir(temp_dir.path())
        .args([
            "remote", "add", "temporary",
            "https://github.com/temp/repo.git"
        ])
        .assert()
        .success();

    let mut remove_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    remove_cmd.current_dir(temp_dir.path())
        .args(["remote", "remove", "temporary"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed remote: temporary"));

    // Verify remote is no longer listed
    let mut list_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    list_cmd.current_dir(temp_dir.path())
        .args(["remote", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("temporary").not());
}

#[test]
fn test_remote_pull() {
    // Test pulling from remote
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Add remote
    let mut add_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    add_cmd.current_dir(temp_dir.path())
        .args([
            "remote", "add", "origin",
            "https://github.com/user/repo.git"
        ])
        .assert()
        .success();

    // Attempt pull (this will likely fail in testing, but should show proper error handling)
    let mut pull_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    pull_cmd.current_dir(temp_dir.path())
        .args(["pull", "origin"])
        .assert()
        .failure() // Expected to fail in test environment
        .stderr(predicate::str::contains("Failed to connect").or(
            predicate::str::contains("No credentials provided")
        ));
}

#[test]
fn test_remote_push() {
    // Test pushing to remote
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Add remote
    let mut add_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    add_cmd.current_dir(temp_dir.path())
        .args([
            "remote", "add", "origin",
            "https://github.com/user/repo.git"
        ])
        .assert()
        .success();

    // Attempt push (this will likely fail in testing, but should show proper error handling)
    let mut push_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    push_cmd.current_dir(temp_dir.path())
        .args(["push", "origin"])
        .assert()
        .failure() // Expected to fail in test environment
        .stderr(predicate::str::contains("Failed to connect").or(
            predicate::str::contains("No credentials provided")
        ));
}

#[test]
fn test_remote_sync_status() {
    // Test checking synchronization status
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Add remote
    let mut add_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    add_cmd.current_dir(temp_dir.path())
        .args([
            "remote", "add", "origin",
            "https://github.com/user/repo.git"
        ])
        .assert()
        .success();

    // Check sync status
    let mut status_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    status_cmd.current_dir(temp_dir.path())
        .args(["remote", "sync-status", "origin"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Synchronization status"));
}

#[test]
fn test_remote_duplicate_name() {
    // Test adding remote with duplicate name
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Add first remote
    let mut cmd1 = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd1.current_dir(temp_dir.path())
        .args([
            "remote", "add", "duplicate",
            "https://github.com/user/repo1.git"
        ])
        .assert()
        .success();

    // Try to add duplicate
    let mut cmd2 = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd2.current_dir(temp_dir.path())
        .args([
            "remote", "add", "duplicate",
            "https://github.com/user/repo2.git"
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Remote already exists"));
}

#[test]
fn test_remote_invalid_url() {
    // Test adding remote with invalid URL
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["remote", "add", "invalid", "not-a-valid-url"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid remote URL"));
}

#[test]
fn test_remote_nonexistent_show() {
    // Test showing details of nonexistent remote
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["remote", "show", "nonexistent"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Remote not found"));
}

#[test]
fn test_remote_with_authentication() {
    // Test remote operations with authentication
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Add remote with credentials
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args([
            "remote", "add", "auth-remote",
            "https://github.com/user/private-repo.git",
            "--username", "testuser",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added remote: auth-remote"));
}

#[test]
fn test_remote_help() {
    // Test remote command help
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.args(["remote", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Manage remotes"))
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("show"))
        .stdout(predicate::str::contains("remove"))
        .stdout(predicate::str::contains("pull"))
        .stdout(predicate::str::contains("push"));
}

#[test]
fn test_remote_no_workspace() {
    // Test remote commands without ODI workspace
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["remote", "list"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No ODI workspace found"));
}