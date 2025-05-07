//! # Models
//!
//! This module contains all the data structures used throughout the application.

mod azure;
mod config;
mod error;
mod kindle;

pub use azure::TokenResponse;
pub use config::Config;
pub use error::KindleError;

// These types are available for other modules but not currently used publicly
pub(crate) use kindle::{Attachment, Body, Email, EmailAddress, Message, Recipient};
