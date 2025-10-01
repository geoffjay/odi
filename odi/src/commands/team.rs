//! Team command implementation

use clap::{Args, Subcommand};
use crate::{Result, AppContext};
use odi_core::{Team, TeamId, UserId, User, UserRepository};
use chrono::Utc;

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
    /// Show team details
    Show {
        /// Team name
        name: String,
    },
    /// Delete a team
    Delete {
        /// Team name
        name: String,
    },
    /// Add member to team
    #[command(name = "add-member")]
    AddMember {
        /// Team name
        team_name: String,
        /// User name to add
        user_name: String,
    },
    /// Remove member from team
    #[command(name = "remove-member")]
    RemoveMember {
        /// Team name
        team_name: String,
        /// User name to remove
        user_name: String,
    },
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
            TeamSubcommand::Show { name } => {
                // Find team by name
                let teams = ctx.user_repository().list_teams(odi_core::TeamQuery::default()).await
                    .map_err(crate::OdiError::Core)?;
                
                if let Some(team) = teams.iter().find(|t| t.name == *name) {
                    println!("Team: {}", team.name);
                    println!("ID: {}", team.id);
                    if let Some(desc) = &team.description {
                        println!("Description: {}", desc);
                    }
                    
                    // Get team members
                    let members = ctx.user_repository().get_team_members(&team.id).await
                        .map_err(crate::OdiError::Core)?;
                    
                    if members.is_empty() {
                        println!("Members: None");
                    } else {
                        println!("Members:");
                        for member in members {
                            println!("  - {}", member.name);
                        }
                    }
                } else {
                    return Err(crate::OdiError::TeamNotFound(name.clone()));
                }
                Ok(())
            },
            TeamSubcommand::Delete { name } => {
                // Find team by name
                let teams = ctx.user_repository().list_teams(odi_core::TeamQuery::default()).await
                    .map_err(crate::OdiError::Core)?;
                
                if let Some(team) = teams.iter().find(|t| t.name == *name) {
                    ctx.user_repository().delete_team(&team.id).await
                        .map_err(crate::OdiError::Core)?;
                    println!("Deleted team: {}", name);
                } else {
                    return Err(crate::OdiError::TeamNotFound(name.clone()));
                }
                Ok(())
            },
            TeamSubcommand::AddMember { team_name, user_name } => {
                // Find team by name
                let teams = ctx.user_repository().list_teams(odi_core::TeamQuery::default()).await
                    .map_err(crate::OdiError::Core)?;
                
                if let Some(team) = teams.iter().find(|t| t.name == *team_name) {
                    // Find or create user
                    let user = match ctx.user_repository().get_user_by_email(user_name).await
                        .map_err(crate::OdiError::Core)? {
                        Some(user) => user,
                        None => {
                            // Create user if doesn't exist
                            let user_id = format!("user-{}", uuid::Uuid::new_v4().to_string()[0..8].to_string());
                            let user = User::new(user_id, user_name.clone(), user_name.clone());
                            ctx.user_repository().create_user(user).await
                                .map_err(crate::OdiError::Core)?
                        }
                    };
                    
                    ctx.user_repository().add_team_member(
                        &team.id, 
                        &user.id
                    ).await.map_err(crate::OdiError::Core)?;
                    
                    println!("Added {} to team {}", user_name, team_name);
                } else {
                    return Err(crate::OdiError::TeamNotFound(team_name.clone()));
                }
                Ok(())
            },
            TeamSubcommand::RemoveMember { team_name, user_name } => {
                // Find team by name
                let teams = ctx.user_repository().list_teams(odi_core::TeamQuery::default()).await
                    .map_err(crate::OdiError::Core)?;
                
                if let Some(team) = teams.iter().find(|t| t.name == *team_name) {
                    // Find user
                    if let Some(user) = ctx.user_repository().get_user_by_email(user_name).await
                        .map_err(crate::OdiError::Core)? {
                        
                        ctx.user_repository().remove_team_member(
                            &team.id, 
                            &user.id
                        ).await.map_err(crate::OdiError::Core)?;
                        
                        println!("Removed {} from team {}", user_name, team_name);
                    } else {
                        return Err(crate::OdiError::UserNotFound(user_name.clone()));
                    }
                } else {
                    return Err(crate::OdiError::TeamNotFound(team_name.clone()));
                }
                Ok(())
            },
        }
    }
}
