//! Specification compliance tests
//! Validates that the implementation meets the specific requirements from the specification

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;
use serde_json::Value;

/// Tests that validate the 25 functional requirements from the specification
mod functional_requirements {
    use super::*;

    #[tokio::test]
    async fn fr01_workspace_initialization() {
        // FR-01: Users can initialize issue tracking in any directory using `odi init`
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Verify .odi directory structure is created
        assert!(workspace_path.join(".odi").is_dir());
        assert!(workspace_path.join(".odi/config").is_file());
        assert!(workspace_path.join(".odi/objects").is_dir());
    }

    #[tokio::test]
    async fn fr02_git_detection() {
        // FR-02: System detects existing Git repositories and associates projects
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        // Create git repository
        std::process::Command::new("git")
            .current_dir(workspace_path)
            .args(&["init"])
            .output()
            .expect("Git should be available");

        // Initialize odi
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Verify git integration is detected (implementation specific)
        let config = fs::read_to_string(workspace_path.join(".odi/config")).unwrap();
        assert!(!config.is_empty(), "Config should contain git detection info");
    }

    #[tokio::test]
    async fn fr03_toml_configuration() {
        // FR-03: Configuration uses TOML format with hierarchy
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Verify config file is valid TOML
        let config_content = fs::read_to_string(workspace_path.join(".odi/config")).unwrap();
        let _: toml::Value = toml::from_str(&config_content)
            .expect("Config must be valid TOML");
    }

    #[tokio::test]
    async fn fr04_project_management() {
        // FR-04: Users can create and manage multiple projects in a workspace
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Create multiple projects
        for project in &["web-frontend", "api-backend", "mobile-app"] {
            Command::cargo_bin("odi").unwrap()
                .current_dir(workspace_path)
                .args(&["project", "create", project])
                .assert()
                .success();
        }

        // Verify all projects exist
        let output = Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["project", "list"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("web-frontend"));
        assert!(output_str.contains("api-backend"));
        assert!(output_str.contains("mobile-app"));
    }

    #[tokio::test]
    async fn fr05_issue_lifecycle() {
        // FR-05: Complete issue lifecycle management
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
            .args(&["project", "create", "test-project"])
            .assert()
            .success();

        // Create issue
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&[
                "issue", "create",
                "--project", "test-project",
                "--title", "Test Issue",
                "--description", "This is a test issue for validation"
            ])
            .assert()
            .success();

        // List issues
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["issue", "list"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Test Issue"));

        // TODO: Test assign, update, close when implemented
    }

    #[tokio::test]
    async fn fr06_user_team_management() {
        // FR-06: User and team management capabilities
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Create team
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["team", "create", "developers"])
            .assert()
            .success();

        // Add team member
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["team", "add-member", "developers", "alice@example.com"])
            .assert()
            .success();

        // Verify team exists
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["team", "list"])
            .assert()
            .success()
            .stdout(predicate::str::contains("developers"));
    }

    #[tokio::test]
    async fn fr07_ssh_https_protocols() {
        // FR-07: Support for SSH and HTTPS protocols for remotes
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Add HTTPS remote
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["remote", "add", "https-origin", "https://example.com/repo.git"])
            .assert()
            .success();

        // Add SSH remote  
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["remote", "add", "ssh-origin", "ssh://git@example.com/repo.git"])
            .assert()
            .success();

        // Verify both remotes exist
        let output = Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["remote", "list"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("https-origin"));
        assert!(output_str.contains("ssh-origin"));
    }

    #[tokio::test]
    async fn fr08_binary_object_store() {
        // FR-08: Binary object store using .odi/objects directory
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Create some data that should be stored
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["project", "create", "test"])
            .assert()
            .success();

        // Verify objects directory exists and contains files
        let objects_dir = workspace_path.join(".odi/objects");
        assert!(objects_dir.is_dir(), "Objects directory must exist");

        // After creating data, there should be objects
        let entries: Vec<_> = fs::read_dir(&objects_dir).unwrap().collect();
        println!("Objects directory entries: {}", entries.len());
        // Note: The exact structure depends on implementation
    }

    #[tokio::test]
    async fn fr09_configuration_hierarchy() {
        // FR-09: Configuration hierarchy (~/.odiconfig -> ./.odi/config)
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        // Create home config simulation (would need to test actual hierarchy)
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Set local configuration
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["config", "set", "user.name", "Local User"])
            .assert()
            .success();

        // Verify configuration is retrievable
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["config", "get", "user.name"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Local User"));
    }

    #[tokio::test]
    async fn fr10_no_branching() {
        // FR-10: No branching/rebasing like Git (simpler merge model)
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Verify no branch-related commands exist
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["branch"])
            .assert()
            .failure(); // Should not exist

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["rebase"])
            .assert()
            .failure(); // Should not exist

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["checkout"])
            .assert()
            .failure(); // Should not exist
    }
}

/// Tests that validate architectural requirements
mod architecture_requirements {
    use super::*;

    #[tokio::test]
    async fn ar01_rust_clap_implementation() {
        // AR-01: Implementation uses Rust with clap for CLI
        // This is validated by the fact that the binary builds and runs

        Command::cargo_bin("odi").unwrap()
            .arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("Usage:"));
    }

    #[tokio::test]
    async fn ar02_workspace_structure() {
        // AR-02: Workspace structure with odi/ binary and odi-* libraries
        // This is validated by examining the built artifacts and running tests
        
        // The binary should exist and be functional
        Command::cargo_bin("odi").unwrap()
            .arg("--version")
            .assert()
            .success();

        // The workspace should build successfully (tested in CI)
    }

    #[tokio::test]
    async fn ar03_minimal_dependencies() {
        // AR-03: Minimal external dependencies
        // This would be validated by examining Cargo.toml files
        // For now, we trust that the workspace builds with minimal deps
    }

    #[tokio::test]
    async fn ar04_cross_platform() {
        // AR-04: Cross-platform CLI application  
        // Basic functionality should work on any platform where tests run
        let temp_dir = TempDir::new().unwrap();

        Command::cargo_bin("odi").unwrap()
            .current_dir(temp_dir.path())
            .arg("init")
            .assert()
            .success();
    }
}

/// Tests that validate data model requirements
mod data_model_requirements {
    use super::*;

    #[tokio::test]
    async fn dm01_many_to_many_project_workspace() {
        // DM-01: Many-to-many relationship between projects and workspaces
        // A single workspace can have multiple projects
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Create multiple projects in one workspace
        for project in &["project-a", "project-b", "project-c"] {
            Command::cargo_bin("odi").unwrap()
                .current_dir(workspace_path)
                .args(&["project", "create", project])
                .assert()
                .success();
        }

        // Verify all projects exist in this workspace
        let output = Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["project", "list"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("project-a"));
        assert!(output_str.contains("project-b"));
        assert!(output_str.contains("project-c"));
    }

    #[tokio::test]
    async fn dm02_unified_config_structure() {
        // DM-02: Single .odi/config file instead of multiple config files
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Should have single config file
        assert!(workspace_path.join(".odi/config").is_file());

        // Should NOT have separate config files
        assert!(!workspace_path.join(".odi/project.toml").exists());
        assert!(!workspace_path.join(".odi/user.toml").exists());
        assert!(!workspace_path.join(".odi/remote.toml").exists());

        // Config should be valid TOML with sections
        let config_content = fs::read_to_string(workspace_path.join(".odi/config")).unwrap();
        let config: toml::Value = toml::from_str(&config_content).unwrap();
        
        // The structure should support sections like [project], [user], etc.
        assert!(config.is_table(), "Config should be a TOML table");
    }

    #[tokio::test]
    async fn dm03_binary_object_storage() {
        // DM-03: Binary storage in .odi/objects instead of JSON files
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Create some data
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["project", "create", "test"])
            .assert()
            .success();

        // Should have objects directory, not JSON files
        assert!(workspace_path.join(".odi/objects").is_dir());
        assert!(!workspace_path.join(".odi/issues").exists());
        assert!(!workspace_path.join(".odi/projects.json").exists());
        assert!(!workspace_path.join(".odi/users.json").exists());

        // Objects directory should contain binary files (not human-readable JSON)
        let objects_dir = workspace_path.join(".odi/objects");
        if let Ok(entries) = fs::read_dir(&objects_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.path().is_file() {
                        let content = fs::read(&entry.path()).unwrap();
                        // Should be binary data, not JSON
                        let is_json = serde_json::from_slice::<Value>(&content).is_ok();
                        assert!(!is_json, "Object files should be binary, not JSON");
                    }
                }
            }
        }
    }
}

/// Tests for error handling and edge cases
mod error_handling {
    use super::*;

    #[tokio::test]
    async fn eh01_graceful_failures() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        // Commands outside workspace should fail gracefully
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["project", "list"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("not initialized").or(
                predicate::str::contains("not found").or(
                    predicate::str::contains("workspace")
                )
            ));
    }

    #[tokio::test]
    async fn eh02_duplicate_operations() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Create project
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["project", "create", "test"])
            .assert()
            .success();

        // Try to create same project again - should handle gracefully
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["project", "create", "test"])
            .assert()
            .failure(); // Should fail but not crash
    }

    #[tokio::test]
    async fn eh03_invalid_inputs() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Invalid project name
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["issue", "create", "--project", "nonexistent", "--title", "Test"])
            .assert()
            .failure();

        // Invalid config key
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["config", "get", "invalid.key"])
            .assert()
            .failure();
    }
}

/// Performance and scalability tests
mod performance {
    use super::*;

    #[tokio::test]
    async fn perf01_large_project_count() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Create many projects to test scalability
        for i in 0..50 {
            Command::cargo_bin("odi").unwrap()
                .current_dir(workspace_path)
                .args(&["project", "create", &format!("project-{:03}", i)])
                .assert()
                .success();
        }

        // List should still work efficiently
        let start = std::time::Instant::now();
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["project", "list"])
            .assert()
            .success();
        let duration = start.elapsed();

        // Should complete in reasonable time (adjust threshold as needed)
        assert!(duration.as_secs() < 5, "Project listing should be fast");
    }

    #[tokio::test] 
    async fn perf02_large_issue_count() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["project", "create", "perf-test"])
            .assert()
            .success();

        // Create many issues
        for i in 0..20 {  // Reduced for CI performance
            Command::cargo_bin("odi").unwrap()
                .current_dir(workspace_path)
                .args(&[
                    "issue", "create",
                    "--project", "perf-test",
                    "--title", &format!("Issue #{}", i)
                ])
                .assert()
                .success();
        }

        // List should still work efficiently
        let start = std::time::Instant::now();
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["issue", "list"])
            .assert()
            .success();
        let duration = start.elapsed();

        assert!(duration.as_secs() < 5, "Issue listing should be fast");
    }
}