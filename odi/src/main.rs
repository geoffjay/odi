//! # ODI - Distributed Issue Tracking
//!
//! A Git-like distributed issue tracking system that enables offline-first
//! collaboration on project management with distributed synchronization.

use clap::Parser;

mod cli;
mod commands;
mod error;

use cli::Cli;
use error::OdiError;

type Result<T> = std::result::Result<T, OdiError>;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Initialize logging and error handling
    init_error_handling();

    // Execute the command
    match cli.execute().await {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn init_error_handling() {
    // Set up panic handler for better error reporting
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("ODI encountered an unexpected error:");
        eprintln!("{}", panic_info);
        eprintln!("Please report this issue to: https://github.com/example/odi/issues");
    }));
}