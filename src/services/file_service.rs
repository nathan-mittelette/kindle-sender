//! # File Management Service
//!
//! This module provides services for working with files and directories in the filesystem.

use std::fs;
use std::path::Path;

use crate::models::KindleError;

/// Service for managing files in the filesystem
pub struct FileService {}

impl FileService {
    /// Create a new instance of FileService
    ///
    /// # Returns
    ///
    /// * `Self` - A new FileService instance
    pub fn new() -> Self {
        FileService {}
    }

    /// List all files in a directory
    ///
    /// # Arguments
    ///
    /// * `directory` - Path to the directory to scan
    ///
    /// # Returns
    ///
    /// * `Result<Vec<String>, KindleError>` - List of file paths or an error
    pub fn list_file_in_directory(&self, directory: &str) -> Result<Vec<String>, KindleError> {
        let mut files = Vec::new();
        let paths = std::fs::read_dir(directory).map_err(|e| KindleError {
            message: format!("Error reading directory: {}", e),
        })?;

        for path in paths {
            let path = path.map_err(|e| KindleError {
                message: format!("Error reading path: {}", e),
            })?;
            if path.path().is_file() {
                files.push(path.path().to_string_lossy().to_string());
            }
        }
        Ok(files)
    }

    /// Move a file from one location to another
    ///
    /// # Arguments
    ///
    /// * `source` - Source file path
    /// * `destination_dir` - Destination directory path
    ///
    /// # Returns
    ///
    /// * `Result<(), KindleError>` - Success or an error
    pub fn move_file<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        source: P,
        destination_dir: Q,
    ) -> Result<(), KindleError> {
        // Create destination directory if it doesn't exist
        fs::create_dir_all(&destination_dir).map_err(|e| KindleError {
            message: format!("Failed to create destination directory: {}", e),
        })?;

        // Get filename from the source path
        let filename = source
            .as_ref()
            .file_name()
            .ok_or_else(|| KindleError {
                message: "Invalid source path: no filename".to_string(),
            })?
            .to_string_lossy()
            .to_string();

        // Create the full destination path
        let destination = destination_dir.as_ref().join(filename);

        // Move the file
        fs::rename(&source, &destination).map_err(|e| KindleError {
            message: format!(
                "Failed to move file from {:?} to {:?}: {}",
                source.as_ref(),
                destination,
                e
            ),
        })?;

        Ok(())
    }
}
