//! T018: Contract test for `odi issue` commands
//!
//! Tests the CLI contract for issue management commands.
//! This test MUST FAIL initially as per Constitutional Principle I (TDD).

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn setup_odi_workspace_with_project(temp_dir: &TempDir) -> &'static str {
    // Initialize ODI workspace and create a test project
    let mut init_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    init_cmd.current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    let mut project_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    project_cmd.current_dir(temp_dir.path())
        .args(["project", "create", "test-project"])
        .assert()
        .success();

    "test-project"
}

#[test]
fn test_issue_create_basic() {
    // Test `odi issue create` command
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = setup_odi_workspace_with_project(&temp_dir);

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args([
            "issue", "create",
            "--project", project_name,
            "--title", "Fix login bug",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Created issue"))
        .stdout(predicate::str::contains("Fix login bug"));
}

#[test]
fn test_issue_create_with_description() {
    // Test issue creation with description
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = setup_odi_workspace_with_project(&temp_dir);

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args([
            "issue", "create",
            "--project", project_name,
            "--title", "Add user authentication",
            "--description", "Implement OAuth2 authentication system",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Add user authentication"));
}

#[test]
fn test_issue_create_with_priority() {
    // Test issue creation with priority
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = setup_odi_workspace_with_project(&temp_dir);

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args([
            "issue", "create",
            "--project", project_name,
            "--title", "Critical security fix",
            "--priority", "critical",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Critical security fix"))
        .stdout(predicate::str::contains("critical"));
}

#[test]
fn test_issue_list_empty() {
    // Test listing issues in project with no issues
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = setup_odi_workspace_with_project(&temp_dir);

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["issue", "list", "--project", project_name])
        .assert()
        .success()
        .stdout(predicate::str::contains("No issues found"));
}

#[test]
fn test_issue_list_multiple() {
    // Test listing multiple issues
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = setup_odi_workspace_with_project(&temp_dir);

    // Create multiple issues
    let issues = [
        ("Bug in login", "high"),
        ("Feature request", "medium"), 
        ("Documentation update", "low"),
    ];

    for (title, priority) in issues.iter() {
        let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
        cmd.current_dir(temp_dir.path())
            .args([
                "issue", "create",
                "--project", project_name,
                "--title", title,
                "--priority", priority,
            ])
            .assert()
            .success();
    }

    // List all issues
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["issue", "list", "--project", project_name])
        .assert()
        .success()
        .stdout(predicate::str::contains("Bug in login"))
        .stdout(predicate::str::contains("Feature request"))
        .stdout(predicate::str::contains("Documentation update"));
}

#[test]
fn test_issue_show_details() {
    // Test showing issue details
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = setup_odi_workspace_with_project(&temp_dir);

    // Create issue first
    let mut create_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    let create_output = create_cmd
        .current_dir(temp_dir.path())
        .args([
            "issue", "create",
            "--project", project_name,
            "--title", "Detailed issue",
            "--description", "This issue has many details",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    // Extract issue ID from output (assuming it's printed)
    let output_str = String::from_utf8_lossy(&create_output);
    // This is a simplified extraction - real implementation would need proper parsing
    let issue_id = "1"; // Placeholder - would extract from output

    // Show issue details
    let mut show_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    show_cmd.current_dir(temp_dir.path())
        .args(["issue", "show", "--project", project_name, issue_id])
        .assert()
        .success()
        .stdout(predicate::str::contains("Detailed issue"))
        .stdout(predicate::str::contains("This issue has many details"));
}

#[test]
fn test_issue_assign() {
    // Test assigning issue to user
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = setup_odi_workspace_with_project(&temp_dir);

    // Create issue first
    let mut create_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    create_cmd.current_dir(temp_dir.path())
        .args([
            "issue", "create",
            "--project", project_name,
            "--title", "Assignable issue",
        ])
        .assert()
        .success();

    // Assign issue (using issue ID 1 as placeholder)
    let mut assign_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    assign_cmd.current_dir(temp_dir.path())
        .args([
            "issue", "assign",
            "--project", project_name,
            "1",
            "--to", "alice",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Assigned issue to alice"));
}

#[test]
fn test_issue_update_status() {
    // Test updating issue status
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = setup_odi_workspace_with_project(&temp_dir);

    // Create issue first
    let mut create_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    create_cmd.current_dir(temp_dir.path())
        .args([
            "issue", "create",
            "--project", project_name,
            "--title", "Status test issue",
        ])
        .assert()
        .success();

    // Update status
    let mut update_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    update_cmd.current_dir(temp_dir.path())
        .args([
            "issue", "update",
            "--project", project_name,
            "1",
            "--status", "in-progress",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Updated issue status"));
}

#[test]
fn test_issue_add_label() {
    // Test adding labels to issue
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = setup_odi_workspace_with_project(&temp_dir);

    // Create issue first
    let mut create_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    create_cmd.current_dir(temp_dir.path())
        .args([
            "issue", "create",
            "--project", project_name,
            "--title", "Labeled issue",
        ])
        .assert()
        .success();

    // Add labels
    let mut label_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    label_cmd.current_dir(temp_dir.path())
        .args([
            "issue", "label",
            "--project", project_name,
            "1",
            "--add", "bug,frontend",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added labels"));
}

#[test]
fn test_issue_list_filter_status() {
    // Test filtering issues by status
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = setup_odi_workspace_with_project(&temp_dir);

    // Create issues with different statuses
    let mut create1 = Command::cargo_bin("odi").expect("Failed to find odi binary");
    create1.current_dir(temp_dir.path())
        .args([
            "issue", "create",
            "--project", project_name,
            "--title", "Open issue",
        ])
        .assert()
        .success();

    let mut create2 = Command::cargo_bin("odi").expect("Failed to find odi binary");
    create2.current_dir(temp_dir.path())
        .args([
            "issue", "create",
            "--project", project_name,
            "--title", "In progress issue",
        ])
        .assert()
        .success();

    // Update second issue status
    let mut update_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    update_cmd.current_dir(temp_dir.path())
        .args([
            "issue", "update",
            "--project", project_name,
            "2",
            "--status", "in-progress",
        ])
        .assert()
        .success();

    // Filter by open status
    let mut filter_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    filter_cmd.current_dir(temp_dir.path())
        .args([
            "issue", "list",
            "--project", project_name,
            "--status", "open",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Open issue"))
        .stdout(predicate::str::contains("In progress issue").not());
}

#[test]
fn test_issue_invalid_project() {
    // Test issue operations with nonexistent project
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let mut init_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    init_cmd.current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args([
            "issue", "create",
            "--project", "nonexistent",
            "--title", "Test issue",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Project not found"));
}

#[test]
fn test_issue_help() {
    // Test issue command help
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.args(["issue", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Manage issues"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("show"))
        .stdout(predicate::str::contains("assign"))
        .stdout(predicate::str::contains("update"));
}

#[test]
fn test_issue_no_workspace() {
    // Test issue commands without ODI workspace
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["issue", "list"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No ODI workspace found"));
}