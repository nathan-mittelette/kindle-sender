//! # Configuration Service
//!
//! This module provides services for loading and managing application configuration.

use crate::models::{Config, KindleError};

/// Service for managing configuration
pub struct ConfigService {}

impl ConfigService {
    /// Create a new ConfigService instance
    #[allow(dead_code)]
    pub fn new() -> Self {
        ConfigService {}
    }

    /// Read and parse the application configuration from the default location
    ///
    /// Attempts to load the application configuration from the default config.json
    /// file in the current directory.
    ///
    /// # Returns
    ///
    /// * `Result<Config, KindleError>` - The loaded configuration or an error
    pub fn read_config() -> Result<Config, KindleError> {
        // Read configuration from the JSON file
        let config_path = "./config.json";
        let config = Config::from_file(config_path).map_err(|e| KindleError {
            message: format!("Error reading configuration file ({}): {}", config_path, e),
        })?;
        Ok(config)
    }
}
