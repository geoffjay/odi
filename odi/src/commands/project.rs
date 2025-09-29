//! Project command implementation

use clap::{Args, Subcommand};
use crate::Result;

#[derive(Args)]
pub struct ProjectArgs {
    #[command(subcommand)]
    pub command: ProjectSubcommand,
}

#[derive(Subcommand)]
pub enum ProjectSubcommand {
    Create { name: String },
    List,
}

impl ProjectArgs {
    pub async fn execute(&self) -> Result<()> {
        println!("Project command placeholder");
        Ok(())
    }
}
