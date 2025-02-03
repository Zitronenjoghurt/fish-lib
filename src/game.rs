use crate::data::location_data::LocationData;
use crate::data::species_data::SpeciesData;
use crate::game::errors::resource::GameResourceError;
use crate::game::errors::GameResult;
use crate::game::interface::GameInterface;
use crate::game::repositories::fishing_history_entry_repository::FishingHistoryEntryRepository;
use crate::game::repositories::user_repository::UserRepository;
use crate::game::services::specimen_service::SpecimenService;
use crate::game::services::user_service::UserService;
use crate::get_config;
use crate::models::fishing_history_entry::FishingHistoryEntry;
use crate::models::specimen::Specimen;
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
pub struct Game;

impl GameInterface for Game {
    /// Find a user by their external ID.
    ///
    /// # Arguments
    ///
    /// * `external_id`: A freely selectable ID that your system will use to identify this user.
    ///
    /// # Returns
    ///
    /// Result<User, GameError>
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

    /// Register a new user by their external ID.
    ///
    /// # Arguments
    ///
    /// * `external_id`: A freely selectable ID that your system will use to identify this user.
    ///
    /// # Returns
    ///
    /// Result<User, GameError>
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

    /// Generate a random specimen of the given species ID and assign it to the given user.
    ///
    /// # Arguments
    ///
    /// * `user`: The user for which the catch is to be registered
    /// * `species_id`: The species ID of the specimen to be caught
    ///
    /// # Returns
    /// Result<(Specimen, FishingHistoryEntry), GameError>
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use fish_lib::config::Config;
    /// use fish_lib::data::species_data::SpeciesData;
    /// use fish_lib::game::prelude::*;
    /// use fish_lib::{set_config, setup_test};
    /// use fish_lib::game::repositories::user_repository::UserRepository;
    /// use fish_lib::models::user::User;
    /// setup_test();
    ///
    /// const USER_EXTERNAL_ID: i64 = 1337;
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
    /// // Create a user
    /// let user = Game::register_user(USER_EXTERNAL_ID).unwrap();
    ///
    /// // Let the user catch a specimen of the specified species ID
    /// let (specimen, history_entry) = Game::user_catch_specific_specimen(&user, SPECIES_ID).unwrap();
    /// assert_eq!(specimen.species_id, SPECIES_ID);
    /// assert_eq!(specimen.user_id, user.id);
    /// assert_eq!(history_entry.species_id, SPECIES_ID);
    /// assert_eq!(history_entry.caught_count, 1);
    ///
    /// // Catch a specimen with a species ID that doesn't exist
    /// let species_error = Game::user_catch_specific_specimen(&user, SPECIES_ID + 1).unwrap_err();
    /// if let Some(resource_error) = species_error.as_resource_error() {
    ///     assert!(resource_error.is_species_not_found());
    ///     assert_eq!(resource_error.get_species_id(), Some(SPECIES_ID + 1));
    /// } else {
    ///     panic!("{:?}", species_error);
    /// }
    ///
    /// // Catch a specimen for a user that doesn't exist
    /// let dummy_user = User {
    ///     id: -1,
    ///     external_id: USER_EXTERNAL_ID + 1,
    ///     ..Default::default()
    /// };
    /// let user_error = Game::user_catch_specific_specimen(&dummy_user, SPECIES_ID).unwrap_err();
    /// if let Some(resource_error) = user_error.as_resource_error() {
    ///     assert!(resource_error.is_user_not_found());
    ///     assert_eq!(resource_error.get_external_id(), Some(USER_EXTERNAL_ID + 1));
    /// } else {
    ///     panic!("{:?}", user_error);
    /// }
    ///
    /// ```
    fn user_catch_specific_specimen(
        user: &User,
        species_id: i32,
    ) -> GameResult<(Specimen, FishingHistoryEntry)> {
        Ok(SpecimenService::process_catch(user, species_id)?)
    }

    /// Check the fishing history of a user with a specified species ID
    ///
    /// # Arguments
    ///
    /// * `user`: The user to check the fishing history of
    /// * `species_id`: Fishing history species ID to check for the giving user
    ///
    /// # Returns
    ///
    /// Result<FishingHistoryEntry, GameError>
    /// - The fishing history of the given user with a specified species, if it exists
    /// - An error, if it does not exist, aka if the user did not catch that species yet or the user does not exist
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use fish_lib::config::Config;
    /// use fish_lib::data::species_data::SpeciesData;
    /// use fish_lib::game::prelude::*;
    /// use fish_lib::{set_config, setup_test};
    /// use fish_lib::game::repositories::user_repository::UserRepository;
    /// use fish_lib::models::user::User;
    /// setup_test();
    ///
    /// const USER_EXTERNAL_ID: i64 = 1337;
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
    /// // Create a user
    /// let user = Game::register_user(USER_EXTERNAL_ID).unwrap();
    ///
    /// // Let the user catch a specimen
    /// Game::user_catch_specific_specimen(&user, SPECIES_ID).unwrap();
    ///
    /// // Fetch the fishing history of the user with the given species ID
    /// let history_entry = Game::user_get_fishing_history(&user, SPECIES_ID).unwrap();
    /// assert_eq!(history_entry.species_id, SPECIES_ID);
    /// assert_eq!(history_entry.user_id, user.id);
    /// assert_eq!(history_entry.caught_count, 1);
    /// assert_eq!(history_entry.sold_count, 0);
    ///
    /// // Trying to fetch the fishing history with a species the user didn't catch yet
    /// let error = Game::user_get_fishing_history(&user, SPECIES_ID + 1).unwrap_err();
    /// if let Some(resource_error) = error.as_resource_error() {
    ///     assert!(resource_error.is_no_fishing_history());
    ///     assert_eq!(resource_error.get_external_id(), Some(USER_EXTERNAL_ID));
    ///     assert_eq!(resource_error.get_species_id(), Some(SPECIES_ID + 1));
    /// } else {
    ///     panic!("{:?}", error);
    /// }
    /// ```
    fn user_get_fishing_history(user: &User, species_id: i32) -> GameResult<FishingHistoryEntry> {
        match FishingHistoryEntryRepository::find_by_user_and_species_id(user.id, species_id)? {
            Some(entry) => Ok(entry),
            None => Err(GameResourceError::no_fishing_history(user.external_id, species_id).into()),
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
    /// Result<Arc<LocationData, Global>, GameError>
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
    /// Result<Arc<SpeciesData, Global>, GameError>
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
