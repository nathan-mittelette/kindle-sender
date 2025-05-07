//! # Kindle-Sender
//!
//! A Rust CLI application that sends e-book files to Kindle devices via email.
//! Uses Microsoft Azure/Graph API for authentication and email sending capabilities.

mod commands;
mod models;
mod services;

use clap::{Parser, Subcommand};
use env_logger::Env;
use log::LevelFilter;
use log::error;

/// Command-line interface definition for the Kindle-Sender application
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The subcommand to execute
    #[command(subcommand)]
    command: Commands,
}

/// Available commands for the Kindle-Sender application
#[derive(Subcommand, Debug)]
enum Commands {
    /// Send e-book files to the configured Kindle device
    Send {},
}

/// Main entry point for the Kindle-Sender application
///
/// Initializes the logger, parses command-line arguments, and executes
/// the appropriate command.
#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info) // Set default level
        .parse_env(Env::default()) // Still allow override via RUST_LOG
        .init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Send {} => {
            if let Err(e) = commands::execute_send_command().await {
                error!("Command failed: {}", e);
                std::process::exit(1);
            }
        }
    }
}
