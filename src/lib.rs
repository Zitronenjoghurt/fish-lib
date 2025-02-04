//! # Fish Lib
//!
//! A library for fish-based games.
//!
//! ## Getting Started
//!
//! The main entry point is [`crate::game::Game`]. That's where you will find the public API to the game and storage logic.
//!
//! ```rust
//! use fish_lib::config::{Config, ConfigBuilderInterface};
//! use fish_lib::game::prelude::*;
//! use fish_lib::game::service_provider::ServiceProviderInterface;
//!
//! let config = Config::builder()/*. ...() */.build();
//!
//! // Create game and clear database for a blank test state
//! let game = Game::new("postgresql://admin:root@db:5432/test_db", Some(config)).unwrap();
//! game.database().write().unwrap().clear().unwrap();
//!
//! // Example of basic usage, registering a user
//! let user = game.register_user(1337).unwrap();
//!
//! // Re-find registered user
//! let found_user = game.get_user(1337).unwrap();
//!
//! assert_eq!(user, found_user);
//! ```
//!
//! ## Core Modules
//!
//! - [`game`] - The primary module containing all game functionality
//! - [`config`] - Configuration types
//! - [`data`] - Supporting data structures

pub mod config;
pub mod data;
pub mod database;
pub mod enums;
pub mod game;
pub mod models;
pub mod schema;
#[cfg(test)]
pub mod tests;
pub mod traits;
pub mod utils;
