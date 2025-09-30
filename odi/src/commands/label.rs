//! Label command implementation

use clap::{Args, Subcommand};
use crate::{Result, AppContext};
use odi_core::{Label, LabelQuery, ProjectRepository};

#[derive(Args)]
pub struct LabelArgs {
    #[command(subcommand)]
    pub command: LabelSubcommand,
}

#[derive(Subcommand)]
pub enum LabelSubcommand {
    /// Create a new label in a project
    Create { 
        /// Label ID (unique identifier)
        id: String, 
        /// Label display name
        name: String,
        /// Label color in hex format (#RRGGBB)
        color: String,
        /// Project ID to create label in
        #[arg(short, long)]
        project: String,
        /// Optional description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// List labels in a project or all projects
    List {
        /// Project ID to list labels from (optional, lists all if not specified)
        #[arg(short, long)]
        project: Option<String>,
        /// Filter by color
        #[arg(short, long)]
        color: Option<String>,
    },
}

impl LabelArgs {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.command {
            LabelSubcommand::Create { id, name, color, project, description } => {
                create_label(ctx, id, name, color, project, description.as_deref()).await
            },
            LabelSubcommand::List { project, color } => {
                list_labels(ctx, project.as_deref(), color.as_deref()).await
            },
        }
    }
}

async fn create_label(ctx: &AppContext, id: &str, name: &str, color: &str, project_id: &str, description: Option<&str>) -> Result<()> {
    // Validate label inputs
    if !Label::validate_id(id) {
        return Err(crate::OdiError::Validation { 
            message: format!("Invalid label ID '{}': must be 1-50 characters, alphanumeric with ._- allowed", id)
        });
    }
    
    if !Label::validate_name(name) {
        return Err(crate::OdiError::Validation { 
            message: format!("Invalid label name '{}': must be 1-50 characters", name)
        });
    }
    
    if !Label::validate_color(color) {
        return Err(crate::OdiError::Validation { 
            message: format!("Invalid color format '{}': must be #RRGGBB (e.g., #FF0000)", color)
        });
    }
    
    if let Some(desc) = description {
        if !Label::validate_description(desc) {
            return Err(crate::OdiError::Validation { 
                message: format!("Description too long: maximum 200 characters")
            });
        }
    }
    
    // Get project repository
    let project_repo = ctx.project_repository();
    
    // Check if project exists
    let project = project_repo.get_project(&project_id.to_string()).await.map_err(|e| crate::OdiError::Storage { 
        message: format!("Failed to get project: {}", e) 
    })?;
    
    if project.is_none() {
        return Err(crate::OdiError::Validation { 
            message: format!("Project not found: {}", project_id)
        });
    }
    
    // Check if label already exists in project
    let existing_label = project_repo.get_label(&project_id.to_string(), &id.to_string()).await.map_err(|e| crate::OdiError::Storage { 
        message: format!("Failed to check if label exists: {}", e) 
    })?;
    
    if existing_label.is_some() {
        return Err(crate::OdiError::Validation { 
            message: format!("Label already exists in project '{}': {}", project_id, id)
        });
    }
    
    // Create new label
    let label = if let Some(desc) = description {
        Label::with_description(id.to_string(), name.to_string(), color.to_string(), desc.to_string())
    } else {
        Label::new(id.to_string(), name.to_string(), color.to_string())
    };
    
    // Store label
    project_repo.create_label(&project_id.to_string(), label).await.map_err(|e| crate::OdiError::Storage { 
        message: format!("Failed to create label: {}", e) 
    })?;
    
    println!("Created label '{}' in project '{}':", id, project_id);
    println!("  Name: {}", name);
    println!("  Color: {}", color);
    if let Some(desc) = description {
        println!("  Description: {}", desc);
    }
    
    Ok(())
}

async fn list_labels(ctx: &AppContext, project_id: Option<&str>, color_filter: Option<&str>) -> Result<()> {
    let project_repo = ctx.project_repository();
    
    let labels = if let Some(proj_id) = project_id {
        // List labels for specific project
        let project = project_repo.get_project(&proj_id.to_string()).await.map_err(|e| crate::OdiError::Storage { 
            message: format!("Failed to get project: {}", e) 
        })?;
        
        if project.is_none() {
            return Err(crate::OdiError::Validation { 
                message: format!("Project not found: {}", proj_id)
            });
        }
        
        let mut query = LabelQuery::new();
        if let Some(color) = color_filter {
            query = query.color(color.to_string());
        }
        
        project_repo.list_labels(&proj_id.to_string(), query).await.map_err(|e| crate::OdiError::Storage { 
            message: format!("Failed to list labels: {}", e) 
        })?
    } else {
        // List all labels across projects
        let mut query = LabelQuery::new();
        if let Some(color) = color_filter {
            query = query.color(color.to_string());
        }
        
        project_repo.get_all_labels(query).await.map_err(|e| crate::OdiError::Storage { 
            message: format!("Failed to list all labels: {}", e) 
        })?
    };
    
    if labels.is_empty() {
        if let Some(proj_id) = project_id {
            println!("No labels found in project '{}'.", proj_id);
            println!("Use 'odi label create <id> <name> <color> --project {}' to create a label.", proj_id);
        } else {
            println!("No labels found.");
            println!("Use 'odi label create <id> <name> <color> --project <project-id>' to create a label.");
        }
        return Ok(());
    }
    
    if let Some(proj_id) = project_id {
        println!("Labels in project '{}':", proj_id);
    } else {
        println!("All labels:");
    }
    println!();
    println!("ID           NAME                      COLOR     DESCRIPTION");
    println!("--           ----                      -----     -----------");
    
    for label in labels {
        let description = label.description.as_deref().unwrap_or("");
        let description_display = if description.len() > 30 {
            format!("{}...", &description[..27])
        } else {
            description.to_string()
        };
        
        // Show color indicator if supported by terminal
        let color_indicator = if label.is_dark_color() {
            format!("{} (dark)", label.color)
        } else {
            format!("{} (light)", label.color)
        };
        
        println!(
            "{:<12} {:<24} {:<9} {}", 
            label.id,
            label.name,
            color_indicator,
            description_display
        );
    }
    
    Ok(())
}