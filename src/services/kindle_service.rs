//! # Kindle Email Service
//!
//! This module provides services for sending e-book files to Kindle devices via email
//! using the Microsoft Graph API.

use base64::{Engine as _, engine::general_purpose};
use log::info;
use reqwest::Client;
use std::fs::File;
use std::io::Read;

use crate::models::{Attachment, Body, Email, EmailAddress, KindleError, Message, Recipient};

/// Service for sending e-books to Kindle devices via email
pub struct KindleService<'a> {
    /// List of recipient email addresses (Kindle addresses)
    pub emails: &'a [String],
}

impl<'a> KindleService<'a> {
    /// Create a new instance of KindleService
    ///
    /// # Arguments
    ///
    /// * `emails` - List of recipient email addresses (Kindle addresses)
    ///
    /// # Returns
    ///
    /// * `Self` - A new KindleService instance
    pub fn new(emails: &'a [String]) -> Self {
        KindleService { emails }
    }

    /// Send a file to Kindle devices
    ///
    /// # Arguments
    ///
    /// * `access_token` - Microsoft Graph API access token
    /// * `file_path` - Path to the file to be sent
    ///
    /// # Returns
    ///
    /// * `Result<(), KindleError>` - Success or an error
    pub async fn send_file(
        &self,
        access_token: String,
        file_path: &str,
    ) -> Result<(), KindleError> {
        let client = Client::new();

        // Read the file and encode it in base64
        let mut file = File::open(file_path).map_err(|e| KindleError {
            message: format!("Failed to open file: {}", e),
        })?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| KindleError {
            message: format!("Failed to read file: {}", e),
        })?;
        let content_bytes = general_purpose::STANDARD.encode(&buffer);

        let filename = file_path
            .split('/')
            .next_back()
            .ok_or_else(|| KindleError {
                message: "Failed to get filename from file path".to_string(),
            })?
            .to_string();

        // Create recipients list
        let to_recipients = self
            .emails
            .iter()
            .map(|email| Recipient {
                email_address: EmailAddress {
                    address: email.to_string(),
                },
            })
            .collect();

        // Create attachment
        let attachment = Attachment {
            odata_type: "#microsoft.graph.fileAttachment".to_string(),
            name: filename,
            content_type: "application/octet-stream".to_string(),
            content_bytes,
        };

        // Create email message
        let message = Message {
            subject: "Your Kindle File".to_string(),
            body: Body {
                content_type: "Text".to_string(),
                content: "".to_string(),
            },
            to_recipients,
            attachments: vec![attachment],
        };

        // Create email payload
        let email_payload = Email {
            message,
            save_to_sent_items: true,
        };

        let response = client
            .post("https://graph.microsoft.com/v1.0/me/sendMail")
            .bearer_auth(&access_token)
            .header("Content-Type", "application/json")
            .json(&email_payload)
            .send()
            .await
            .map_err(|e| KindleError {
                message: format!("Failed to send email: {}", e),
            })?;

        if response.status().is_success() {
            info!("Email with attachment sent successfully!");
            Ok(())
        } else {
            let response_status = response.status();
            let message = String::from_utf8(
                response
                    .bytes()
                    .await
                    .map_err(|e| KindleError {
                        message: format!("Failed to read response: {}", e),
                    })?
                    .to_vec(),
            )
            .map_err(|e| KindleError {
                message: format!("Failed to convert response to string: {}", e),
            })?;
            Err(KindleError {
                message: format!("Failed to send email: {:?} {:?}", response_status, message),
            })
        }
    }
}
