//! # Azure Authentication Service
//!
//! This module provides services for authenticating with the Microsoft Azure API,
//! including token acquisition, refresh, and storage.

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use chrono::Utc;
use log::info;
use reqwest::Client;
use tokio::sync::oneshot;
use warp::Filter;

use crate::models::{KindleError, TokenResponse};

/// Service for handling Azure authentication and API operations
pub struct AzureService<'a> {
    /// Azure application client ID
    pub client_id: &'a str,
    /// Azure application client secret
    pub client_secret: &'a str,
    /// Azure tenant ID
    pub tenant_id: &'a str,
    /// OAuth callback URL for redirection after authentication
    pub callback_url: &'a str,
}

impl<'a> AzureService<'a> {
    /// Create a new instance of AzureService
    ///
    /// # Arguments
    ///
    /// * `client_id` - The Azure application client ID
    /// * `client_secret` - The Azure application client secret
    /// * `tenant_id` - The Azure tenant ID
    /// * `callback_url` - The OAuth callback URL
    ///
    /// # Returns
    ///
    /// * `Self` - A new AzureService instance
    pub fn new(
        client_id: &'a str,
        client_secret: &'a str,
        tenant_id: &'a str,
        callback_url: &'a str,
    ) -> Self {
        AzureService {
            client_id,
            client_secret,
            tenant_id,
            callback_url,
        }
    }

    /// Authenticate with Azure and get an access token
    ///
    /// This method will:
    /// 1. Try to use a cached token if it's still valid
    /// 2. Try to refresh the token if it's expired but we have a refresh token
    /// 3. Start a new authentication flow if needed
    ///
    /// # Returns
    ///
    /// * `Result<String, KindleError>` - The access token or an error
    pub async fn authenticate(&self) -> Result<String, KindleError> {
        info!("Authenticating with Azure...");

        let auth_file_path = dirs::home_dir().unwrap().join(".kindle_sender/auth.json");

        // Check if the auth file exists and read the token
        if let Ok(token_response) = Self::read_token_from_file(&auth_file_path) {
            if token_response.is_token_valid() {
                return Ok(token_response.access_token);
            } else if let Some(refresh_token) = &token_response.refresh_token {
                let new_token_response =
                    self.refresh_access_token(refresh_token)
                        .await
                        .map_err(|e| KindleError {
                            message: format!("Error refreshing token: {}", e),
                        })?;
                Self::write_token_to_file(&auth_file_path, &new_token_response).map_err(|e| {
                    KindleError {
                        message: format!("Error writing token to file: {}", e),
                    }
                })?;
                return Ok(new_token_response.access_token);
            }
        }

        let scopes = "offline_access%20Mail.Send";

        let auth_url = format!(
            "https://login.microsoftonline.com/{}/oauth2/v2.0/authorize?client_id={}&response_type=code&redirect_uri={}&response_mode=query&scope={}",
            self.tenant_id, self.client_id, self.callback_url, scopes
        );

        info!(
            "Please open the following URL in your browser:\n{}",
            auth_url
        );

        // Channel to receive the auth code
        let (tx, rx) = oneshot::channel();
        let tx = Arc::new(Mutex::new(Some(tx)));

        // Warp filter to handle the redirect
        let callback_route = warp::path("callback")
            .and(warp::query::<HashMap<String, String>>())
            .map(move |query: HashMap<String, String>| {
                if let Some(code) = query.get("code") {
                    if let Some(tx) = tx.lock().unwrap().take() {
                        tx.send(code.clone()).ok();
                    }
                }
                warp::reply::html("You can close this tab and return to the CLI.")
            });

        // Start the warp server
        tokio::spawn(warp::serve(callback_route).run(([127, 0, 0, 1], 8080)));

        // Wait for the auth code
        let auth_code = rx.await.map_err(|_| KindleError {
            message: "Failed to receive auth code".to_string(),
        })?;

        // Exchange the auth code for a token
        let mut token_response = self
            .exchange_code_for_token(auth_code, self.callback_url)
            .await
            .map_err(|e| KindleError {
                message: format!("Error exchanging code for token: {}", e),
            })?;

        // Calculate the expiration time
        token_response.expires_at = Some(Utc::now().timestamp() + token_response.expires_in as i64);

        // Store the token response
        Self::write_token_to_file(&auth_file_path, &token_response).map_err(|e| KindleError {
            message: format!("Error writing token to file: {}", e),
        })?;

        Ok(token_response.access_token)
    }

    /// Exchange an authorization code for an access token
    ///
    /// # Arguments
    ///
    /// * `auth_code` - Authorization code received from the OAuth redirect
    /// * `redirect_uri` - The redirect URI used in the initial authorization request
    ///
    /// # Returns
    ///
    /// * `Result<TokenResponse, Box<dyn Error>>` - Token response or an error
    async fn exchange_code_for_token(
        &self,
        auth_code: String,
        redirect_uri: &str,
    ) -> Result<TokenResponse, Box<dyn Error>> {
        let client = Client::new();
        let params = [
            ("client_id", self.client_id),
            ("scope", "Mail.Send"),
            ("code", auth_code.as_str()),
            ("redirect_uri", redirect_uri),
            ("grant_type", "authorization_code"),
            ("client_secret", self.client_secret),
        ];

        let res = client
            .post(format!(
                "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
                self.tenant_id
            ))
            .form(&params)
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }

    /// Refresh an access token using a refresh token
    ///
    /// # Arguments
    ///
    /// * `refresh_token` - The refresh token to use
    ///
    /// # Returns
    ///
    /// * `Result<TokenResponse, Box<dyn Error>>` - New token response or an error
    async fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> Result<TokenResponse, Box<dyn Error>> {
        let client = Client::new();
        let params = [
            ("client_id", self.client_id),
            ("scope", "https://graph.microsoft.com/.default"),
            ("refresh_token", refresh_token),
            ("grant_type", "refresh_token"),
            ("client_secret", self.client_secret),
        ];

        let res = client
            .post(format!(
                "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
                self.tenant_id
            ))
            .form(&params)
            .send()
            .await?
            .json::<TokenResponse>()
            .await?;

        Ok(res)
    }

    /// Read a token from a file
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file containing the token
    ///
    /// # Returns
    ///
    /// * `Result<TokenResponse, Box<dyn Error>>` - Token response or an error
    fn read_token_from_file(file_path: &PathBuf) -> Result<TokenResponse, Box<dyn Error>> {
        if file_path.exists() {
            let mut file = File::open(file_path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let token_response: TokenResponse = serde_json::from_str(&contents)?;
            Ok(token_response)
        } else {
            Err("File not found".into())
        }
    }

    /// Write a token to a file
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to write the token to
    /// * `token_response` - Token response to write
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn Error>>` - Success or an error
    fn write_token_to_file(
        file_path: &PathBuf,
        token_response: &TokenResponse,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(parent_dir) = file_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }
        let mut file = File::create(file_path)?;
        let json = serde_json::to_string(token_response)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
