//! # Send Service Module
//!
//! This module orchestrates the sending of e-book files to Kindle devices
//! by coordinating between Azure authentication and Kindle email services.

use log::{info, warn};
use std::path::Path;

use crate::models::{Config, KindleError};
use crate::services::{AzureService, FileService, KindleService};

/// Service that coordinates the Azure authentication and Kindle email services
/// to send e-book files to Kindle devices
pub struct SendService<'a> {
    /// Azure service for authentication and API access
    pub azure_service: AzureService<'a>,
    /// Kindle service for sending e-books via email
    pub kindle_service: KindleService<'a>,
    /// File service for file system operations
    pub file_service: FileService,
    /// Configuration for the service
    pub config: &'a Config,
}

impl<'a> SendService<'a> {
    /// Create a new instance of SendService
    ///
    /// # Arguments
    ///
    /// * `azure_service` - The Azure service for authentication
    /// * `kindle_service` - The Kindle service for sending emails
    /// * `config` - Configuration for the service
    ///
    /// # Returns
    ///
    /// * `Self` - A new SendService instance
    pub fn new(
        azure_service: AzureService<'a>,
        kindle_service: KindleService<'a>,
        config: &'a Config,
    ) -> Self {
        SendService {
            azure_service,
            kindle_service,
            file_service: FileService::new(),
            config,
        }
    }

    /// Send e-book files to Kindle devices
    ///
    /// Authenticates with Azure and sends all configured e-book files
    /// to the Kindle email addresses, then moves them to the sent directory.
    ///
    /// # Returns
    ///
    /// * `Result<(), KindleError>` - Success or an error
    pub async fn send_files(&self) -> Result<(), KindleError> {
        info!("Starting file sending process...");

        // List all files in the to-send directory
        let files = self
            .file_service
            .list_file_in_directory(&self.config.ebook_to_send_directory)?;

        if files.is_empty() {
            info!("No files found in directory to send.");
            return Ok(());
        }

        info!("Found {} files to send", files.len());

        // Authenticate with Azure
        let access_token = self.azure_service.authenticate().await?;

        // Send each file and move it to the sent directory
        let mut success_count = 0;
        let mut failure_count = 0;

        for file_path in &files {
            let path = Path::new(&file_path);
            let filename = path
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
                .unwrap_or_else(|| String::from("Unknown file"));

            info!("Sending file: {}", filename);

            match self
                .kindle_service
                .send_file(access_token.clone(), file_path)
                .await
            {
                Ok(_) => {
                    info!("Successfully sent file: {}", filename);

                    // Move file to sent directory
                    match self
                        .file_service
                        .move_file(file_path, &self.config.ebook_sent_directory)
                    {
                        Ok(_) => {
                            info!("Moved file to sent directory: {}", filename);
                            success_count += 1;
                        }
                        Err(e) => {
                            warn!("Failed to move file {}: {}", filename, e.message);
                            failure_count += 1;
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to send file {}: {}", filename, e.message);
                    failure_count += 1;
                }
            }
        }

        info!(
            "Sending process completed. Successfully sent: {}, Failed: {}",
            success_count, failure_count
        );

        if failure_count > 0 {
            return Err(KindleError {
                message: format!("Failed to process {} files", failure_count),
            });
        }

        Ok(())
    }
}
