//! # Error models
//!
//! This module defines the error types used throughout the application.

/// Custom error type for the Kindle-Sender application
#[derive(Debug)]
pub struct KindleError {
    /// Error message describing the problem
    pub message: String,
}

impl std::fmt::Display for KindleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for KindleError {}
