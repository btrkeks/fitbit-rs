//! # Fitbit-rs
//!
//! A Rust client for the Fitbit API that allows fetching sleep data and activity summaries.
//!
//! ## Features
//!
//! * Authentication using access tokens
//! * Fetch sleep data with detailed sleep stages and levels
//! * Fetch activity summaries including steps, calories, heart rate zones, etc.
//! * Response caching to minimize API calls
//!
//! ## Examples
//!
//! ```no_run
//! use fitbit_rs::{FitbitClient, FitbitClientTrait};
//! use chrono::NaiveDate;
//!
//! fn main() -> Result<(), fitbit_rs::FitbitError> {
//!     // Get access token (typically from environment or config file)
//!     let access_token = "your_access_token".to_string();
//!
//!     // Create a client
//!     let client = FitbitClient::new(access_token);
//!
//!     // Fetch today's sleep data
//!     let today = chrono::Local::now().date_naive();
//!     let sleep_data = client.fetch_sleep_data(today)?;
//!
//!     println!("Sleep duration: {} minutes", sleep_data.summary.total_minutes_asleep);
//!
//!     Ok(())
//! }
//! ```

pub mod access_token;
pub mod activity_summary;
pub mod error;
pub mod fitbit_client;
mod response_cache;
pub mod sleep;

// Re-export the most commonly used types
pub use access_token::{AccessTokenError, get_access_token};
pub use activity_summary::ActivitySummaryResponse;
pub use error::FitbitError;
pub use fitbit_client::{FitbitClient, FitbitClientTrait};
pub use response_cache::FitbitResponseCache;
pub use sleep::{SleepLevel, SleepResponse, SleepResponseV1_2};
