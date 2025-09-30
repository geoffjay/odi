//! T017: Contract test for `odi project` commands
//!
//! Tests the CLI contract for project management commands.
//! This test MUST FAIL initially as per Constitutional Principle I (TDD).

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
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
fn test_project_create_basic() {
    // Test `odi project create <name>` command
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["project", "create", "my-project"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Created project: my-project"));

    // Verify project was created
    let mut list_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    list_cmd.current_dir(temp_dir.path())
        .args(["project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("my-project"));
}

#[test]
fn test_project_create_with_description() {
    // Test project creation with description
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args([
            "project", "create", "documented-project",
            "--description", "A project with documentation"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("documented-project"));

    // Verify description is stored
    let mut show_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    show_cmd.current_dir(temp_dir.path())
        .args(["project", "show", "documented-project"])
        .assert()
        .success()
        .stdout(predicate::str::contains("A project with documentation"));
}

#[test]
fn test_project_list_empty() {
    // Test listing projects in new workspace
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No projects found"));
}

#[test]
fn test_project_list_multiple() {
    // Test listing multiple projects
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Create multiple projects
    for project_name in ["project-a", "project-b", "project-c"] {
        let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
        cmd.current_dir(temp_dir.path())
            .args(["project", "create", project_name])
            .assert()
            .success();
    }

    // List all projects
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("project-a"))
        .stdout(predicate::str::contains("project-b"))
        .stdout(predicate::str::contains("project-c"));
}

#[test]
fn test_project_show_details() {
    // Test showing project details
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Create project with details
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args([
            "project", "create", "detail-project",
            "--description", "Project with details"
        ])
        .assert()
        .success();

    // Show project details
    let mut show_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    show_cmd.current_dir(temp_dir.path())
        .args(["project", "show", "detail-project"])
        .assert()
        .success()
        .stdout(predicate::str::contains("detail-project"))
        .stdout(predicate::str::contains("Project with details"))
        .stdout(predicate::str::contains("Created:"));
}

#[test]
fn test_project_delete() {
    // Test project deletion
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Create and then delete project
    let mut create_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    create_cmd.current_dir(temp_dir.path())
        .args(["project", "create", "temp-project"])
        .assert()
        .success();

    let mut delete_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    delete_cmd.current_dir(temp_dir.path())
        .args(["project", "delete", "temp-project"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Deleted project: temp-project"));

    // Verify project is no longer listed
    let mut list_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    list_cmd.current_dir(temp_dir.path())
        .args(["project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("temp-project").not());
}

#[test]
fn test_project_duplicate_name() {
    // Test creating project with duplicate name
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Create first project
    let mut cmd1 = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd1.current_dir(temp_dir.path())
        .args(["project", "create", "duplicate"])
        .assert()
        .success();

    // Try to create duplicate
    let mut cmd2 = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd2.current_dir(temp_dir.path())
        .args(["project", "create", "duplicate"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Project already exists"));
}

#[test]
fn test_project_invalid_name() {
    // Test creating project with invalid name
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["project", "create", ""])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid project name"));
}

#[test]
fn test_project_show_nonexistent() {
    // Test showing details of nonexistent project
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["project", "show", "nonexistent"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Project not found"));
}

#[test]
fn test_project_list_format_json() {
    // Test listing projects in JSON format
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Create test project
    let mut create_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    create_cmd.current_dir(temp_dir.path())
        .args(["project", "create", "json-test"])
        .assert()
        .success();

    // List in JSON format
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["project", "list", "--format", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("{"))
        .stdout(predicate::str::contains("json-test"));
}

#[test]
fn test_project_help() {
    // Test project command help
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.args(["project", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Manage projects"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("show"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn test_project_no_workspace() {
    // Test project commands without ODI workspace
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["project", "list"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No ODI workspace found"));
}