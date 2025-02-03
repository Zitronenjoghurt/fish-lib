use crate::data::location_data::LocationData;
use crate::data::species_data::SpeciesData;
use crate::game::errors::resource::GameResourceError;
use crate::game::errors::GameResult;
use crate::game::interface::GameInterface;
use crate::game::repositories::user_repository::UserRepository;
use crate::game::services::user_service::UserService;
use crate::get_config;
use crate::models::user::User;
use std::sync::Arc;

pub mod errors;
pub mod interface;
pub mod prelude;
pub mod repositories;
pub mod services;
pub mod systems;

/// # Game
/// Primary interface for all game operations.
///
/// The Game struct implements [`GameInterface`] and serves as the main entry point
/// for interacting with the game system. All game functionality is accessed
/// through this struct's implementation.
/// ```
pub struct Game;

impl GameInterface for Game {
    /// Register a new user by their external ID.
    ///
    /// # Arguments
    ///
    /// * `external_id`: A freely selectable ID that your system will use to identify this user.
    ///
    /// # Returns
    ///
    /// returns: Result<User, GameError>
    /// - A newly created user with the given external id
    /// - An error, if:
    ///     - A user with the given external id already exists
    ///     - Database operations fail
    ///
    /// # Examples
    ///
    /// ```
    /// use fish_lib::game::prelude::*;
    /// use fish_lib::setup_test;
    /// setup_test();
    ///
    /// const EXTERNAL_ID: i64 = 1337;
    ///
    /// // Registering a new user
    /// let user = Game::register_user(EXTERNAL_ID).unwrap();
    /// assert_eq!(user.external_id, EXTERNAL_ID);
    ///
    /// // Registering an already existing user
    /// let error = Game::register_user(EXTERNAL_ID).unwrap_err();
    /// if let Some(resource_error) = error.as_resource_error() {
    ///     assert!(resource_error.is_user_already_exists());
    ///     assert_eq!(resource_error.get_external_id(), Some(EXTERNAL_ID));
    /// } else {
    ///     panic!("{:?}", error);
    /// }
    /// ```
    fn register_user(external_id: i64) -> GameResult<User> {
        match UserRepository::find_by_external_id(external_id)? {
            Some(_) => Err(GameResourceError::user_already_exists(external_id).into()),
            None => Ok(UserService::create_and_save_user(external_id)?),
        }
    }

    /// Find a user by their external ID.
    ///
    /// # Arguments
    ///
    /// * `external_id`: A freely selectable ID that your system will use to identify this user.
    ///
    /// # Returns
    ///
    /// returns: Result<User, GameError>
    /// - A user with the given external ID
    /// - An error, if:
    ///     - The user is not found
    ///     - Database operations fail
    ///
    /// # Examples
    ///
    /// ```
    /// use fish_lib::game::prelude::*;
    /// use fish_lib::setup_test;
    /// setup_test();
    ///
    /// const EXTERNAL_ID: i64 = 1337;
    ///
    /// // Finding an existing user
    /// let new_user = Game::register_user(EXTERNAL_ID).unwrap();
    /// let found_user = Game::get_user(EXTERNAL_ID).unwrap();
    /// assert_eq!(new_user, found_user);
    ///
    /// // Searching for a non-existent user
    /// let error = Game::get_user(EXTERNAL_ID + 1).unwrap_err();
    /// if let Some(resource_error) = error.as_resource_error() {
    ///     assert!(resource_error.is_user_not_found());
    ///     assert_eq!(resource_error.get_external_id(), Some(EXTERNAL_ID + 1));
    /// } else {
    ///     panic!("{:?}", error);
    /// }
    /// ```
    fn get_user(external_id: i64) -> GameResult<User> {
        match UserRepository::find_by_external_id(external_id)? {
            Some(user) => Ok(user),
            None => Err(GameResourceError::user_not_found(external_id).into()),
        }
    }

    /// Get location data for the specified location ID.
    ///
    /// # Arguments
    ///
    /// * `location_id`: The ID of the location to get the data of.
    ///
    /// # Returns
    ///
    /// returns: Result<Arc<LocationData, Global>, GameError>
    /// - The location data, if the location with the given ID exists
    /// - An error, if no location with the given ID exists
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use fish_lib::config::Config;
    /// use fish_lib::game::prelude::*;
    /// use fish_lib::{set_config, setup_test};
    /// use fish_lib::data::location_data::LocationData;
    /// setup_test();
    ///
    /// const LOCATION_ID: i32 = 1;
    /// const LOCATION_NAME: &str = "Central Europe";
    ///
    /// // Define some location data
    /// let location_data = LocationData {
    ///     name: LOCATION_NAME.to_string(),
    ///     ..Default::default()
    /// };
    /// let location_data_map = HashMap::from([(LOCATION_ID, location_data)]);
    ///
    /// // Add the location data to the config
    /// let config = Config::builder().locations(location_data_map).build();
    /// set_config(config);
    ///
    /// // Finding the location data
    /// let found_location_data = Game::get_location_data(LOCATION_ID).unwrap();
    /// assert_eq!(&found_location_data.name, LOCATION_NAME);
    ///
    /// // Searching for non-existent location data
    /// let error = Game::get_location_data(LOCATION_ID + 1).unwrap_err();
    /// if let Some(resource_error) = error.as_resource_error() {
    ///     assert!(resource_error.is_location_not_found());
    ///     assert_eq!(resource_error.get_location_id(), Some(LOCATION_ID + 1));
    /// } else {
    ///     panic!("{:?}", error);
    /// }
    /// ```
    fn get_location_data(location_id: i32) -> GameResult<Arc<LocationData>> {
        match get_config().locations.get(&location_id) {
            Some(data) => Ok(data.clone()),
            None => Err(GameResourceError::location_not_found(location_id).into()),
        }
    }

    /// Get species data for the specified species ID.
    ///
    /// # Arguments
    ///
    /// * `species_id`: The ID of the species to get the data of.
    ///
    /// # Returns
    ///
    /// returns: Result<Arc<SpeciesData, Global>, GameError>
    /// - The species data, if the species with the given ID exists
    /// - An error, if no species with the given ID exists
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use fish_lib::config::Config;
    /// use fish_lib::data::species_data::SpeciesData;
    /// use fish_lib::game::prelude::*;
    /// use fish_lib::{set_config, setup_test};
    /// setup_test();
    ///
    /// const SPECIES_ID: i32 = 1;
    /// const SPECIES_NAME: &str = "Salmon";
    ///
    /// // Define some species data
    /// let species_data = SpeciesData {
    ///     name: SPECIES_NAME.to_string(),
    ///     ..Default::default()
    /// };
    /// let species_data_map = HashMap::from([(SPECIES_ID, species_data)]);
    ///
    /// // Add the species data to the config
    /// let config = Config::builder().species(species_data_map).build();
    /// set_config(config);
    ///
    /// // Finding the species data
    /// let found_species_data = Game::get_species_data(SPECIES_ID).unwrap();
    /// assert_eq!(&found_species_data.name, SPECIES_NAME);
    ///
    /// // Searching for non-existent species data
    /// let error = Game::get_species_data(SPECIES_ID + 1).unwrap_err();
    /// if let Some(resource_error) = error.as_resource_error() {
    ///     assert!(resource_error.is_species_not_found());
    ///     assert_eq!(resource_error.get_species_id(), Some(SPECIES_ID + 1));
    /// } else {
    ///     panic!("{:?}", error);
    /// }
    /// ```
    fn get_species_data(species_id: i32) -> GameResult<Arc<SpeciesData>> {
        match get_config().species.get(&species_id) {
            Some(data) => Ok(data.clone()),
            None => Err(GameResourceError::species_not_found(species_id).into()),
        }
    }
}
