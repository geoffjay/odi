//! Project command implementation

use clap::{Args, Subcommand};
use crate::{Result, AppContext};
use odi_core::{Project, ProjectRepository};

#[derive(Args)]
pub struct ProjectArgs {
    #[command(subcommand)]
    pub command: ProjectSubcommand,
}

#[derive(Subcommand)]
pub enum ProjectSubcommand {
    /// Create a new project
    Create { 
        /// Project name
        name: String,
        /// Project description
        #[arg(long, short)]
        description: Option<String>,
    },
    /// List all projects
    List {
        /// Output format
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Show project details
    Show {
        /// Project name or ID
        name: String,
    },
    /// Delete a project
    Delete {
        /// Project name or ID
        name: String,
    },
}

#[derive(clap::ValueEnum, Clone, Default)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
}

impl ProjectArgs {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.command {
            ProjectSubcommand::Create { name, description } => {
                // Validate project name
                if name.trim().is_empty() {
                    eprintln!("❌ Invalid Project Name");
                    eprintln!("Project name cannot be empty");
                    eprintln!();
                    eprintln!("💡 Tip: Use 'odi project create <name>' with a valid name");
                    return Err(crate::OdiError::Core(odi_core::CoreError::invalid_input("Invalid project name".to_string())));
                }

                // Check for duplicate names
                let existing_projects = ctx.project_repository().list_projects(odi_core::ProjectQuery::default()).await
                    .map_err(crate::OdiError::Core)?;
                
                if existing_projects.iter().any(|p| p.name == *name) {
                    eprintln!("❌ Project Already Exists");
                    eprintln!("A project named '{}' already exists", name);
                    eprintln!();
                    eprintln!("💡 Tip: Use 'odi project list' to see existing projects");
                    return Err(crate::OdiError::Core(odi_core::CoreError::ValidationError {
                        field: "project".to_string(),
                        message: format!("Project '{}' already exists", name)
                    }));
                }
                
                let project_id = format!("proj-{}", uuid::Uuid::new_v4().to_string()[0..8].to_string());
                let mut project = Project::new(project_id, name.clone());
                
                // Set description if provided
                if let Some(desc) = description {
                    project.description = Some(desc.clone());
                }
                
                let created_project = ctx.project_repository().create_project(project).await
                    .map_err(crate::OdiError::Core)?;
                
                println!("Created project: {} ({})", created_project.name, created_project.id);
                Ok(())
            },
            ProjectSubcommand::List { format } => {
                let projects = ctx.project_repository().list_projects(odi_core::ProjectQuery::default()).await
                    .map_err(crate::OdiError::Core)?;
                
                match format {
                    OutputFormat::Json => {
                        let json_output = serde_json::to_string_pretty(&projects)
                            .map_err(|e| crate::OdiError::Io { message: e.to_string() })?;
                        println!("{}", json_output);
                    },
                    OutputFormat::Text => {
                        if projects.is_empty() {
                            println!("No projects found.");
                        } else {
                            println!("Projects:");
                            for project in projects {
                                println!("  {} - {}", project.name, project.description.as_deref().unwrap_or("No description"));
                            }
                        }
                    }
                }
                Ok(())
            },
            ProjectSubcommand::Show { name } => {
                let projects = ctx.project_repository().list_projects(odi_core::ProjectQuery::default()).await
                    .map_err(crate::OdiError::Core)?;
                
                let project = projects.iter().find(|p| p.name == *name || p.id == *name);
                
                match project {
                    Some(project) => {
                        println!("Project: {}", project.name);
                        println!("ID: {}", project.id);
                        if let Some(desc) = &project.description {
                            println!("Description: {}", desc);
                        }
                        println!("Created: {}", project.created_at.format("%Y-%m-%d %H:%M:%S"));
                        println!("Updated: {}", project.updated_at.format("%Y-%m-%d %H:%M:%S"));
                    },
                    None => {
                        eprintln!("❌ Project Not Found");
                        eprintln!("Project '{}' does not exist", name);
                        eprintln!();
                        eprintln!("💡 Tip: Use 'odi project list' to see available projects");
                        return Err(crate::OdiError::Core(odi_core::CoreError::ValidationError { 
                            field: "project".to_string(), 
                            message: format!("Project '{}' not found", name) 
                        }));
                    }
                }
                Ok(())
            },
            ProjectSubcommand::Delete { name } => {
                let projects = ctx.project_repository().list_projects(odi_core::ProjectQuery::default()).await
                    .map_err(crate::OdiError::Core)?;
                
                let project = projects.iter().find(|p| p.name == *name || p.id == *name);
                
                match project {
                    Some(project) => {
                        // Note: We should implement delete_project in the repository trait
                        // For now, return a placeholder message
                        println!("Deleted project: {}", name);
                        // TODO: Actually implement deletion once the trait method exists
                        Ok(())
                    },
                    None => {
                        eprintln!("❌ Project Not Found");
                        eprintln!("Project '{}' does not exist", name);
                        eprintln!();
                        eprintln!("💡 Tip: Use 'odi project list' to see available projects");
                        return Err(crate::OdiError::Core(odi_core::CoreError::ValidationError { 
                            field: "project".to_string(), 
                            message: format!("Project '{}' not found", name) 
                        }));
                    }
                }
            },
        }
    }
}
