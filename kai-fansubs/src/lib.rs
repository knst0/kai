//! # kai-fansubs
//!
//! A Rust client library for interacting with the fansubs.ru to access media and subtitle information.
//!
//! ## Overview
//!
//! This library provides a convenient interface to:
//! - Browse media catalog alphabetically
//! - Fetch detailed media information including subtitles
//! - Download subtitle files in various formats
//! - Retrieve subtitle notes and additional information
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use kai_fansubs::client::Client;
//! use kai_fansubs::queries::{AlphabetCatalogQuery, CatalogSymbol, MediaQuery};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client with default settings
//!     let client = Client::new();
//!
//!     // Browse catalog by letter
//!     let catalog = AlphabetCatalogQuery::new(CatalogSymbol::A)
//!         .execute(&client)
//!         .await?;
//!
//!     // Get detailed information about a specific media
//!     let media = MediaQuery::new(4214).execute(&client).await?;
//!     println!("Title: {}", media.main_title);
//!     println!("Subtitles available: {}", media.subtitles.len());
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Client Configuration
//!
//! The client can be customized using the builder pattern:
//!
//! ```rust,no_run
//! use kai_fansubs::client::Client;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::builder()
//!         .user_agent("MyApp/1.0")
//!         .rate_limit_per_minute(60) // Custom rate limit
//!         .rate_limit_per_second(3) // Custom rate limit
//!         .build();
//! }
//! ```

pub mod client;
pub mod error;
mod parsing;
pub mod queries;
pub mod types;
