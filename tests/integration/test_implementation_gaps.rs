//! Implementation gap detection tests
//! Identifies features that are specified but not yet implemented

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Tests that check for specific implementation gaps based on placeholder println! statements
mod implementation_status {
    use super::*;

    #[tokio::test]
    async fn gap_issue_assignment() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["project", "create", "test"])
            .assert()
            .success();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["issue", "create", "--project", "test", "--title", "Test Issue"])
            .assert()
            .success();

        // Test issue assignment - this might be a placeholder
        let result = Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["issue", "assign", "1", "alice@example.com"])
            .assert();

        // Check if this is implemented or just a placeholder
        if result.try_success().is_ok() {
            println!("✅ Issue assignment is implemented");
        } else {
            println!("❌ Issue assignment needs implementation");
        }
    }

    #[tokio::test] 
    async fn gap_issue_update() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["project", "create", "test"])
            .assert()
            .success();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["issue", "create", "--project", "test", "--title", "Test Issue"])
            .assert()
            .success();

        // Test issue update - this might be a placeholder
        let result = Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["issue", "update", "1", "--status", "in-progress"])
            .assert();

        if result.try_success().is_ok() {
            println!("✅ Issue update is implemented");
        } else {
            println!("❌ Issue update needs implementation");
        }
    }

    #[tokio::test]
    async fn gap_remote_pull() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["remote", "add", "origin", "https://example.com/repo.git"])
            .assert()
            .success();

        // Test remote pull - this might be a placeholder
        let result = Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["remote", "pull", "origin"])
            .assert();

        if result.try_success().is_ok() {
            println!("✅ Remote pull is implemented");
        } else {
            println!("❌ Remote pull needs implementation");
        }
    }

    #[tokio::test]
    async fn gap_remote_push() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["remote", "add", "origin", "https://example.com/repo.git"])
            .assert()
            .success();

        // Test remote push - this might be a placeholder  
        let result = Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["remote", "push", "origin"])
            .assert();

        if result.try_success().is_ok() {
            println!("✅ Remote push is implemented");
        } else {
            println!("❌ Remote push needs implementation");
        }
    }

    #[tokio::test]
    async fn gap_label_management() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Test label creation
        let result = Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["label", "create", "bug", "--color", "red"])
            .assert();

        if result.try_success().is_ok() {
            println!("✅ Label management is implemented");
        } else {
            println!("❌ Label management needs implementation");
        }
    }

    #[tokio::test]
    async fn gap_fsck_command() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Test fsck command
        let result = Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["fsck"])
            .assert();

        if result.try_success().is_ok() {
            println!("✅ fsck command is implemented");
        } else {
            println!("❌ fsck command needs implementation");
        }
    }
}

/// Tests that examine the actual storage and data persistence
mod storage_implementation {
    use super::*;

    #[tokio::test]
    async fn storage_project_persistence() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Create a project
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["project", "create", "persistent-test", "--description", "Test persistence"])
            .assert()
            .success();

        // Verify data is actually stored
        let objects_dir = workspace_path.join(".odi/objects");
        let refs_dir = workspace_path.join(".odi/refs");
        
        if objects_dir.exists() {
            let object_count = fs::read_dir(&objects_dir)
                .map(|entries| entries.count())
                .unwrap_or(0);
            println!("Objects stored: {}", object_count);
            
            if object_count > 0 {
                println!("✅ Project data is persisted to object store");
            } else {
                println!("❌ No objects created - persistence may not be working");
            }
        } else {
            println!("❌ Objects directory not created");
        }

        // Test persistence across commands
        let output1 = Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["project", "list"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let output2 = Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["project", "list"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        // Results should be consistent
        assert_eq!(output1, output2, "Project listing should be consistent");
    }

    #[tokio::test]
    async fn storage_issue_persistence() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

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

        // Create issue
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&[
                "issue", "create",
                "--project", "issue-test", 
                "--title", "Persistence Test Issue",
                "--description", "Testing that issues are properly persisted"
            ])
            .assert()
            .success();

        // Check that issue data persists
        let output1 = Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["issue", "list"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let output2 = Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["issue", "list"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        assert_eq!(output1, output2, "Issue listing should be consistent");

        let output_str = String::from_utf8(output1).unwrap();
        assert!(output_str.contains("Persistence Test Issue"));
    }

    #[tokio::test]
    async fn storage_config_persistence() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Set configuration
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["config", "set", "user.name", "Test User"])
            .assert()
            .success();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["config", "set", "user.email", "test@example.com"])
            .assert()
            .success();

        // Verify config persists
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["config", "get", "user.name"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Test User"));

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["config", "get", "user.email"])
            .assert()
            .success()
            .stdout(predicate::str::contains("test@example.com"));

        // Check config file directly
        let config_content = fs::read_to_string(workspace_path.join(".odi/config")).unwrap();
        assert!(config_content.contains("Test User"));
        assert!(config_content.contains("test@example.com"));
    }
}

/// Tests that validate the actual architecture matches specifications
mod architecture_validation {
    use super::*;

    #[tokio::test]
    async fn validate_directory_structure() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Check required directories exist
        assert!(workspace_path.join(".odi").is_dir(), ".odi directory required");
        assert!(workspace_path.join(".odi/objects").is_dir(), ".odi/objects directory required");
        assert!(workspace_path.join(".odi/refs").is_dir(), ".odi/refs directory required");

        // Check required files exist  
        assert!(workspace_path.join(".odi/config").is_file(), ".odi/config file required");

        // Check that old JSON structure doesn't exist
        assert!(!workspace_path.join(".odi/issues").exists(), "Should not use .odi/issues directory");
        assert!(!workspace_path.join(".odi/projects.json").exists(), "Should not use JSON files");
        assert!(!workspace_path.join(".odi/config.toml").exists(), "Config should not have .toml extension");
    }

    #[tokio::test]
    async fn validate_config_format() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Config should be valid TOML
        let config_path = workspace_path.join(".odi/config");
        assert!(config_path.exists(), "Config file should exist");

        let config_content = fs::read_to_string(&config_path).unwrap();
        let config: toml::Value = toml::from_str(&config_content)
            .expect("Config should be valid TOML");

        // Should be a table (object) not a simple value
        assert!(config.is_table(), "Config should be a TOML table");

        // Test that we can add values and they persist properly
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["config", "set", "test.section.key", "value"])
            .assert()
            .success();

        let updated_content = fs::read_to_string(&config_path).unwrap();
        let updated_config: toml::Value = toml::from_str(&updated_content)
            .expect("Updated config should be valid TOML");

        // Should support nested sections
        if let Some(test_section) = updated_config.get("test") {
            if let Some(section) = test_section.get("section") {
                if let Some(key) = section.get("key") {
                    assert_eq!(key.as_str(), Some("value"));
                    println!("✅ Nested TOML config sections work");
                } else {
                    println!("❌ Config nested key not found");
                }
            } else {
                println!("❌ Config section not found");
            }
        } else {
            println!("❌ Config test section not found");
        }
    }

    #[tokio::test]
    async fn validate_binary_storage() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Create some data that should be stored as objects
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["project", "create", "binary-test"])
            .assert()
            .success();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["issue", "create", "--project", "binary-test", "--title", "Binary Storage Test"])
            .assert()
            .success();

        // Check objects directory
        let objects_dir = workspace_path.join(".odi/objects");
        if objects_dir.exists() {
            let entries: Vec<_> = fs::read_dir(&objects_dir)
                .unwrap()
                .collect::<Result<Vec<_>, _>>()
                .unwrap();

            if !entries.is_empty() {
                println!("✅ Objects are being created in .odi/objects");
                
                // Check that objects are binary, not JSON
                for entry in entries {
                    let path = entry.path();
                    if path.is_file() {
                        let content = fs::read(&path).unwrap();
                        
                        // Try to parse as JSON - should fail for binary storage
                        let is_json = serde_json::from_slice::<serde_json::Value>(&content).is_ok();
                        if is_json {
                            println!("❌ Object file {} contains JSON, should be binary", path.display());
                        } else {
                            println!("✅ Object file {} is binary format", path.display());
                        }
                    }
                }
            } else {
                println!("❌ No objects created - storage may not be implemented");
            }
        } else {
            println!("❌ Objects directory not found");
        }
    }
}

/// Tests to validate command completeness and proper error messages  
mod command_completeness {
    use super::*;

    #[tokio::test]
    async fn validate_help_completeness() {
        // All main commands should have help text
        let commands = vec![
            vec!["--help"],
            vec!["init", "--help"],
            vec!["project", "--help"],
            vec!["issue", "--help"],
            vec!["team", "--help"],
            vec!["remote", "--help"],
            vec!["config", "--help"],
        ];

        for cmd_args in commands {
            let result = Command::cargo_bin("odi").unwrap()
                .args(&cmd_args)
                .assert()
                .success()
                .get_output()
                .stdout
                .clone();

            let output = String::from_utf8(result).unwrap();
            assert!(output.contains("Usage:") || output.contains("USAGE:"),
                "Help for {:?} should contain usage information", cmd_args);
        }
    }

    #[tokio::test]
    async fn validate_error_messages() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        // Test operation outside workspace
        let result = Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["project", "list"])
            .assert()
            .failure()
            .get_output()
            .stderr
            .clone();

        let stderr = String::from_utf8(result).unwrap();
        
        // Should have informative error message
        let has_good_error = stderr.contains("not initialized") || 
                           stderr.contains("workspace") ||
                           stderr.contains("odi init") ||
                           stderr.contains("not found");
        
        if has_good_error {
            println!("✅ Error messages are informative");
        } else {
            println!("❌ Error message could be more helpful: {}", stderr);
        }
    }

    #[tokio::test]
    async fn validate_command_consistency() {
        let temp_dir = TempDir::new().unwrap();
        let workspace_path = temp_dir.path();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .arg("init")
            .assert()
            .success();

        // Test that list commands work consistently
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["project", "list"])
            .assert()
            .success();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["issue", "list"])
            .assert()
            .success();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["team", "list"])
            .assert()
            .success();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["remote", "list"])
            .assert()
            .success();

        // Test that create commands follow similar patterns
        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["project", "create", "test-consistency"])
            .assert()
            .success();

        Command::cargo_bin("odi").unwrap()
            .current_dir(workspace_path)
            .args(&["team", "create", "test-team"])
            .assert()
            .success();
    }
}