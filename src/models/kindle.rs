//! # Kindle Email Models
//!
//! This module defines data structures for sending emails to Kindle devices.

use serde::Serialize;

/// Structure representing an email to be sent via the Graph API
#[derive(Serialize)]
pub struct Email {
    /// The email message content
    pub message: Message,
    /// Whether to save the email in the sent items folder
    #[serde(rename = "saveToSentItems")]
    pub save_to_sent_items: bool,
}

/// Structure representing an email message
#[derive(Serialize)]
pub struct Message {
    /// Email subject line
    pub subject: String,
    /// Email body content
    pub body: Body,
    /// List of email recipients
    #[serde(rename = "toRecipients")]
    pub to_recipients: Vec<Recipient>,
    /// List of file attachments
    pub attachments: Vec<Attachment>,
}

/// Structure representing an email body
#[derive(Serialize)]
pub struct Body {
    /// Content type (e.g., "Text", "HTML")
    #[serde(rename = "contentType")]
    pub content_type: String,
    /// Actual content of the email body
    pub content: String,
}

/// Structure representing an email recipient
#[derive(Serialize)]
pub struct Recipient {
    /// Email address of the recipient
    #[serde(rename = "emailAddress")]
    pub email_address: EmailAddress,
}

/// Structure representing an email address
#[derive(Serialize)]
pub struct EmailAddress {
    /// The email address string
    pub address: String,
}

/// Structure representing an email attachment
#[derive(Serialize)]
pub struct Attachment {
    /// OData type for the attachment (required by Microsoft Graph API)
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
    /// Filename of the attachment
    pub name: String,
    /// MIME type of the attachment
    #[serde(rename = "contentType")]
    pub content_type: String,
    /// Base64-encoded content of the attachment
    #[serde(rename = "contentBytes")]
    pub content_bytes: String,
}
