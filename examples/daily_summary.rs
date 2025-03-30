//! Example of fetching and displaying daily summary data from Fitbit.
//!
//! This example demonstrates how to fetch and display sleep and activity data
//! for a specific date using the fitbit-rs crate.
//!
//! # Running
//!
//! ```bash
//! cargo run --example daily_summary -- 2023-01-01
//! ```
//!
//! If no date is provided, today's date will be used.

use chrono::{Local, NaiveDate};
use fitbit_rs::{FitbitClient, FitbitClientTrait, FitbitError, SleepResponse, access_token};
use std::env;

fn main() -> Result<(), FitbitError> {
    // Parse date from command line arguments or use today's date
    let date = env::args()
        .nth(1)
        .and_then(|date_str| NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").ok())
        .unwrap_or_else(|| Local::now().date_naive());

    println!("Fetching Fitbit data for date: {}", date);

    // Get access token
    let access_token = match access_token::get_access_token() {
        Ok(token) => token,
        Err(err) => {
            eprintln!("Failed to get access token: {}", err);
            eprintln!("You can set an access token manually by running:");
            eprintln!("cargo run --example store_token -- YOUR_ACCESS_TOKEN");
            return Err(FitbitError::AccessTokenError(err));
        }
    };

    // Create Fitbit client
    let client = FitbitClient::new(access_token);

    // Fetch sleep data
    match client.fetch_sleep_data(date) {
        Ok(sleep_data) => {
            println!("\n=== Sleep Data ===");
            println!(
                "Total time in bed: {} minutes",
                sleep_data.summary.total_time_in_bed
            );
            println!(
                "Total time asleep: {} minutes",
                sleep_data.summary.total_minutes_asleep
            );
            println!(
                "Sleep efficiency: {}%",
                sleep_data.get_sleep_efficiency().unwrap_or(0)
            );

            // Display sleep stages if available
            println!("\nSleep Stages:");
            println!("  Deep sleep: {} minutes", sleep_data.summary.stages.deep);
            println!("  Light sleep: {} minutes", sleep_data.summary.stages.light);
            println!("  REM sleep: {} minutes", sleep_data.summary.stages.rem);
            println!("  Awake: {} minutes", sleep_data.summary.stages.wake);

            // Display wake-up and fall-asleep times if available
            if let Some(wake_up_time) = sleep_data.get_wake_up_time() {
                println!("\nWoke up at: {}", wake_up_time);
            }

            if let Some(fell_asleep_time) = sleep_data.get_time_fell_asleep() {
                println!("Fell asleep at: {}", fell_asleep_time);
            }
        }
        Err(err) => {
            eprintln!("Failed to fetch sleep data: {}", err);
        }
    }

    // Fetch activity summary
    match client.fetch_activity_summary(date) {
        Ok(activity_data) => {
            println!("\n=== Activity Summary ===");
            println!("Steps: {}", activity_data.summary.steps);
            println!("Calories burned: {}", activity_data.summary.calories_out);
            println!(
                "Active minutes: {}",
                activity_data.summary.fairly_active_minutes
                    + activity_data.summary.very_active_minutes
            );

            // Display heart rate data if available
            if !activity_data.summary.heart_rate_zones.is_empty() {
                println!("\nHeart Rate Zones:");
                for zone in &activity_data.summary.heart_rate_zones {
                    println!("  {:?}: {} minutes", zone.name, zone.minutes);
                }

                println!(
                    "\nResting heart rate: {}",
                    activity_data.summary.resting_heart_rate
                );
            }

            // Display goal progress
            println!("\nGoal Progress:");
            println!(
                "  Steps: {}/{} ({}%)",
                activity_data.summary.steps,
                activity_data.goals.steps,
                (activity_data.summary.steps as f64 / activity_data.goals.steps as f64 * 100.0)
                    .round()
            );

            println!(
                "  Active minutes: {}/{} ({}%)",
                activity_data.summary.fairly_active_minutes
                    + activity_data.summary.very_active_minutes,
                activity_data.goals.active_minutes,
                ((activity_data.summary.fairly_active_minutes
                    + activity_data.summary.very_active_minutes) as f64
                    / activity_data.goals.active_minutes as f64
                    * 100.0)
                    .round()
            );
        }
        Err(err) => {
            eprintln!("Failed to fetch activity data: {}", err);
        }
    }

    Ok(())
}
