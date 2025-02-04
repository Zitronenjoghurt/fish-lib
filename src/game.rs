use crate::config::{Config, ConfigBuilderInterface, ConfigInterface};
use crate::data::location_data::LocationData;
use crate::data::species_data::SpeciesData;
use crate::database::{Database, DatabaseInterface};
use crate::game::errors::resource::GameResourceError;
use crate::game::errors::GameResult;
use crate::game::interface::GameInterface;
use crate::game::repositories::fishing_history_entry_repository::FishingHistoryEntryRepositoryInterface;
use crate::game::repositories::pond_repository::PondRepositoryInterface;
use crate::game::repositories::specimen_repository::SpecimenRepositoryInterface;
use crate::game::repositories::user_repository::UserRepositoryInterface;
use crate::game::service_provider::{ServiceProvider, ServiceProviderInterface};
use crate::game::services::encounter_service::EncounterServiceInterface;
use crate::game::services::fishing_history_service::FishingHistoryServiceInterface;
use crate::game::services::pond_service::PondServiceInterface;
use crate::game::services::specimen_service::SpecimenServiceInterface;
use crate::game::services::user_service::UserServiceInterface;
use crate::game::services::weather_service::WeatherServiceInterface;
use crate::models::fishing_history_entry::FishingHistoryEntry;
use crate::models::specimen::Specimen;
use crate::models::user::User;
use std::sync::{Arc, RwLock};

pub mod errors;
pub mod interface;
pub mod prelude;
pub mod repositories;
pub mod service_provider;
pub mod services;
pub mod systems;

/// # Game
/// Primary interface for all game operations.
///
/// The Game struct implements [`GameInterface`] and serves as the main entry point
/// for interacting with the game system. All game functionality is accessed
/// through this struct's implementation.
pub struct Game {
    service_provider: Arc<dyn ServiceProviderInterface>,
}

impl Game {
    pub fn new(db_url: &str, config: Option<Arc<dyn ConfigInterface>>) -> GameResult<Self> {
        let config = config.unwrap_or(Config::builder().build());
        let db = Database::create();
        db.write()
            .expect("Failed to get database write lock")
            .connect(db_url)?;

        let service_provider = ServiceProvider::create(config, db);
        let game = Game { service_provider };
        Ok(game)
    }
}

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
    /// use fish_lib::game::service_provider::ServiceProviderInterface;
    ///
    /// const EXTERNAL_ID: i64 = 1337;
    ///
    /// // Create game and clear database for a blank test state
    /// let game = Game::new("postgresql://admin:root@db:5432/test_db", None).unwrap();
    /// game.database().write().unwrap().clear().unwrap();
    ///
    /// // Finding an existing user
    /// let new_user = game.register_user(EXTERNAL_ID).unwrap();
    /// let found_user = game.get_user(EXTERNAL_ID).unwrap();
    /// assert_eq!(new_user, found_user);
    ///
    /// // Searching for a non-existent user
    /// let error = game.get_user(EXTERNAL_ID + 1).unwrap_err();
    /// if let Some(resource_error) = error.as_resource_error() {
    ///     assert!(resource_error.is_user_not_found());
    ///     assert_eq!(resource_error.get_external_id(), Some(EXTERNAL_ID + 1));
    /// } else {
    ///     panic!("{:?}", error);
    /// }
    /// ```
    fn get_user(&self, external_id: i64) -> GameResult<User> {
        match self.user_repository().find_by_external_id(external_id)? {
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
    /// use fish_lib::game::service_provider::ServiceProviderInterface;
    ///
    /// const EXTERNAL_ID: i64 = 1337;
    ///
    /// // Create game and clear database for a blank test state
    /// let game = Game::new("postgresql://admin:root@db:5432/test_db", None).unwrap();
    /// game.database().write().unwrap().clear().unwrap();
    ///
    /// // Registering a new user
    /// let user = game.register_user(EXTERNAL_ID).unwrap();
    /// assert_eq!(user.external_id, EXTERNAL_ID);
    ///
    /// // Registering an already existing user
    /// let error = game.register_user(EXTERNAL_ID).unwrap_err();
    /// if let Some(resource_error) = error.as_resource_error() {
    ///     assert!(resource_error.is_user_already_exists());
    ///     assert_eq!(resource_error.get_external_id(), Some(EXTERNAL_ID));
    /// } else {
    ///     panic!("{:?}", error);
    /// }
    /// ```
    fn register_user(&self, external_id: i64) -> GameResult<User> {
        match self.user_repository().find_by_external_id(external_id)? {
            Some(_) => Err(GameResourceError::user_already_exists(external_id).into()),
            None => Ok(self.user_service().create_and_save_user(external_id)?),
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
    /// use fish_lib::config::{Config, ConfigBuilderInterface};
    /// use fish_lib::data::species_data::SpeciesData;
    /// use fish_lib::game::prelude::*;
    /// use fish_lib::game::repositories::user_repository::UserRepository;
    /// use fish_lib::game::service_provider::ServiceProviderInterface;
    /// use fish_lib::models::user::User;
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
    ///
    /// // Create game and clear database for a blank test state
    /// let game = Game::new("postgresql://admin:root@db:5432/test_db", Some(config)).unwrap();
    /// game.database().write().unwrap().clear().unwrap();
    ///
    /// // Create a user
    /// let user = game.register_user(USER_EXTERNAL_ID).unwrap();
    ///
    /// // Let the user catch a specimen of the specified species ID
    /// let (specimen, history_entry) = game.user_catch_specific_specimen(&user, SPECIES_ID).unwrap();
    /// assert_eq!(specimen.species_id, SPECIES_ID);
    /// assert_eq!(specimen.user_id, user.id);
    /// assert_eq!(history_entry.species_id, SPECIES_ID);
    /// assert_eq!(history_entry.caught_count, 1);
    ///
    /// // Catch a specimen with a species ID that doesn't exist
    /// let species_error = game.user_catch_specific_specimen(&user, SPECIES_ID + 1).unwrap_err();
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
    /// let user_error = game.user_catch_specific_specimen(&dummy_user, SPECIES_ID).unwrap_err();
    /// if let Some(resource_error) = user_error.as_resource_error() {
    ///     assert!(resource_error.is_user_not_found());
    ///     assert_eq!(resource_error.get_external_id(), Some(USER_EXTERNAL_ID + 1));
    /// } else {
    ///     panic!("{:?}", user_error);
    /// }
    ///
    /// ```
    fn user_catch_specific_specimen(
        &self,
        user: &User,
        species_id: i32,
    ) -> GameResult<(Specimen, FishingHistoryEntry)> {
        self.specimen_service().process_catch(user, species_id)
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
    /// use fish_lib::config::{Config, ConfigBuilderInterface};
    /// use fish_lib::data::species_data::SpeciesData;
    /// use fish_lib::game::prelude::*;
    /// use fish_lib::game::repositories::user_repository::UserRepository;
    /// use fish_lib::game::service_provider::ServiceProviderInterface;
    /// use fish_lib::models::user::User;
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
    ///
    /// // Create game and clear database for a blank test state
    /// let game = Game::new("postgresql://admin:root@db:5432/test_db", Some(config)).unwrap();
    /// game.database().write().unwrap().clear().unwrap();
    ///
    /// // Create a user
    /// let user = game.register_user(USER_EXTERNAL_ID).unwrap();
    ///
    /// // Let the user catch a specimen
    /// game.user_catch_specific_specimen(&user, SPECIES_ID).unwrap();
    ///
    /// // Fetch the fishing history of the user with the given species ID
    /// let history_entry = game.user_get_fishing_history(&user, SPECIES_ID).unwrap();
    /// assert_eq!(history_entry.species_id, SPECIES_ID);
    /// assert_eq!(history_entry.user_id, user.id);
    /// assert_eq!(history_entry.caught_count, 1);
    /// assert_eq!(history_entry.sold_count, 0);
    ///
    /// // Trying to fetch the fishing history with a species the user didn't catch yet
    /// let error = game.user_get_fishing_history(&user, SPECIES_ID + 1).unwrap_err();
    /// if let Some(resource_error) = error.as_resource_error() {
    ///     assert!(resource_error.is_no_fishing_history());
    ///     assert_eq!(resource_error.get_external_id(), Some(USER_EXTERNAL_ID));
    ///     assert_eq!(resource_error.get_species_id(), Some(SPECIES_ID + 1));
    /// } else {
    ///     panic!("{:?}", error);
    /// }
    /// ```
    fn user_get_fishing_history(
        &self,
        user: &User,
        species_id: i32,
    ) -> GameResult<FishingHistoryEntry> {
        match self
            .fishing_history_entry_repository()
            .find_by_user_and_species_id(user.id, species_id)?
        {
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
    /// use fish_lib::config::{Config, ConfigBuilderInterface};
    /// use fish_lib::game::prelude::*;
    /// use fish_lib::data::location_data::LocationData;
    /// use fish_lib::game::service_provider::ServiceProviderInterface;
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
    ///
    /// // Create game and clear database for a blank test state
    /// let game = Game::new("postgresql://admin:root@db:5432/test_db", Some(config)).unwrap();
    /// game.database().write().unwrap().clear().unwrap();
    ///
    /// // Finding the location data
    /// let found_location_data = game.get_location_data(LOCATION_ID).unwrap();
    /// assert_eq!(&found_location_data.name, LOCATION_NAME);
    ///
    /// // Searching for non-existent location data
    /// let error = game.get_location_data(LOCATION_ID + 1).unwrap_err();
    /// if let Some(resource_error) = error.as_resource_error() {
    ///     assert!(resource_error.is_location_not_found());
    ///     assert_eq!(resource_error.get_location_id(), Some(LOCATION_ID + 1));
    /// } else {
    ///     panic!("{:?}", error);
    /// }
    /// ```
    fn get_location_data(&self, location_id: i32) -> GameResult<Arc<LocationData>> {
        match self.config().locations().get(&location_id) {
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
    /// use fish_lib::config::{Config, ConfigBuilderInterface};
    /// use fish_lib::data::species_data::SpeciesData;
    /// use fish_lib::game::prelude::*;
    /// use fish_lib::game::service_provider::ServiceProviderInterface;
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
    ///
    /// // Create game and clear database for a blank test state
    /// let game = Game::new("postgresql://admin:root@db:5432/test_db", Some(config)).unwrap();
    /// game.database().write().unwrap().clear().unwrap();
    ///
    /// // Finding the species data
    /// let found_species_data = game.get_species_data(SPECIES_ID).unwrap();
    /// assert_eq!(&found_species_data.name, SPECIES_NAME);
    ///
    /// // Searching for non-existent species data
    /// let error = game.get_species_data(SPECIES_ID + 1).unwrap_err();
    /// if let Some(resource_error) = error.as_resource_error() {
    ///     assert!(resource_error.is_species_not_found());
    ///     assert_eq!(resource_error.get_species_id(), Some(SPECIES_ID + 1));
    /// } else {
    ///     panic!("{:?}", error);
    /// }
    /// ```
    fn get_species_data(&self, species_id: i32) -> GameResult<Arc<SpeciesData>> {
        match self.config().species().get(&species_id) {
            Some(data) => Ok(data.clone()),
            None => Err(GameResourceError::species_not_found(species_id).into()),
        }
    }
}

impl ServiceProviderInterface for Game {
    fn config(&self) -> Arc<dyn ConfigInterface> {
        self.service_provider.config()
    }

    fn database(&self) -> Arc<RwLock<dyn DatabaseInterface>> {
        self.service_provider.database()
    }

    fn fishing_history_entry_repository(&self) -> Arc<dyn FishingHistoryEntryRepositoryInterface> {
        self.service_provider.fishing_history_entry_repository()
    }

    fn pond_repository(&self) -> Arc<dyn PondRepositoryInterface> {
        self.service_provider.pond_repository()
    }

    fn specimen_repository(&self) -> Arc<dyn SpecimenRepositoryInterface> {
        self.service_provider.specimen_repository()
    }

    fn user_repository(&self) -> Arc<dyn UserRepositoryInterface> {
        self.service_provider.user_repository()
    }

    fn encounter_service(&self) -> Arc<dyn EncounterServiceInterface> {
        self.service_provider.encounter_service()
    }

    fn fishing_history_service(&self) -> Arc<dyn FishingHistoryServiceInterface> {
        self.service_provider.fishing_history_service()
    }

    fn pond_service(&self) -> Arc<dyn PondServiceInterface> {
        self.service_provider.pond_service()
    }

    fn specimen_service(&self) -> Arc<dyn SpecimenServiceInterface> {
        self.service_provider.specimen_service()
    }

    fn user_service(&self) -> Arc<dyn UserServiceInterface> {
        self.service_provider.user_service()
    }

    fn weather_service(&self) -> Arc<dyn WeatherServiceInterface> {
        self.service_provider.weather_service()
    }
}
