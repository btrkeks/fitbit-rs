//! Example for storing a Fitbit API access token.
//!
//! This example demonstrates how to store a Fitbit API access token for later use.
//!
//! # Running
//!
//! ```bash
//! cargo run --example store_token -- YOUR_ACCESS_TOKEN
//! ```

use fitbit_rs::access_token;
use std::env;
use std::process;

fn main() {
    // Get the access token from command line arguments
    let access_token = match env::args().nth(1) {
        Some(token) => token,
        None => {
            eprintln!("Usage: cargo run --example store_token -- YOUR_ACCESS_TOKEN");
            process::exit(1);
        }
    };

    // Store the access token
    match access_token::store_access_token(&access_token) {
        Ok(_) => {
            println!("Successfully stored access token!");
            println!("You can now use the library without manually providing an access token.");
        }
        Err(err) => {
            eprintln!("Failed to store access token: {}", err);
            process::exit(1);
        }
    }

    // Verify that the token can be retrieved
    match access_token::get_access_token() {
        Ok(token) => {
            if token == access_token {
                println!("Verified that the token was stored correctly.");
            } else {
                eprintln!("Warning: The retrieved token does not match the stored token!");
            }
        }
        Err(err) => {
            eprintln!("Failed to retrieve the stored access token: {}", err);
            process::exit(1);
        }
    }
}
