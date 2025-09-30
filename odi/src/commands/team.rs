//! Team command implementation

use clap::{Args, Subcommand};
use crate::{Result, AppContext};
use odi_core::{Team, TeamId, UserRepository};

#[derive(Args)]
pub struct TeamArgs {
    #[command(subcommand)]
    pub command: TeamSubcommand,
}

#[derive(Subcommand)]
pub enum TeamSubcommand {
    /// Create a new team
    Create { 
        /// Team name
        name: String,
        /// Team description
        #[arg(long, short)]
        description: Option<String>,
    },
    /// List all teams
    List,
}

impl TeamArgs {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.command {
            TeamSubcommand::Create { name, description } => {
                let team_id = format!("team-{}", uuid::Uuid::new_v4().to_string()[0..8].to_string());
                let mut team = Team::new(team_id, name.clone());
                
                // Set description if provided
                if let Some(desc) = description {
                    team.description = Some(desc.clone());
                }
                
                let created_team = ctx.user_repository().create_team(team).await
                    .map_err(crate::OdiError::Core)?;
                
                println!("Created team: {} ({})", created_team.name, created_team.id);
                Ok(())
            },
            TeamSubcommand::List => {
                let teams = ctx.user_repository().list_teams(odi_core::TeamQuery::default()).await
                    .map_err(crate::OdiError::Core)?;
                
                if teams.is_empty() {
                    println!("No teams found.");
                } else {
                    println!("Teams:");
                    for team in teams {
                        println!("  {} - {}", team.name, team.description.as_deref().unwrap_or("No description"));
                    }
                }
                Ok(())
            },
        }
    }
}
