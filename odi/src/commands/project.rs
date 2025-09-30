//! Project command implementation

use clap::{Args, Subcommand};
use crate::{Result, AppContext};
use odi_core::{Project, ProjectId, ProjectRepository};

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
    List,
}

impl ProjectArgs {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.command {
            ProjectSubcommand::Create { name, description } => {
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
            ProjectSubcommand::List => {
                let projects = ctx.project_repository().list_projects(odi_core::ProjectQuery::default()).await
                    .map_err(crate::OdiError::Core)?;
                
                if projects.is_empty() {
                    println!("No projects found.");
                } else {
                    println!("Projects:");
                    for project in projects {
                        println!("  {} - {}", project.name, project.description.as_deref().unwrap_or("No description"));
                    }
                }
                Ok(())
            },
        }
    }
}
