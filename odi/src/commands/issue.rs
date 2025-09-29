//! Issue command implementation

use clap::{Args, Subcommand};
use crate::Result;

#[derive(Args)]
pub struct IssueArgs {
    #[command(subcommand)]
    pub command: IssueSubcommand,
}

#[derive(Subcommand)]  
pub enum IssueSubcommand {
    Create { title: String },
    List,
}

impl IssueArgs {
    pub async fn execute(&self) -> Result<()> {
        println!("Issue command placeholder");
        Ok(())
    }
}
