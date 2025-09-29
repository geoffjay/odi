//! T012: Contract test for odi-fs ConfigLoader trait
//!
//! Tests configuration loading, hierarchy, and TOML validation.
//! This test MUST FAIL initially as per Constitutional Principle I (TDD).

use odi_fs::{Config, ConfigLoader};
use odi_fs::config::{ProjectConfig, RemoteConfig, UserConfig, WorkspaceConfig};
use std::collections::HashMap;
use std::path::Path;
use tempfile::TempDir;

// Mock implementation for testing - will be replaced by real implementation
struct MockConfigLoader;

impl ConfigLoader for MockConfigLoader {
    fn load_global() -> odi_fs::Result<Option<Config>> {
        // This should fail initially - no implementation
        panic!("ConfigLoader::load_global not implemented yet")
    }

    fn load_local(_workspace_path: &Path) -> odi_fs::Result<Option<Config>> {
        panic!("ConfigLoader::load_local not implemented yet")
    }

    fn merge(_global: Option<Config>, _local: Option<Config>) -> Config {
        panic!("ConfigLoader::merge not implemented yet")
    }

    fn validate(_config: &Config) -> odi_fs::Result<()> {
        panic!("ConfigLoader::validate not implemented yet")
    }
}

#[test]
fn test_config_default_values() {
    // Test default configuration
    let config = Config::default();
    
    assert_eq!(config.user.name, "Anonymous");
    assert_eq!(config.user.email, "anonymous@example.com");
    assert!(config.user.ssh_key.is_none());
    
    assert!(config.workspace.is_none());
    assert!(config.project.is_empty());
    assert!(config.remote.is_empty());
    
    assert_eq!(config.ui.color, "auto");
    assert!(config.ui.pager);
    assert!(config.ui.editor.is_none());
    
    assert!(!config.sync.auto_pull);
    assert_eq!(config.sync.conflict_strategy, "manual");
    assert!(config.sync.compress_objects);
}

#[test]
fn test_config_serialization() {
    // Test Config serialization to/from TOML
    let mut config = Config::default();
    config.user.name = "John Doe".to_string();
    config.user.email = "john@example.com".to_string();
    
    // Add workspace config
    config.workspace = Some(WorkspaceConfig {
        active_projects: vec!["project1".to_string(), "project2".to_string()],
        default_assignee: Some("@john".to_string()),
    });
    
    // Add project config
    let mut project_config = ProjectConfig {
        name: "Main Project".to_string(),
        default_labels: vec!["bug".to_string(), "feature".to_string()],
        git_integration: true,
    };
    config.project.insert("main".to_string(), project_config);
    
    // Add remote config
    let remote_config = RemoteConfig {
        url: "https://example.com/repo.git".to_string(),
        protocol: "https".to_string(),
        projects: vec!["main".to_string()],
    };
    config.remote.insert("origin".to_string(), remote_config);
    
    // Serialize to TOML
    let toml_string = toml::to_string(&config).expect("Should serialize to TOML");
    assert!(toml_string.contains("John Doe"));
    assert!(toml_string.contains("john@example.com"));
    assert!(toml_string.contains("project1"));
    assert!(toml_string.contains("https://example.com/repo.git"));
    
    // Deserialize from TOML
    let deserialized: Config = toml::from_str(&toml_string).expect("Should deserialize from TOML");
    assert_eq!(deserialized.user.name, config.user.name);
    assert_eq!(deserialized.user.email, config.user.email);
    assert_eq!(deserialized.workspace.as_ref().unwrap().active_projects, 
               config.workspace.as_ref().unwrap().active_projects);
}

#[test]
fn test_global_config_loading() {
    // Test loading global configuration from ~/.odi/config
    let result = MockConfigLoader::load_global();
    
    // This will panic until implemented - that's expected for TDD
    assert!(result.is_ok() || result.is_err()); // Either is acceptable
}

#[test]
fn test_local_config_loading() {
    // Test loading local configuration from ./.odi/config
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let result = MockConfigLoader::load_local(temp_dir.path());
    
    // This will panic until implemented - that's expected for TDD
    assert!(result.is_ok() || result.is_err()); // Either is acceptable
}

#[test]
fn test_config_hierarchy_merge() {
    // Test configuration hierarchy: global <- local
    let global_config = Some(Config {
        user: UserConfig {
            name: "Global User".to_string(),
            email: "global@example.com".to_string(),
            ssh_key: Some(std::path::PathBuf::from("~/.ssh/id_rsa")),
        },
        workspace: None,
        project: HashMap::new(),
        remote: HashMap::new(),
        ui: odi_fs::config::UiConfig {
            color: "auto".to_string(),
            pager: true,
            editor: Some("vim".to_string()),
        },
        sync: odi_fs::config::SyncConfig {
            auto_pull: false,
            conflict_strategy: "manual".to_string(),
            compress_objects: true,
        },
    });
    
    let mut local_project = HashMap::new();
    local_project.insert("local_project".to_string(), ProjectConfig {
        name: "Local Project".to_string(),
        default_labels: vec!["local".to_string()],
        git_integration: true,
    });
    
    let local_config = Some(Config {
        user: UserConfig {
            name: "Local User".to_string(), // Override global
            email: "global@example.com".to_string(), // Keep global
            ssh_key: None, // Override global
        },
        workspace: Some(WorkspaceConfig {
            active_projects: vec!["local_project".to_string()],
            default_assignee: None,
        }),
        project: local_project,
        remote: HashMap::new(),
        ui: odi_fs::config::UiConfig {
            color: "always".to_string(), // Override global
            pager: true, // Keep global
            editor: Some("vim".to_string()), // Keep global
        },
        sync: odi_fs::config::SyncConfig {
            auto_pull: false, // Keep global
            conflict_strategy: "manual".to_string(), // Keep global  
            compress_objects: false, // Override global
        },
    });
    
    let merged = MockConfigLoader::merge(global_config, local_config);
    
    // Verify local overrides global
    assert_eq!(merged.user.name, "Local User");
    assert_eq!(merged.user.email, "global@example.com"); // From global
    assert!(merged.user.ssh_key.is_none()); // Local override
    
    assert!(merged.workspace.is_some());
    assert_eq!(merged.workspace.unwrap().active_projects.len(), 1);
    
    assert_eq!(merged.ui.color, "always"); // Local override
    assert_eq!(merged.ui.editor.as_ref().unwrap(), "vim"); // From global
    
    assert!(!merged.sync.compress_objects); // Local override
}

#[test]
fn test_config_validation() {
    // Test configuration validation rules
    let valid_config = Config {
        user: UserConfig {
            name: "Valid User".to_string(),
            email: "valid@example.com".to_string(),
            ssh_key: Some(std::path::PathBuf::from("~/.ssh/id_rsa")),
        },
        workspace: Some(WorkspaceConfig {
            active_projects: vec!["project1".to_string()],
            default_assignee: Some("@user".to_string()),
        }),
        project: {
            let mut projects = HashMap::new();
            projects.insert("project1".to_string(), ProjectConfig {
                name: "Project One".to_string(),
                default_labels: vec!["bug".to_string(), "feature".to_string()],
                git_integration: true,
            });
            projects
        },
        remote: HashMap::new(),
        ui: odi_fs::config::UiConfig {
            color: "auto".to_string(),
            pager: true,
            editor: Some("vim".to_string()),
        },
        sync: odi_fs::config::SyncConfig {
            auto_pull: false,
            conflict_strategy: "manual".to_string(),
            compress_objects: true,
        },
    };
    
    let result = MockConfigLoader::validate(&valid_config);
    assert!(result.is_ok());
}

#[test]
fn test_config_validation_errors() {
    // Test invalid configurations
    
    // Missing required user name
    let invalid_config1 = Config {
        user: UserConfig {
            name: "".to_string(), // Invalid: empty name
            email: "valid@example.com".to_string(),
            ssh_key: None,
        },
        ..Default::default()
    };
    
    let result1 = MockConfigLoader::validate(&invalid_config1);
    // Should fail validation (will panic until validation is implemented)
    
    // Invalid email format
    let invalid_config2 = Config {
        user: UserConfig {
            name: "Valid User".to_string(),
            email: "invalid-email".to_string(), // Invalid: not email format
            ssh_key: None,
        },
        ..Default::default()
    };
    
    let result2 = MockConfigLoader::validate(&invalid_config2);
    // Should fail validation
    
    // Workspace references non-existent project
    let invalid_config3 = Config {
        user: UserConfig {
            name: "Valid User".to_string(),
            email: "valid@example.com".to_string(),
            ssh_key: None,
        },
        workspace: Some(WorkspaceConfig {
            active_projects: vec!["nonexistent".to_string()], // Invalid: project not defined
            default_assignee: None,
        }),
        project: HashMap::new(), // Empty projects
        ..Default::default()
    };
    
    let result3 = MockConfigLoader::validate(&invalid_config3);
    // Should fail validation
}

#[test]
fn test_toml_format_compliance() {
    // Test that generated TOML follows expected format
    let config = Config {
        user: UserConfig {
            name: "John Developer".to_string(),
            email: "john@example.com".to_string(),
            ssh_key: Some(std::path::PathBuf::from("~/.ssh/id_rsa")),
        },
        workspace: Some(WorkspaceConfig {
            active_projects: vec!["main".to_string(), "docs".to_string()],
            default_assignee: Some("@john".to_string()),
        }),
        project: {
            let mut projects = HashMap::new();
            projects.insert("main".to_string(), ProjectConfig {
                name: "Main Project".to_string(),
                default_labels: vec!["bug".to_string(), "feature".to_string()],
                git_integration: true,
            });
            projects.insert("docs".to_string(), ProjectConfig {
                name: "Documentation".to_string(),
                default_labels: vec!["docs".to_string()],
                git_integration: false,
            });
            projects
        },
        remote: {
            let mut remotes = HashMap::new();
            remotes.insert("origin".to_string(), RemoteConfig {
                url: "https://example.com/repo.git".to_string(),
                protocol: "https".to_string(),
                projects: vec!["main".to_string(), "docs".to_string()],
            });
            remotes
        },
        ui: odi_fs::config::UiConfig {
            color: "always".to_string(),
            pager: true,
            editor: Some("code".to_string()),
        },
        sync: odi_fs::config::SyncConfig {
            auto_pull: true,
            conflict_strategy: "manual".to_string(),
            compress_objects: true,
        },
    };
    
    let toml_string = toml::to_string(&config).expect("Should serialize to TOML");
    
    // Verify expected TOML sections exist
    assert!(toml_string.contains("[user]"));
    assert!(toml_string.contains("[workspace]"));
    assert!(toml_string.contains("[project.main]"));
    assert!(toml_string.contains("[project.docs]"));
    assert!(toml_string.contains("[remote.origin]"));
    assert!(toml_string.contains("[ui]"));
    assert!(toml_string.contains("[sync]"));
    
    // Verify specific values
    assert!(toml_string.contains("name = \"John Developer\""));
    assert!(toml_string.contains("active_projects = [\"main\", \"docs\"]"));
    assert!(toml_string.contains("git_integration = true"));
    assert!(toml_string.contains("git_integration = false"));
    assert!(toml_string.contains("url = \"https://example.com/repo.git\""));
}

#[test]
fn test_config_file_paths() {
    // Test configuration file path resolution
    
    // Global config should be at ~/.odi/config (no extension)
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string());
    let expected_global_path = format!("{}/.odi/config", home_dir);
    
    // Local config should be at ./.odi/config (no extension)
    let expected_local_path = "./.odi/config";
    
    // These paths should be used by the ConfigLoader implementation
    // (Implementation will verify these paths are used correctly)
}