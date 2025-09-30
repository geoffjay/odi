//! T020: Contract test for `odi team` commands
//!
//! Tests the CLI contract for team management commands.
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
fn test_team_create_basic() {
    // Test `odi team create` command
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["team", "create", "developers"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Created team: developers"));

    // Verify team was created
    let mut list_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    list_cmd.current_dir(temp_dir.path())
        .args(["team", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("developers"));
}

#[test]
fn test_team_create_with_description() {
    // Test team creation with description
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args([
            "team", "create", "qa-team",
            "--description", "Quality Assurance Team"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("qa-team"));

    // Verify description is stored
    let mut show_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    show_cmd.current_dir(temp_dir.path())
        .args(["team", "show", "qa-team"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Quality Assurance Team"));
}

#[test]
fn test_team_list_empty() {
    // Test listing teams when none exist
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["team", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No teams found"));
}

#[test]
fn test_team_list_multiple() {
    // Test listing multiple teams
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Create multiple teams
    let teams = ["frontend", "backend", "devops"];

    for team_name in teams.iter() {
        let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
        cmd.current_dir(temp_dir.path())
            .args(["team", "create", team_name])
            .assert()
            .success();
    }

    // List all teams
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["team", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("frontend"))
        .stdout(predicate::str::contains("backend"))
        .stdout(predicate::str::contains("devops"));
}

#[test]
fn test_team_add_member() {
    // Test adding member to team
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Create team first
    let mut create_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    create_cmd.current_dir(temp_dir.path())
        .args(["team", "create", "developers"])
        .assert()
        .success();

    // Add member to team
    let mut add_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    add_cmd.current_dir(temp_dir.path())
        .args(["team", "add-member", "developers", "alice"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added alice to team developers"));

    // Verify member was added
    let mut show_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    show_cmd.current_dir(temp_dir.path())
        .args(["team", "show", "developers"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alice"));
}

#[test]
fn test_team_add_multiple_members() {
    // Test adding multiple members to team
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Create team
    let mut create_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    create_cmd.current_dir(temp_dir.path())
        .args(["team", "create", "large-team"])
        .assert()
        .success();

    // Add multiple members
    let members = ["alice", "bob", "charlie"];
    for member in members.iter() {
        let mut add_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
        add_cmd.current_dir(temp_dir.path())
            .args(["team", "add-member", "large-team", member])
            .assert()
            .success();
    }

    // Verify all members were added
    let mut show_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    show_cmd.current_dir(temp_dir.path())
        .args(["team", "show", "large-team"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alice"))
        .stdout(predicate::str::contains("bob"))
        .stdout(predicate::str::contains("charlie"));
}

#[test]
fn test_team_remove_member() {
    // Test removing member from team
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Create team and add member
    let mut create_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    create_cmd.current_dir(temp_dir.path())
        .args(["team", "create", "temporary-team"])
        .assert()
        .success();

    let mut add_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    add_cmd.current_dir(temp_dir.path())
        .args(["team", "add-member", "temporary-team", "temp-user"])
        .assert()
        .success();

    // Remove member from team
    let mut remove_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    remove_cmd.current_dir(temp_dir.path())
        .args(["team", "remove-member", "temporary-team", "temp-user"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed temp-user from team temporary-team"));

    // Verify member was removed
    let mut show_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    show_cmd.current_dir(temp_dir.path())
        .args(["team", "show", "temporary-team"])
        .assert()
        .success()
        .stdout(predicate::str::contains("temp-user").not());
}

#[test]
fn test_team_show_details() {
    // Test showing team details
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Create team with details
    let mut create_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    create_cmd.current_dir(temp_dir.path())
        .args([
            "team", "create", "detailed-team",
            "--description", "Team with detailed information"
        ])
        .assert()
        .success();

    // Add some members
    let mut add1_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    add1_cmd.current_dir(temp_dir.path())
        .args(["team", "add-member", "detailed-team", "member1"])
        .assert()
        .success();

    let mut add2_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    add2_cmd.current_dir(temp_dir.path())
        .args(["team", "add-member", "detailed-team", "member2"])
        .assert()
        .success();

    // Show team details
    let mut show_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    show_cmd.current_dir(temp_dir.path())
        .args(["team", "show", "detailed-team"])
        .assert()
        .success()
        .stdout(predicate::str::contains("detailed-team"))
        .stdout(predicate::str::contains("Team with detailed information"))
        .stdout(predicate::str::contains("member1"))
        .stdout(predicate::str::contains("member2"))
        .stdout(predicate::str::contains("Members: 2"));
}

#[test]
fn test_team_delete() {
    // Test deleting team
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Create and then delete team
    let mut create_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    create_cmd.current_dir(temp_dir.path())
        .args(["team", "create", "deletable-team"])
        .assert()
        .success();

    let mut delete_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    delete_cmd.current_dir(temp_dir.path())
        .args(["team", "delete", "deletable-team"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Deleted team: deletable-team"));

    // Verify team is no longer listed
    let mut list_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    list_cmd.current_dir(temp_dir.path())
        .args(["team", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("deletable-team").not());
}

#[test]
fn test_team_duplicate_name() {
    // Test creating team with duplicate name
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Create first team
    let mut cmd1 = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd1.current_dir(temp_dir.path())
        .args(["team", "create", "duplicate"])
        .assert()
        .success();

    // Try to create duplicate
    let mut cmd2 = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd2.current_dir(temp_dir.path())
        .args(["team", "create", "duplicate"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Team already exists"));
}

#[test]
fn test_team_add_duplicate_member() {
    // Test adding duplicate member to team
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Create team and add member
    let mut create_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    create_cmd.current_dir(temp_dir.path())
        .args(["team", "create", "dup-member-team"])
        .assert()
        .success();

    let mut add1_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    add1_cmd.current_dir(temp_dir.path())
        .args(["team", "add-member", "dup-member-team", "duplicate-user"])
        .assert()
        .success();

    // Try to add same member again
    let mut add2_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    add2_cmd.current_dir(temp_dir.path())
        .args(["team", "add-member", "dup-member-team", "duplicate-user"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("User already in team"));
}

#[test]
fn test_team_nonexistent_operations() {
    // Test operations on nonexistent team
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Show nonexistent team
    let mut show_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    show_cmd.current_dir(temp_dir.path())
        .args(["team", "show", "nonexistent"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Team not found"));

    // Add member to nonexistent team
    let mut add_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    add_cmd.current_dir(temp_dir.path())
        .args(["team", "add-member", "nonexistent", "user"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Team not found"));
}

#[test]
fn test_team_help() {
    // Test team command help
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.args(["team", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Manage teams"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("show"))
        .stdout(predicate::str::contains("add-member"))
        .stdout(predicate::str::contains("remove-member"));
}

#[test]
fn test_team_no_workspace() {
    // Test team commands without ODI workspace
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["team", "list"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No ODI workspace found"));
}