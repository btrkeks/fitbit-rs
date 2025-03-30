//! Main client for interacting with the Fitbit API.
//!
//! This module provides the `FitbitClient` which handles communication with the Fitbit API,
//! including authentication, request formation, and response parsing.

use crate::activity_summary::ActivitySummaryResponse;
use crate::error::FitbitError;
use crate::sleep::SleepResponseV1_2;
use chrono::NaiveDate;
use std::sync::Arc;
use ureq::Agent;

/// Base URL for the Fitbit API
const API_BASE_URL: &str = "https://api.fitbit.com";

/// API version for sleep endpoints
const SLEEP_API_VERSION: &str = "1.2";

/// API version for activity endpoints
const ACTIVITY_API_VERSION: &str = "1";

/// Trait defining the operations available on a Fitbit client
///
/// This trait abstracts the Fitbit API operations, making it easier to test
/// code that depends on the Fitbit client through mocking.
#[cfg_attr(test, mockall::automock)]
pub trait FitbitClientTrait {
    /// Fetches sleep data for a specific date
    ///
    /// # Arguments
    ///
    /// * `date` - The date for which to fetch sleep data
    ///
    /// # Returns
    ///
    /// Sleep data response or an error if the request failed
    fn fetch_sleep_data(&self, date: NaiveDate) -> Result<SleepResponseV1_2, FitbitError>;

    /// Fetches activity summary for a specific date
    ///
    /// # Arguments
    ///
    /// * `date` - The date for which to fetch activity data
    ///
    /// # Returns
    ///
    /// Activity summary response or an error if the request failed
    fn fetch_activity_summary(
        &self,
        date: NaiveDate,
    ) -> Result<ActivitySummaryResponse, FitbitError>;
}

/// Client for interacting with the Fitbit API
///
/// This client handles authentication, request formation, and response parsing
/// for Fitbit API endpoints.
#[derive(Clone)]
pub struct FitbitClient {
    access_token: Arc<String>,
    agent: ureq::Agent,
}

impl FitbitClient {
    /// Creates a new Fitbit client with the given access token
    ///
    /// # Arguments
    ///
    /// * `access_token` - The OAuth2 access token for authenticating with the Fitbit API
    ///
    /// # Example
    ///
    /// ```
    /// use fitbit_rs::FitbitClient;
    ///
    /// let client = FitbitClient::new("your_access_token".to_string());
    /// ```
    pub fn new(access_token: String) -> Self {
        let agent: Agent = Agent::config_builder()
            .timeout_global(Some(std::time::Duration::from_secs(30)))
            .build()
            .into();

        Self {
            access_token: Arc::new(access_token),
            agent,
        }
    }

    /// Creates a new Fitbit client with a custom agent configuration
    ///
    /// This allows more control over the HTTP client configuration, such as timeout values,
    /// proxy settings, etc.
    ///
    /// # Arguments
    ///
    /// * `access_token` - The OAuth2 access token for authenticating with the Fitbit API
    /// * `agent` - Custom configured ureq agent
    ///
    /// # Example
    ///
    /// ```
    /// use fitbit_rs::FitbitClient;
    /// use std::time::Duration;
    /// use ureq::Agent;
    ///
    /// let agent: Agent = Agent::config_builder()
    ///             .timeout_global(Some(std::time::Duration::from_secs(30)))
    ///             .into();
    ///
    /// let client = FitbitClient::with_agent("your_access_token".to_string(), agent);
    /// ```
    pub fn with_agent(access_token: String, agent: ureq::Agent) -> Self {
        Self {
            access_token: Arc::new(access_token),
            agent,
        }
    }

    /// Makes an API request to the given URL and deserializes the JSON response
    ///
    /// # Arguments
    ///
    /// * `url` - The full API URL to request
    ///
    /// # Returns
    ///
    /// The deserialized response or an error if the request or deserialization failed
    fn make_api_request<T>(&self, url: &str) -> Result<T, FitbitError>
    where
        T: serde::de::DeserializeOwned,
    {
        self.agent
            .get(url)
            .header("Authorization", &format!("Bearer {}", self.access_token))
            .call()
            .map_err(FitbitError::RequestError)?
            .body_mut()
            .read_json()
            .map_err(|e| FitbitError::JsonError(e.to_string()))
    }
}

impl FitbitClientTrait for FitbitClient {
    fn fetch_sleep_data(&self, date: NaiveDate) -> Result<SleepResponseV1_2, FitbitError> {
        let url = format!(
            "{}/{}/user/-/sleep/date/{}.json",
            API_BASE_URL,
            SLEEP_API_VERSION,
            date.format("%Y-%m-%d")
        );

        self.make_api_request(&url)
    }

    fn fetch_activity_summary(
        &self,
        date: NaiveDate,
    ) -> Result<ActivitySummaryResponse, FitbitError> {
        let url = format!(
            "{}/{}/user/-/activities/date/{}.json",
            API_BASE_URL,
            ACTIVITY_API_VERSION,
            date.format("%Y-%m-%d")
        );

        self.make_api_request(&url)
    }
}
