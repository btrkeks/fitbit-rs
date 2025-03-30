//! Cache for Fitbit API responses.
//!
//! This module provides a caching mechanism for Fitbit API responses to reduce the number
//! of API calls made.

use crate::activity_summary::ActivitySummaryResponse;
use crate::error::FitbitError;
use crate::fitbit_client::FitbitClientTrait;
use crate::sleep::SleepResponseV1_2;
use chrono::NaiveDate;
use std::collections::HashMap;

/// A cache for Fitbit API responses.
///
/// This cache stores responses from the Fitbit API to reduce the number of API calls
/// made. It caches responses by date, so multiple requests for the same date will
/// only result in a single API call.
pub struct FitbitResponseCache<C: FitbitClientTrait> {
    fitbit_client: C,
    sleep_responses: HashMap<NaiveDate, SleepResponseV1_2>,
    activity_summary_responses: HashMap<NaiveDate, ActivitySummaryResponse>,
}

impl<C: FitbitClientTrait> FitbitResponseCache<C> {
    /// Creates a new cache with the given Fitbit client.
    ///
    /// # Arguments
    ///
    /// * `fitbit_client` - The Fitbit client to use for making API calls
    ///
    /// # Example
    ///
    /// ```
    /// use fitbit_rs::{FitbitClient, FitbitResponseCache};
    ///
    /// let client = FitbitClient::new("your_access_token".to_string());
    /// let cache = FitbitResponseCache::new(client);
    /// ```
    pub fn new(fitbit_client: C) -> Self {
        Self {
            fitbit_client,
            sleep_responses: HashMap::new(),
            activity_summary_responses: HashMap::new(),
        }
    }

    /// Gets a sleep response for the given date.
    ///
    /// If the response is not in the cache, it will be fetched from the API and cached.
    ///
    /// # Arguments
    ///
    /// * `date` - The date for which to get sleep data
    ///
    /// # Returns
    ///
    /// A reference to the cached sleep response or an error if the request failed
    ///
    /// # Example
    ///
    /// ```
    /// # use fitbit_rs::{FitbitClient, FitbitResponseCache};
    /// # use chrono::NaiveDate;
    /// #
    /// # let client = FitbitClient::new("your_access_token".to_string());
    /// # let mut cache = FitbitResponseCache::new(client);
    /// #
    /// let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
    /// let sleep_data = cache.get_sleep_response(date);
    /// ```
    pub fn get_sleep_response(
        &mut self,
        date: NaiveDate,
    ) -> Result<&SleepResponseV1_2, FitbitError> {
        if !self.sleep_responses.contains_key(&date) {
            let response = self.fitbit_client.fetch_sleep_data(date)?;
            self.sleep_responses.insert(date, response);
        }

        Ok(self.sleep_responses.get(&date).unwrap())
    }

    /// Gets an activity summary response for the given date.
    ///
    /// If the response is not in the cache, it will be fetched from the API and cached.
    ///
    /// # Arguments
    ///
    /// * `date` - The date for which to get activity data
    ///
    /// # Returns
    ///
    /// A reference to the cached activity summary response or an error if the request failed
    ///
    /// # Example
    ///
    /// ```
    /// # use fitbit_rs::{FitbitClient, FitbitResponseCache};
    /// # use chrono::NaiveDate;
    /// #
    /// # let client = FitbitClient::new("your_access_token".to_string());
    /// # let mut cache = FitbitResponseCache::new(client);
    /// #
    /// let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
    /// let activity_data = cache.get_activity_summary_response(date);
    /// ```
    pub fn get_activity_summary_response(
        &mut self,
        date: NaiveDate,
    ) -> Result<&ActivitySummaryResponse, FitbitError> {
        if !self.activity_summary_responses.contains_key(&date) {
            let response = self.fitbit_client.fetch_activity_summary(date)?;
            self.activity_summary_responses.insert(date, response);
        }

        Ok(self.activity_summary_responses.get(&date).unwrap())
    }

    /// Clears all cached responses.
    ///
    /// This can be useful if you want to force a refresh of all data.
    pub fn clear_cache(&mut self) {
        self.sleep_responses.clear();
        self.activity_summary_responses.clear();
    }

    /// Removes a specific date from the cache.
    ///
    /// This can be useful if you want to force a refresh of data for a specific date.
    ///
    /// # Arguments
    ///
    /// * `date` - The date to remove from the cache
    pub fn remove_from_cache(&mut self, date: NaiveDate) {
        self.sleep_responses.remove(&date);
        self.activity_summary_responses.remove(&date);
    }

    /// Gets a reference to the underlying Fitbit client.
    ///
    /// # Returns
    ///
    /// A reference to the Fitbit client
    pub fn client(&self) -> &C {
        &self.fitbit_client
    }
}

#[cfg(test)]
mod response_cache_tests {
    use super::*;
    use crate::fitbit_client::MockFitbitClientTrait;
    use chrono::NaiveDate;
    use mockall::predicate::*;

    #[test]
    fn test_cache_behavior() -> Result<(), FitbitError> {
        let mut mock_client = MockFitbitClientTrait::new();
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        // Setup mock expectations - sleep data should only be called once
        mock_client
            .expect_fetch_sleep_data()
            .with(eq(date))
            .times(1)
            .returning(|_| Ok(create_mock_sleep_response()));

        let mut cache = FitbitResponseCache::new(mock_client);

        // First call should fetch from API
        let _response1 = cache.get_sleep_response(date)?;
        // Second call should use cached data
        let _response2 = cache.get_sleep_response(date)?;

        Ok(())
    }

    #[test]
    fn test_clear_cache() -> Result<(), FitbitError> {
        let mut mock_client = MockFitbitClientTrait::new();
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        // Setup mock expectations - should be called twice due to cache clearing
        mock_client
            .expect_fetch_sleep_data()
            .with(eq(date))
            .times(2)
            .returning(|_| Ok(create_mock_sleep_response()));

        let mut cache = FitbitResponseCache::new(mock_client);

        // First call should fetch from API
        let _response1 = cache.get_sleep_response(date)?;

        // Clear cache
        cache.clear_cache();

        // Next call should fetch from API again
        let _response2 = cache.get_sleep_response(date)?;

        Ok(())
    }

    #[test]
    fn test_remove_from_cache() -> Result<(), FitbitError> {
        let mut mock_client = MockFitbitClientTrait::new();
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        // Setup mock expectations - should be called twice due to cache removal
        mock_client
            .expect_fetch_sleep_data()
            .with(eq(date))
            .times(2)
            .returning(|_| Ok(create_mock_sleep_response()));

        let mut cache = FitbitResponseCache::new(mock_client);

        // First call should fetch from API
        let _response1 = cache.get_sleep_response(date)?;

        // Remove specific date from cache
        cache.remove_from_cache(date);

        // Next call for same date should fetch from API again
        let _response2 = cache.get_sleep_response(date)?;

        Ok(())
    }

    fn create_mock_sleep_response() -> SleepResponseV1_2 {
        SleepResponseV1_2::default()
    }
}
