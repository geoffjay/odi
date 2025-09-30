//! T021: Contract test for `odi config` commands
//!
//! Tests the CLI contract for configuration management commands.
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
fn test_config_get_basic() {
    // Test `odi config get` command
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // First set a value, then get it
    let mut set_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    set_cmd.current_dir(temp_dir.path())
        .args(["config", "set", "user.name", "Test User"])
        .assert()
        .success();

    let mut get_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    get_cmd.current_dir(temp_dir.path())
        .args(["config", "get", "user.name"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Test User"));
}

#[test]
fn test_config_set_basic() {
    // Test `odi config set` command
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["config", "set", "user.email", "test@example.com"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Set user.email = test@example.com"));

    // Verify it was set by reading config file
    let config_content = fs::read_to_string(temp_dir.path().join(".odi/config"))
        .expect("Should read config file");
    assert!(config_content.contains("test@example.com"));
}

#[test]
fn test_config_list_all() {
    // Test listing all configuration values
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Set multiple config values
    let configs = [
        ("user.name", "Test User"),
        ("user.email", "test@example.com"),
        ("ui.color", "true"),
    ];

    for (key, value) in configs.iter() {
        let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
        cmd.current_dir(temp_dir.path())
            .args(["config", "set", key, value])
            .assert()
            .success();
    }

    // List all configs
    let mut list_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    list_cmd.current_dir(temp_dir.path())
        .args(["config", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("user.name"))
        .stdout(predicate::str::contains("user.email"))
        .stdout(predicate::str::contains("ui.color"));
}

#[test]
fn test_config_unset() {
    // Test unsetting configuration value
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Set and then unset value
    let mut set_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    set_cmd.current_dir(temp_dir.path())
        .args(["config", "set", "temp.value", "temporary"])
        .assert()
        .success();

    let mut unset_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    unset_cmd.current_dir(temp_dir.path())
        .args(["config", "unset", "temp.value"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Unset temp.value"));

    // Verify value is no longer set
    let mut get_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    get_cmd.current_dir(temp_dir.path())
        .args(["config", "get", "temp.value"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Configuration key not found"));
}

#[test]
fn test_config_hierarchy_global() {
    // Test global configuration precedence
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    
    // Create fake global config
    let home_dir = temp_dir.path().join("fake_home");
    fs::create_dir_all(&home_dir).expect("Should create fake home dir");
    let global_config_dir = home_dir.join(".odi");
    fs::create_dir_all(&global_config_dir).expect("Should create global .odi dir");
    fs::write(
        global_config_dir.join("config"),
        "[user]\nname = \"Global User\"\nemail = \"global@example.com\""
    ).expect("Should write global config");

    setup_odi_workspace(&temp_dir);

    // Set HOME environment variable and test
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .env("HOME", &home_dir)
        .args(["config", "get", "user.name"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Global User"));
}

#[test]
fn test_config_hierarchy_local_override() {
    // Test local configuration overriding global
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    
    // Create fake global config
    let home_dir = temp_dir.path().join("fake_home");
    fs::create_dir_all(&home_dir).expect("Should create fake home dir");
    let global_config_dir = home_dir.join(".odi");
    fs::create_dir_all(&global_config_dir).expect("Should create global .odi dir");
    fs::write(
        global_config_dir.join("config"),
        "[user]\nname = \"Global User\""
    ).expect("Should write global config");

    setup_odi_workspace(&temp_dir);

    // Set local config to override global
    let mut set_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    set_cmd.current_dir(temp_dir.path())
        .env("HOME", &home_dir)
        .args(["config", "set", "user.name", "Local User"])
        .assert()
        .success();

    // Verify local overrides global
    let mut get_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    get_cmd.current_dir(temp_dir.path())
        .env("HOME", &home_dir)
        .args(["config", "get", "user.name"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Local User"));
}

#[test]
fn test_config_nested_keys() {
    // Test nested configuration keys
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Set nested configuration values
    let nested_configs = [
        ("ui.color.enabled", "true"),
        ("ui.color.scheme", "dark"),
        ("sync.auto.enabled", "false"),
        ("sync.auto.interval", "300"),
    ];

    for (key, value) in nested_configs.iter() {
        let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
        cmd.current_dir(temp_dir.path())
            .args(["config", "set", key, value])
            .assert()
            .success();
    }

    // Verify nested values can be retrieved
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["config", "get", "ui.color.scheme"])
        .assert()
        .success()
        .stdout(predicate::str::contains("dark"));
}

#[test]
fn test_config_list_section() {
    // Test listing configuration values by section
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Set values in different sections
    let configs = [
        ("user.name", "Test User"),
        ("user.email", "test@example.com"),
        ("ui.color", "true"),
        ("ui.format", "table"),
    ];

    for (key, value) in configs.iter() {
        let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
        cmd.current_dir(temp_dir.path())
            .args(["config", "set", key, value])
            .assert()
            .success();
    }

    // List only user section
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["config", "list", "--section", "user"])
        .assert()
        .success()
        .stdout(predicate::str::contains("user.name"))
        .stdout(predicate::str::contains("user.email"))
        .stdout(predicate::str::contains("ui.color").not());
}

#[test]
fn test_config_validation() {
    // Test configuration value validation
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Test invalid boolean value
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["config", "set", "ui.color", "maybe"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid boolean value"));

    // Test invalid email format (if validation is implemented)
    let mut email_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    email_cmd.current_dir(temp_dir.path())
        .args(["config", "set", "user.email", "not-an-email"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid email format"));
}

#[test]
fn test_config_get_nonexistent() {
    // Test getting nonexistent configuration key
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["config", "get", "nonexistent.key"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Configuration key not found"));
}

#[test]
fn test_config_format_json() {
    // Test outputting configuration in JSON format
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Set some values
    let mut set_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    set_cmd.current_dir(temp_dir.path())
        .args(["config", "set", "user.name", "JSON User"])
        .assert()
        .success();

    // List in JSON format
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["config", "list", "--format", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("{"))
        .stdout(predicate::str::contains("JSON User"));
}

#[test]
fn test_config_reset_section() {
    // Test resetting entire configuration section
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_odi_workspace(&temp_dir);

    // Set multiple values in user section
    let configs = [
        ("user.name", "Reset User"),
        ("user.email", "reset@example.com"),
        ("ui.color", "true"), // Different section
    ];

    for (key, value) in configs.iter() {
        let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
        cmd.current_dir(temp_dir.path())
            .args(["config", "set", key, value])
            .assert()
            .success();
    }

    // Reset user section
    let mut reset_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    reset_cmd.current_dir(temp_dir.path())
        .args(["config", "reset", "--section", "user"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Reset section: user"));

    // Verify user configs are gone but ui config remains
    let mut get_user_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    get_user_cmd.current_dir(temp_dir.path())
        .args(["config", "get", "user.name"])
        .assert()
        .failure();

    let mut get_ui_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    get_ui_cmd.current_dir(temp_dir.path())
        .args(["config", "get", "ui.color"])
        .assert()
        .success();
}

#[test]
fn test_config_help() {
    // Test config command help
    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.args(["config", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Manage configuration"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("set"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("unset"));
}

#[test]
fn test_config_no_workspace() {
    // Test config commands without ODI workspace (should work with global config)
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let mut cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    cmd.current_dir(temp_dir.path())
        .args(["config", "set", "user.name", "Global Only"])
        .assert()
        .success(); // Should create global config

    let mut get_cmd = Command::cargo_bin("odi").expect("Failed to find odi binary");
    get_cmd.current_dir(temp_dir.path())
        .args(["config", "get", "user.name"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Global Only"));
}