//! # Azure Authentication Models
//!
//! This module defines data structures for Azure authentication.

use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Response structure from the OAuth token endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct TokenResponse {
    /// The OAuth access token
    pub access_token: String,
    /// Optional refresh token for obtaining a new access token
    pub refresh_token: Option<String>,
    /// Optional ID token containing user identity information
    pub id_token: Option<String>,
    /// Number of seconds until the access token expires
    pub expires_in: u32,
    /// Type of token, typically "Bearer"
    pub token_type: String,
    /// Optional timestamp when the token will expire (stored locally)
    pub expires_at: Option<i64>,
}

impl TokenResponse {
    /// Check if the current token is still valid
    ///
    /// # Returns
    ///
    /// * `bool` - true if the token is valid, false otherwise
    pub fn is_token_valid(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = Utc::now().timestamp();
            return now < expires_at;
        }
        false
    }
}
