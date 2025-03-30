//! Error types for the Fitbit client.
//!
//! This module defines the various error types that can occur when interacting with
//! the Fitbit API.

use thiserror::Error;

/// Errors that can occur when interacting with the Fitbit API
#[derive(Error, Debug)]
pub enum FitbitError {
    /// Error occurring during HTTP request
    #[error("Request failed: {0}")]
    RequestError(#[from] ureq::Error),

    /// Error parsing JSON response
    #[error("JSON parsing failed: {0}")]
    JsonError(String),

    /// API rate limit exceeded
    #[error("Rate limit exceeded - retry after {0} seconds")]
    RateLimitExceeded(u64),

    /// Authentication error
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    /// API responded with an error
    #[error("API error: {status_code} - {message}")]
    ApiError {
        /// HTTP status code
        status_code: u16,
        /// Error message from the API
        message: String,
    },

    /// Client configuration error
    #[error("Client configuration error: {0}")]
    ConfigurationError(String),

    /// Error retrieving or using access token
    #[error("Access token error: {0}")]
    AccessTokenError(#[from] crate::access_token::AccessTokenError),
}

/// Helper functions for working with Fitbit errors
impl FitbitError {
    /// Creates a new API error from a status code and message
    ///
    /// # Arguments
    ///
    /// * `status_code` - HTTP status code
    /// * `message` - Error message
    ///
    /// # Returns
    ///
    /// A new `FitbitError::ApiError`
    pub fn api_error(status_code: u16, message: impl Into<String>) -> Self {
        FitbitError::ApiError {
            status_code,
            message: message.into(),
        }
    }

    /// Creates a new authentication error
    ///
    /// # Arguments
    ///
    /// * `message` - Error message
    ///
    /// # Returns
    ///
    /// A new `FitbitError::AuthenticationError`
    pub fn authentication_error(message: impl Into<String>) -> Self {
        FitbitError::AuthenticationError(message.into())
    }

    /// Checks if the error is a rate limit error
    ///
    /// # Returns
    ///
    /// `true` if the error is a rate limit error, `false` otherwise
    pub fn is_rate_limit(&self) -> bool {
        matches!(self, FitbitError::RateLimitExceeded(_))
    }

    /// Checks if the error is an authentication error
    ///
    /// # Returns
    ///
    /// `true` if the error is an authentication error, `false` otherwise
    pub fn is_authentication_error(&self) -> bool {
        matches!(self, FitbitError::AuthenticationError(_))
    }

    /// Checks if the error is a client configuration error
    ///
    /// # Returns
    ///
    /// `true` if the error is a client configuration error, `false` otherwise
    pub fn is_configuration_error(&self) -> bool {
        matches!(self, FitbitError::ConfigurationError(_))
    }
}
