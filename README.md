# fitbit-rs

[![Crates.io](https://img.shields.io/crates/v/fitbit-rs.svg)](https://crates.io/crates/fitbit-rs)
[![Documentation](https://docs.rs/fitbit-rs/badge.svg)](https://docs.rs/fitbit-rs)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/fitbit-rs.svg)](./LICENSE)

A Rust client library for the Fitbit API that allows fetching sleep data and activity summaries.

## Features

- Authentication using access tokens
- Fetch sleep data with detailed sleep stages and levels
- Fetch activity summaries including steps, calories, heart rate zones, etc.
- Response caching to minimize API calls
- Fully documented API with examples

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
fitbit-rs = "0.1.0"
```

## Usage

### Basic Usage

```rust
use fitbit_rs::{FitbitClient, FitbitClientTrait};
use chrono::NaiveDate;

fn main() -> Result<(), fitbit_rs::FitbitError> {
    // Get access token (typically from environment or config file)
    let access_token = "your_access_token".to_string();
    
    // Create a client
    let client = FitbitClient::new(access_token);
    
    // Fetch today's sleep data
    let today = chrono::Local::now().date_naive();
    let sleep_data = client.fetch_sleep_data(today)?;
    
    println!("Sleep duration: {} minutes", sleep_data.summary.total_minutes_asleep);
    
    // Fetch today's activity summary
    let activity = client.fetch_activity_summary(today)?;
    println!("Steps today: {}", activity.summary.steps);
    
    Ok(())
}
```

### Storing Access Tokens

The library provides utilities for storing and retrieving access tokens:

```rust
use fitbit_rs::access_token::{get_access_token, store_access_token};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Store an access token
    store_access_token("your_new_access_token")?;
    
    // Retrieve the stored access token
    let token = get_access_token()?;
    println!("Retrieved token: {}", token);
    
    Ok(())
}
```

## Getting an Access Token

To use this library, you need a Fitbit API access token. You can get one by:

1. Register an application at [dev.fitbit.com](https://dev.fitbit.com/apps/new)
2. Use the OAuth 2.0 flow to get an access token
3. Store the access token using the `store_access_token` function or pass it directly to the client

## Documentation

For more detailed documentation, see the [API Documentation](https://docs.rs/fitbit-rs).

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.