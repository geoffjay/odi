//! T010: Contract test for odi-core Project/Workspace entities
//!
//! Tests Project, Workspace, and Label entities with relationships.
//! This test MUST FAIL initially as per Constitutional Principle I (TDD).

use odi_core::{Label, LabelId, Project, ProjectId, Workspace, WorkspaceId};
use serde_json;
use std::path::PathBuf;

#[test]
fn test_project_creation() {
    // Test basic project creation
    let project = Project::new("backend_api".to_string(), "Backend API Service".to_string());
    
    assert_eq!(project.id, "backend_api");
    assert_eq!(project.name, "Backend API Service");
    assert!(project.description.is_none());
    assert!(project.issues.is_empty());
    assert!(project.teams.is_empty());
    assert!(project.labels.is_empty());
    assert!(project.workspaces.is_empty());
}

#[test]
fn test_project_serialization() {
    // Test Project serialization to/from JSON
    let mut project = Project::new("frontend_app".to_string(), "Frontend Application".to_string());
    project.description = Some("React-based user interface".to_string());
    project.teams.push("frontend_team".to_string());
    project.workspaces.push("main_workspace".to_string());
    
    // Serialize to JSON
    let json = serde_json::to_string(&project).expect("Should serialize to JSON");
    assert!(json.contains("Frontend Application"));
    assert!(json.contains("React-based user interface"));
    assert!(json.contains("frontend_team"));
    
    // Deserialize from JSON
    let deserialized: Project = serde_json::from_str(&json).expect("Should deserialize from JSON");
    assert_eq!(deserialized.name, project.name);
    assert_eq!(deserialized.description, project.description);
    assert_eq!(deserialized.teams, project.teams);
}

#[test]
fn test_project_validation_rules() {
    // Test project ID validation (3-100 characters, alphanumeric + ._-)
    let valid_ids = vec![
        "api",           // 3 chars minimum
        "backend-api",   // with dash
        "frontend.app",  // with dot
        "mobile_client", // with underscore
    ];
    
    for id in valid_ids {
        let project = Project::new(id.to_string(), "Test Project".to_string());
        assert_eq!(project.id, id);
    }
    
    // Test project name length (1-100 characters)
    let project1 = Project::new("short".to_string(), "A".to_string());
    assert_eq!(project1.name.len(), 1);
    
    let long_name = "A".repeat(100);
    let project2 = Project::new("long".to_string(), long_name.clone());
    assert_eq!(project2.name.len(), 100);
}

#[test]
fn test_workspace_creation() {
    // Test basic workspace creation
    let path = PathBuf::from("/home/user/projects/myproject");
    let workspace = Workspace::new("workspace_1".to_string(), path.clone());
    
    assert_eq!(workspace.id, "workspace_1");
    assert_eq!(workspace.path, path);
    assert!(workspace.projects.is_empty());
}

#[test]
fn test_workspace_serialization() {
    // Test Workspace serialization to/from JSON
    let path = PathBuf::from("/path/to/workspace");
    let mut workspace = Workspace::new("dev_workspace".to_string(), path.clone());
    workspace.projects.push("project1".to_string());
    workspace.projects.push("project2".to_string());
    
    // Serialize to JSON
    let json = serde_json::to_string(&workspace).expect("Should serialize to JSON");
    assert!(json.contains("dev_workspace"));
    assert!(json.contains("/path/to/workspace"));
    assert!(json.contains("project1"));
    
    // Deserialize from JSON
    let deserialized: Workspace = serde_json::from_str(&json).expect("Should deserialize from JSON");
    assert_eq!(deserialized.id, workspace.id);
    assert_eq!(deserialized.path, workspace.path);
    assert_eq!(deserialized.projects, workspace.projects);
}

#[test]
fn test_workspace_path_handling() {
    // Test different path formats
    let paths = vec![
        PathBuf::from("/absolute/unix/path"),
        PathBuf::from("relative/path"),
        PathBuf::from("./current/dir"),
        PathBuf::from("../parent/dir"),
    ];
    
    for (i, path) in paths.iter().enumerate() {
        let workspace = Workspace::new(format!("workspace_{}", i), path.clone());
        assert_eq!(workspace.path, *path);
    }
}

#[test]
fn test_label_creation() {
    // Test basic label creation
    let label = Label::new(
        "bug".to_string(),
        "Bug Report".to_string(),
        "#FF0000".to_string(),
    );
    
    assert_eq!(label.id, "bug");
    assert_eq!(label.name, "Bug Report");
    assert_eq!(label.color, "#FF0000");
    assert!(label.description.is_none());
}

#[test]
fn test_label_serialization() {
    // Test Label serialization to/from JSON
    let mut label = Label::new(
        "enhancement".to_string(),
        "Enhancement".to_string(),
        "#00FF00".to_string(),
    );
    label.description = Some("Feature improvements and additions".to_string());
    
    // Serialize to JSON
    let json = serde_json::to_string(&label).expect("Should serialize to JSON");
    assert!(json.contains("Enhancement"));
    assert!(json.contains("#00FF00"));
    assert!(json.contains("Feature improvements"));
    
    // Deserialize from JSON
    let deserialized: Label = serde_json::from_str(&json).expect("Should deserialize from JSON");
    assert_eq!(deserialized.name, label.name);
    assert_eq!(deserialized.color, label.color);
    assert_eq!(deserialized.description, label.description);
}

#[test]
fn test_label_color_validation() {
    // Test hex color format validation
    let valid_colors = vec![
        "#FF0000", // Red
        "#00FF00", // Green  
        "#0000FF", // Blue
        "#FFFFFF", // White
        "#000000", // Black
        "#A1B2C3", // Mixed case should be handled
    ];
    
    for color in valid_colors {
        let label = Label::new("test".to_string(), "Test Label".to_string(), color.to_string());
        assert_eq!(label.color, color);
        
        // Verify hex format (6 chars after #)
        assert_eq!(label.color.len(), 7);
        assert!(label.color.starts_with('#'));
    }
}

#[test]
fn test_project_workspace_relationships() {
    // Test many-to-many relationship between projects and workspaces
    let mut project1 = Project::new("shared_project".to_string(), "Shared Project".to_string());
    let mut project2 = Project::new("local_project".to_string(), "Local Project".to_string());
    
    let mut workspace1 = Workspace::new("workspace1".to_string(), PathBuf::from("/ws1"));
    let mut workspace2 = Workspace::new("workspace2".to_string(), PathBuf::from("/ws2"));
    
    // Project can be in multiple workspaces
    project1.workspaces.push(workspace1.id.clone());
    project1.workspaces.push(workspace2.id.clone());
    
    // Workspace can contain multiple projects
    workspace1.projects.push(project1.id.clone());
    workspace1.projects.push(project2.id.clone());
    workspace2.projects.push(project1.id.clone());
    
    // Verify relationships
    assert_eq!(project1.workspaces.len(), 2);
    assert!(project1.workspaces.contains(&workspace1.id));
    assert!(project1.workspaces.contains(&workspace2.id));
    
    assert_eq!(workspace1.projects.len(), 2);
    assert!(workspace1.projects.contains(&project1.id));
    assert!(workspace1.projects.contains(&project2.id));
    
    assert_eq!(workspace2.projects.len(), 1);
    assert!(workspace2.projects.contains(&project1.id));
}

#[test]
fn test_project_label_management() {
    // Test label management within projects
    let mut project = Project::new("labeled_project".to_string(), "Labeled Project".to_string());
    
    // Add labels
    let bug_label = Label::new("bug".to_string(), "Bug".to_string(), "#FF0000".to_string());
    let feature_label = Label::new("feature".to_string(), "Feature".to_string(), "#00FF00".to_string());
    
    project.labels.push(bug_label.clone());
    project.labels.push(feature_label.clone());
    
    assert_eq!(project.labels.len(), 2);
    assert_eq!(project.labels[0].id, "bug");
    assert_eq!(project.labels[1].id, "feature");
    
    // Remove label
    project.labels.retain(|label| label.id != "bug");
    assert_eq!(project.labels.len(), 1);
    assert_eq!(project.labels[0].id, "feature");
}

#[test]
fn test_entity_timestamps() {
    // Test timestamp behavior for all entities
    let project = Project::new("time_project".to_string(), "Time Project".to_string());
    let workspace = Workspace::new("time_workspace".to_string(), PathBuf::from("/time"));
    let label = Label::new("time_label".to_string(), "Time Label".to_string(), "#123456".to_string());
    
    let now = chrono::Utc::now();
    
    // All entities should have timestamps <= now
    assert!(project.created_at <= now);
    assert!(project.updated_at <= now);
    assert_eq!(project.created_at, project.updated_at);
    
    assert!(workspace.created_at <= now);
    assert!(workspace.updated_at <= now);
    assert_eq!(workspace.created_at, workspace.updated_at);
    
    assert!(label.created_at <= now);
}