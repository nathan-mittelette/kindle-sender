//! # Send Command
//!
//! This module implements the "send" command for sending e-books to Kindle devices.

use log::{error, info};

use crate::models::KindleError;
use crate::services::{AzureService, ConfigService, KindleService, SendService};

/// Execute the send command
///
/// This function reads the configuration, initializes the required services,
/// and sends e-book files to the configured Kindle devices.
///
/// # Returns
///
/// * `Result<(), KindleError>` - Success or an error
pub async fn execute_send_command() -> Result<(), KindleError> {
    // Read the configuration
    let config_result = ConfigService::read_config();

    if let Err(e) = config_result {
        error!("Error reading configuration: {}", e.message);
        return Err(e);
    }

    let config = config_result.unwrap();

    // Initialize AzureService with login hints if provided
    let azure_service = if let (Some(login_hint), Some(domain_hint)) = (
        &config.azure.login_hint,
        &config.azure.domain_hint,
    ) {
        AzureService::with_hints(
            &config.azure.client_id,
            &config.azure.client_secret,
            &config.azure.tenant_id,
            &config.callback_uri,
            Some(login_hint),
            Some(domain_hint),
        )
    } else if let Some(login_hint) = &config.azure.login_hint {
        AzureService::with_hints(
            &config.azure.client_id,
            &config.azure.client_secret,
            &config.azure.tenant_id,
            &config.callback_uri,
            Some(login_hint),
            None,
        )
    } else if let Some(domain_hint) = &config.azure.domain_hint {
        AzureService::with_hints(
            &config.azure.client_id,
            &config.azure.client_secret,
            &config.azure.tenant_id,
            &config.callback_uri,
            None,
            Some(domain_hint),
        )
    } else {
        AzureService::new(
            &config.azure.client_id,
            &config.azure.client_secret,
            &config.azure.tenant_id,
            &config.callback_uri,
        )
    };

    // Initialize KindleService
    let kindle_service = KindleService::new(&config.receivers);

    // Initialize SendService
    let send_service = SendService::new(azure_service, kindle_service, &config);

    // Send files
    let result = send_service.send_files().await;

    match result {
        Ok(_) => {
            info!("Files sent successfully!");
            Ok(())
        }
        Err(e) => {
            error!("Error sending files: {}", e.message);
            Err(e)
        }
    }
}
