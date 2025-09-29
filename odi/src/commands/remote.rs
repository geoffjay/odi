//! Remote command implementation

use clap::{Args, Subcommand};
use crate::Result;

#[derive(Args)]
pub struct RemoteArgs {
    #[command(subcommand)]
    pub command: RemoteSubcommand,
}

#[derive(Subcommand)]
pub enum RemoteSubcommand {
    Add { name: String, url: String },
    List,
}

impl RemoteArgs {
    pub async fn execute(&self) -> Result<()> {
        println!("Remote command placeholder");
        Ok(())
    }
}
