//! # Services
//!
//! This module contains all the service components that provide
//! the core functionality of the application.

mod azure_service;
mod config_service;
mod file_service;
mod kindle_service;
mod send_service;

pub use azure_service::AzureService;
pub use config_service::ConfigService;
pub use file_service::FileService;
pub use kindle_service::KindleService;
pub use send_service::SendService;
