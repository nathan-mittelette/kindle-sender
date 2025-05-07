//! # Configuration models
//!
//! This module defines the configuration data structures for the application.

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Main configuration structure for the application
///
/// This struct holds all configuration parameters needed by the application,
/// loaded from a JSON configuration file.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// URI for OAuth callback endpoint
    pub callback_uri: String,
    /// Directory path where new e-books to be sent are located
    pub ebook_to_send_directory: String,
    /// Directory path where e-books are moved after being sent
    pub ebook_sent_directory: String,
    /// List of email addresses to send e-books to (Kindle addresses)
    pub receivers: Vec<String>,
    /// Azure API configuration
    pub azure: AzureConfig,
}

/// Azure API configuration parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct AzureConfig {
    /// Azure application client ID
    pub client_id: String,
    /// Azure application client secret
    pub client_secret: String,
    /// Azure tenant ID (often "common" for multi-tenant applications)
    pub tenant_id: String,
}

impl Config {
    /// Load configuration from a JSON file at the specified path
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file
    ///
    /// # Returns
    ///
    /// * `Result<Self, Box<dyn Error>>` - The loaded configuration or an error
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        // Open the file
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Deserialize the JSON
        let config: Config = serde_json::from_reader(reader)?;
        Ok(config)
    }

    /// Display the current configuration settings
    ///
    /// Outputs all configuration values to the console for debugging purposes
    #[allow(dead_code)]
    pub fn display(&self) {
        println!("Configuration:");
        println!("  Callback URI: {}", self.callback_uri);
        println!(
            "  Ebook to send directory: {}",
            self.ebook_to_send_directory
        );
        println!("  Ebook sent directory: {}", self.ebook_sent_directory);
        println!("  Receivers:");
        for (index, email) in self.receivers.iter().enumerate() {
            println!("    {}. {}", index + 1, email);
        }
    }
}
