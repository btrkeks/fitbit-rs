//! Functionality for retrieving Fitbit API access tokens.
//!
//! This module provides functions to read the Fitbit access token from a configuration file
//! located in the user's home directory.

use ini::Ini;
use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur when retrieving an access token
#[derive(Error, Debug)]
pub enum AccessTokenError {
    /// The user's home directory could not be determined
    #[error("Home directory not found")]
    HomeDirectoryNotFound,

    /// Failed to load or parse the configuration file
    #[error("Failed to load config file: {0}")]
    ConfigLoadError(#[from] ini::Error),

    /// The access token was not found in the configuration file
    #[error("ACCESS_TOKEN not found in config.ini")]
    AccessTokenNotFound,

    /// The configuration directory or file could not be created
    #[error("Failed to create config directory or file: {0}")]
    ConfigCreationError(std::io::Error),
}

/// Returns the path to the configuration file
///
/// The configuration file is located at `~/.config/fitbit-rs/config.ini`.
pub fn get_config_path() -> Result<PathBuf, AccessTokenError> {
    dirs::home_dir()
        .ok_or(AccessTokenError::HomeDirectoryNotFound)
        .map(|home| home.join(".config").join("fitbit-rs").join("config.ini"))
}

/// Retrieves the Fitbit API access token from the configuration file
///
/// The access token is expected to be stored in the `[Fitbit]` section under the key
/// `ACCESS_TOKEN` in the configuration file at `~/.config/lifestats/config.ini`.
///
/// # Returns
///
/// The access token as a string if found, otherwise an error.
///
/// # Errors
///
/// Returns an error if:
/// - The home directory could not be determined
/// - The configuration file could not be loaded or parsed
/// - The access token was not found in the configuration file
///
/// # Example
///
/// ```no_run
/// use fitbit_rs::access_token::get_access_token;
///
/// match get_access_token() {
///     Ok(token) => println!("Found access token: {}", token),
///     Err(err) => eprintln!("Error getting access token: {}", err),
/// }
/// ```
pub fn get_access_token() -> Result<String, AccessTokenError> {
    let config_path = get_config_path()?;

    // Load the config if it exists
    let config = Ini::load_from_file(&config_path)?;

    // Retrieve the access token
    config
        .get_from(Some("Fitbit"), "ACCESS_TOKEN")
        .ok_or(AccessTokenError::AccessTokenNotFound)
        .map(String::from)
}

/// Stores a Fitbit API access token in the configuration file
///
/// Creates the configuration file and directory if they don't exist.
///
/// # Arguments
///
/// * `access_token` - The access token to store
///
/// # Returns
///
/// `Ok(())` if successful, otherwise an error.
///
/// # Errors
///
/// Returns an error if:
/// - The home directory could not be determined
/// - The configuration directory could not be created
/// - The configuration file could not be written
pub fn store_access_token(access_token: &str) -> Result<(), AccessTokenError> {
    let config_path = get_config_path()?;

    // Create parent directories if they don't exist
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent).map_err(AccessTokenError::ConfigCreationError)?;
    }

    // Load existing config or create a new one
    let mut config = Ini::load_from_file(&config_path).unwrap_or_else(|_| Ini::new());

    // Set the access token
    config
        .with_section(Some("Fitbit"))
        .set("ACCESS_TOKEN", access_token);

    // Save the config
    config
        .write_to_file(&config_path)
        .map_err(AccessTokenError::ConfigCreationError)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_store_and_retrieve_access_token() {
        // TODO: Make the original code more testable
        // Create a temporary directory for testing
        let temp_dir = tempdir().unwrap();
        let config_dir = temp_dir.path().join(".config").join("lifestats");
        fs::create_dir_all(&config_dir).unwrap();

        // Mock the home directory
        let original_home = env::var("HOME").ok();
        unsafe {
            env::set_var("HOME", temp_dir.path());
        }

        // Test token
        let test_token = "test_access_token_123";

        // Store the token
        store_access_token(test_token).unwrap();

        // Retrieve the token
        let retrieved_token = get_access_token().unwrap();

        // Verify
        assert_eq!(retrieved_token, test_token);

        // Restore original home directory
        if let Some(home) = original_home {
            unsafe {
                env::set_var("HOME", home);
            }
        } else {
            unsafe {
                env::remove_var("HOME");
            }
        }
    }
}
