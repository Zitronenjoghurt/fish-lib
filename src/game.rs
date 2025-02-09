use crate::config::{Config, ConfigBuilderInterface, ConfigInterface};
use crate::data::location_data::LocationData;
use crate::data::species_data::SpeciesData;
use crate::database::{Database, DatabaseInterface};
use crate::dto::user_location_unlock::UserLocationUnlock;
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
use crate::game::services::location_service::LocationServiceInterface;
use crate::game::services::pond_service::PondServiceInterface;
use crate::game::services::species_service::SpeciesServiceInterface;
use crate::game::services::specimen_service::SpecimenServiceInterface;
use crate::game::services::user_service::UserServiceInterface;
use crate::game::services::weather_service::WeatherServiceInterface;
use crate::game::systems::weather_system::weather::Weather;
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
        let config = config.unwrap_or(Config::builder().build().unwrap());
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
    /// Get [LocationData] for the specified location ID.
    ///
    /// # Arguments
    ///
    /// * `location_id`: The ID of the location to get the data of. (See [Config])
    ///
    /// # Returns
    ///
    /// Result<Arc<[LocationData], Global>, [errors::GameError]>
    /// - The [LocationData], if the location with the given ID exists
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
    /// let config = Config::builder().locations(location_data_map).build().unwrap();
    ///
    /// // Create game and clear database for a blank test state
    /// let game = Game::new("postgresql://admin:root@db:5432/test_db", Some(config)).unwrap();
    /// game.database().write().unwrap().clear().unwrap();
    ///
    /// // Finding the location data
    /// let found_location_data = game.location_find(LOCATION_ID).unwrap();
    /// assert_eq!(&found_location_data.name, LOCATION_NAME);
    ///
    /// // Searching for non-existent location data
    /// let error = game.location_find(LOCATION_ID + 1).unwrap_err();
    /// assert!(error.is_not_found());
    /// if let Some(resource_error) = error.as_resource_error() {
    ///     assert!(resource_error.is_location_not_found());
    ///     assert_eq!(resource_error.get_location_id(), Some(LOCATION_ID + 1));
    /// } else {
    ///     panic!("{:?}", error);
    /// }
    /// ```
    fn location_find(&self, location_id: i32) -> GameResult<Arc<LocationData>> {
        match self.config().locations().get(&location_id) {
            Some(data) => Ok(data.clone()),
            None => Err(GameResourceError::location_not_found(location_id).into()),
        }
    }

    /// Get the current [Weather] of a specified location.
    /// You will be able to get the weather for all locations specified by you in your [Config].
    ///
    /// # Arguments
    ///
    /// * `location_id`: The ID of the location to get the current [Weather] from. (See [Config])
    ///
    /// # Returns
    /// Result<[Weather], [errors::GameError]>
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use fish_lib::config::{Config, ConfigBuilderInterface};
    /// use fish_lib::data::location_data::LocationData;
    /// use fish_lib::data::season_data::SeasonData;
    /// use fish_lib::game::prelude::*;
    /// use fish_lib::game::service_provider::ServiceProviderInterface;
    ///
    /// const LOCATION_ID: i32 = 1;
    ///
    /// // For simplicity in testing, create a location with constant weather
    /// let every_season = SeasonData {
    ///     min_temp_c: 10.0,
    ///     max_temp_c: 10.0,
    ///     ..Default::default()
    /// };
    ///
    /// let location_data = LocationData {
    ///     spring: every_season.clone(),
    ///     summer: every_season.clone(),
    ///     autumn: every_season.clone(),
    ///     winter: every_season.clone(),
    ///     ..Default::default()
    /// };
    ///
    /// let location_data_map = HashMap::from([(LOCATION_ID, location_data)]);
    /// let config = Config::builder().locations(location_data_map).build().unwrap();
    ///
    /// // Create game and clear database for a blank test state
    /// let game = Game::new("postgresql://admin:root@db:5432/test_db", Some(config)).unwrap();
    /// game.database().write().unwrap().clear().unwrap();
    ///
    /// // Get the current weather
    /// let location_data = game.location_find(LOCATION_ID).unwrap();
    /// let weather = game.location_weather_current(location_data).unwrap();
    /// assert_eq!(weather.temperature_c, 10.0);
    /// ```
    fn location_weather_current(&self, location: Arc<LocationData>) -> GameResult<Weather> {
        self.weather_service().get_current_weather(location)
    }

    /// Get [SpeciesData] for the specified species ID.
    ///
    /// # Arguments
    ///
    /// * `species_id`: The ID of the species to get the data of. (See [Config])
    ///
    /// # Returns
    ///
    /// Result<Arc<[SpeciesData], Global>, [errors::GameError]>
    /// - The [SpeciesData], if the species with the given ID exists
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
    /// let config = Config::builder().species(species_data_map).build().unwrap();
    ///
    /// // Create game and clear database for a blank test state
    /// let game = Game::new("postgresql://admin:root@db:5432/test_db", Some(config)).unwrap();
    /// game.database().write().unwrap().clear().unwrap();
    ///
    /// // Finding the species data
    /// let found_species_data = game.species_find(SPECIES_ID).unwrap();
    /// assert_eq!(&found_species_data.name, SPECIES_NAME);
    ///
    /// // Searching for non-existent species data
    /// let error = game.species_find(SPECIES_ID + 1).unwrap_err();
    /// assert!(error.is_not_found());
    /// if let Some(resource_error) = error.as_resource_error() {
    ///     assert!(resource_error.is_species_not_found());
    ///     assert_eq!(resource_error.get_species_id(), Some(SPECIES_ID + 1));
    /// } else {
    ///     panic!("{:?}", error);
    /// }
    /// ```
    fn species_find(&self, species_id: i32) -> GameResult<Arc<SpeciesData>> {
        match self.config().species().get(&species_id) {
            Some(data) => Ok(data.clone()),
            None => Err(GameResourceError::species_not_found(species_id).into()),
        }
    }

    /// Generate a random [Specimen] of the given species ID and assign it to the given [User].
    ///
    /// # Arguments
    ///
    /// * `user`: The [User] for which the catch is to be registered
    /// * `species_id`: The species ID of the [Specimen] to be caught (See [Config])
    ///
    /// # Returns
    /// Result<([Specimen], [FishingHistoryEntry]), [errors::GameError]>
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
    /// let config = Config::builder().species(species_data_map).build().unwrap();
    ///
    /// // Create game and clear database for a blank test state
    /// let game = Game::new("postgresql://admin:root@db:5432/test_db", Some(config)).unwrap();
    /// game.database().write().unwrap().clear().unwrap();
    ///
    /// // Fetch the species data
    /// let species = game.species_find(1).unwrap();
    ///
    /// // Create a user
    /// let user = game.user_register(USER_EXTERNAL_ID).unwrap();
    ///
    /// // Let the user catch a specimen of the specified species ID
    /// let (specimen, history_entry) = game.user_catch_specific_specimen(&user, species.clone()).unwrap();
    /// assert_eq!(specimen.species_id, SPECIES_ID);
    /// assert_eq!(specimen.user_id, user.id);
    /// assert_eq!(history_entry.species_id, SPECIES_ID);
    /// assert_eq!(history_entry.caught_count, 1);
    ///
    /// // Catch a specimen for a user that doesn't exist
    /// let dummy_user = User {
    ///     id: -1,
    ///     external_id: USER_EXTERNAL_ID + 1,
    ///     ..Default::default()
    /// };
    /// let user_error = game.user_catch_specific_specimen(&dummy_user, species).unwrap_err();
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
        species: Arc<SpeciesData>,
    ) -> GameResult<(Specimen, FishingHistoryEntry)> {
        let specimen = self.specimen_service().process_catch(user, species)?;
        let entry = self.fishing_history_service().register_catch(&specimen)?;
        Ok((specimen, entry))
    }

    /// Check the fishing history of a [User] with a specified species ID
    ///
    /// # Arguments
    ///
    /// * `user`: The [User] to check the fishing history of
    /// * `species_id`: Fishing history species ID to check for the giving [User] (See [Config])
    ///
    /// # Returns
    ///
    /// Result<[FishingHistoryEntry], [errors::GameError]>
    /// - The fishing history of the given [User] with a specified species, if it exists
    /// - An error, if it does not exist, aka if the [User] did not catch that species yet or the [User] does not exist
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
    /// let species_data2 = SpeciesData {
    ///     ..Default::default()
    ///  };
    /// let species_data_map = HashMap::from([(SPECIES_ID, species_data), (SPECIES_ID + 1, species_data2)]);
    ///
    /// // Add the species data to the config
    /// let config = Config::builder().species(species_data_map).build().unwrap();
    ///
    /// // Create game and clear database for a blank test state
    /// let game = Game::new("postgresql://admin:root@db:5432/test_db", Some(config)).unwrap();
    /// game.database().write().unwrap().clear().unwrap();
    ///
    /// // Create a user
    /// let user = game.user_register(USER_EXTERNAL_ID).unwrap();
    ///
    /// // Get species data
    /// let species = game.species_find(SPECIES_ID).unwrap();
    /// let species2 = game.species_find(SPECIES_ID + 1).unwrap();
    ///
    /// // Let the user catch a specimen
    /// game.user_catch_specific_specimen(&user, species.clone()).unwrap();
    ///
    /// // Fetch the fishing history of the user with the given species ID
    /// let history_entry = game.user_get_fishing_history(&user, species.clone()).unwrap();
    /// assert_eq!(history_entry.species_id, SPECIES_ID);
    /// assert_eq!(history_entry.user_id, user.id);
    /// assert_eq!(history_entry.caught_count, 1);
    /// assert_eq!(history_entry.sold_count, 0);
    ///
    /// // Trying to fetch the fishing history with a species the user didn't catch yet
    /// let error = game.user_get_fishing_history(&user, species2).unwrap_err();
    /// assert!(error.is_not_found());
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
        species: Arc<SpeciesData>,
    ) -> GameResult<FishingHistoryEntry> {
        match self
            .fishing_history_entry_repository()
            .find_by_user_and_species_id(user.id, species.id)?
        {
            Some(entry) => Ok(entry),
            None => Err(GameResourceError::no_fishing_history(user.external_id, species.id).into()),
        }
    }

    /// Find a [User] by their external ID.
    ///
    /// # Arguments
    ///
    /// * `external_id`: A freely selectable ID that your system will use to identify this [User].
    ///
    /// # Returns
    ///
    /// Result<[User], [errors::GameError]>
    /// - A [User] with the given external ID
    /// - An error, if:
    ///     - The [User] is not found
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
    /// let new_user = game.user_register(EXTERNAL_ID).unwrap();
    /// let found_user = game.user_find(EXTERNAL_ID).unwrap();
    /// assert_eq!(new_user, found_user);
    ///
    /// // Searching for a non-existent user
    /// let error = game.user_find(EXTERNAL_ID + 1).unwrap_err();
    /// assert!(error.is_not_found());
    /// if let Some(resource_error) = error.as_resource_error() {
    ///     assert!(resource_error.is_user_not_found());
    ///     assert_eq!(resource_error.get_external_id(), Some(EXTERNAL_ID + 1));
    /// } else {
    ///     panic!("{:?}", error);
    /// }
    /// ```
    fn user_find(&self, external_id: i64) -> GameResult<User> {
        match self.user_repository().find_by_external_id(external_id)? {
            Some(user) => Ok(user),
            None => Err(GameResourceError::user_not_found(external_id).into()),
        }
    }

    /// Fetch the unlocked locations of a given user.
    ///
    /// # Arguments
    ///
    /// * `user`: The [User] to get the unlocked locations ([UserLocationUnlock]) from.
    ///
    /// # Returns
    /// Result<Vec<[UserLocationUnlock], Global>, [errors::GameError]>
    /// - A vector with information about all location unlocks
    /// - An error, if database operations fail
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use fish_lib::config::{Config, ConfigBuilderInterface};
    /// use fish_lib::data::location_data::LocationData;
    /// use fish_lib::game::prelude::*;
    /// use fish_lib::game::service_provider::ServiceProviderInterface;
    ///
    /// const EXTERNAL_ID: i64 = 1337;
    /// const LOCATION_ID: i32 = 13;
    /// const LOCATION_NAME: &str = "Island";
    ///
    /// // Define some location data
    /// let location_data = LocationData {
    ///     name: LOCATION_NAME.to_string(),
    ///     ..Default::default()
    /// };
    /// let location_data_map = HashMap::from([(LOCATION_ID, location_data)]);
    ///
    /// // Add the location data to the config
    /// let config = Config::builder().locations(location_data_map).build().unwrap();
    ///
    /// // Create game and clear database for a blank test state
    /// let game = Game::new("postgresql://admin:root@db:5432/test_db", Some(config)).unwrap();
    /// game.database().write().unwrap().clear().unwrap();
    ///
    /// // Registering a new user
    /// let user = game.user_register(EXTERNAL_ID).unwrap();
    ///
    /// // Let the user unlock a location
    /// let island_location = game.location_find(LOCATION_ID).unwrap();
    /// let unlocked_location = game.user_unlock_location(&user, island_location).unwrap();
    ///
    /// // Get unlocked locations
    /// let unlocked_locations = game.user_get_unlocked_locations(&user).unwrap();
    /// assert_eq!(unlocked_locations.len(), 1);
    /// assert_eq!(unlocked_locations[0], unlocked_location);
    /// ```
    fn user_get_unlocked_locations(&self, user: &User) -> GameResult<Vec<UserLocationUnlock>> {
        let user_locations = self.user_service().get_unlocked_locations(user)?;
        let location_unlocks =
            UserLocationUnlock::from_user_locations(user_locations, |location_id| {
                self.location_find(location_id).ok()
            });
        Ok(location_unlocks)
    }

    /// Register a new [User] by their external ID.
    ///
    /// # Arguments
    ///
    /// * `external_id`: A freely selectable ID that your system will use to identify this [User].
    ///
    /// # Returns
    ///
    /// Result<[User], [errors::GameError]>
    /// - A newly created [User] with the given external id
    /// - An error, if:
    ///     - A [User] with the given external id already exists
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
    /// let user = game.user_register(EXTERNAL_ID).unwrap();
    /// assert_eq!(user.external_id, EXTERNAL_ID);
    ///
    /// // Registering an already existing user
    /// let error = game.user_register(EXTERNAL_ID).unwrap_err();
    /// assert!(error.is_already_exists());
    /// if let Some(resource_error) = error.as_resource_error() {
    ///     assert!(resource_error.is_user_already_exists());
    ///     assert_eq!(resource_error.get_external_id(), Some(EXTERNAL_ID));
    /// } else {
    ///     panic!("{:?}", error);
    /// }
    /// ```
    fn user_register(&self, external_id: i64) -> GameResult<User> {
        match self.user_repository().find_by_external_id(external_id)? {
            Some(_) => Err(GameResourceError::user_already_exists(external_id).into()),
            None => Ok(self.user_service().create_and_save_user(external_id)?),
        }
    }

    /// Save a [User].
    ///
    /// # Arguments
    ///
    /// * `user`: The [User] to save
    ///
    /// # Returns
    /// Result<[User], [errors::GameError]>
    /// - The updated [User] entity, if saving succeeded
    /// - An error, if saving failed (database errors, or the user doesn't exist)
    ///
    /// # Examples
    ///
    /// ```
    /// use fish_lib::game::prelude::*;
    /// use fish_lib::game::service_provider::ServiceProviderInterface;    ///
    ///
    /// use fish_lib::models::user::User;
    ///
    /// const DUMMY_USER_ID: i64 = 64;
    /// const USER_EXTERNAL_ID: i64 = 1337;
    /// const USER_CREDITS: i64 = 293;
    ///
    /// // Create game and clear database for a blank test state
    /// let game = Game::new("postgresql://admin:root@db:5432/test_db", None).unwrap();
    /// game.database().write().unwrap().clear().unwrap();
    ///
    /// // Create a new user and update their credits
    /// let mut user = game.user_register(USER_EXTERNAL_ID).unwrap();
    /// user.credits = USER_CREDITS;
    ///
    /// // Save the user and check if the credits were updated properly
    /// let updated_user = game.user_save(user).unwrap();
    /// assert_eq!(updated_user.credits, USER_CREDITS);
    ///
    /// // Find user again and check if credits are updated properly
    /// let found_user = game.user_find(USER_EXTERNAL_ID).unwrap();
    /// assert_eq!(found_user.credits, USER_CREDITS);
    ///
    /// // Try to save a non-existent user
    /// let dummy_user = User {
    ///     id: DUMMY_USER_ID,
    ///     ..Default::default()
    /// };
    ///
    /// let error_not_found = game.user_save(dummy_user).unwrap_err();
    /// assert!(error_not_found.is_not_found())
    /// ```
    fn user_save(&self, user: User) -> GameResult<User> {
        Ok(self.user_repository().save(user)?)
    }

    /// Unlocks a given location for a given user
    ///
    /// # Arguments
    ///
    /// * `user`: The [User] to unlock a location for
    /// * `location`: The location to unlock for the given [User]
    ///
    /// # Returns
    /// Result<[UserLocationUnlock], [errors::GameError]>
    /// - Information about the location unlock, if it succeeded
    /// - An error, if:
    ///     - the location does not exist
    ///     - the location was already unlocked
    ///     - database operations fail
    ///
    /// # Examples
    ///
    /// ```
    /// use fish_lib::game::prelude::*;
    /// use std::collections::HashMap;
    /// use fish_lib::config::{Config, ConfigBuilderInterface};
    /// use fish_lib::data::location_data::LocationData;
    /// use fish_lib::game::prelude::*;
    /// use fish_lib::game::service_provider::ServiceProviderInterface;
    ///
    /// const EXTERNAL_ID: i64 = 1337;
    /// const LOCATION_ID: i32 = 13;
    /// const LOCATION_NAME: &str = "Island";
    ///
    /// // Define some location data
    /// let location_data = LocationData {
    ///     name: LOCATION_NAME.to_string(),
    ///     ..Default::default()
    /// };
    /// let location_data_map = HashMap::from([(LOCATION_ID, location_data)]);
    ///
    /// // Add the location data to the config
    /// let config = Config::builder().locations(location_data_map).build().unwrap();
    ///
    /// // Create game and clear database for a blank test state
    /// let game = Game::new("postgresql://admin:root@db:5432/test_db", Some(config)).unwrap();
    /// game.database().write().unwrap().clear().unwrap();
    ///
    /// // Registering a new user
    /// let user = game.user_register(EXTERNAL_ID).unwrap();
    ///
    /// // Unlock a location for the user
    /// let island = game.location_find(LOCATION_ID).unwrap();
    /// let location_unlock = game.user_unlock_location(&user, island).unwrap();
    ///
    /// // Find unlocked locations
    /// let unlocked_locations = game.user_get_unlocked_locations(&user).unwrap();
    /// assert_eq!(unlocked_locations.len(), 1);
    /// assert_eq!(unlocked_locations[0], location_unlock);
    /// ```
    fn user_unlock_location(
        &self,
        user: &User,
        location: Arc<LocationData>,
    ) -> GameResult<UserLocationUnlock> {
        let user_location = self
            .user_service()
            .unlock_location(user, location.clone())?;
        let user_location_unlock =
            UserLocationUnlock::from_user_location(user_location, &|location_id| {
                self.location_find(location_id).ok()
            });
        match user_location_unlock {
            Some(location_unlock) => Ok(location_unlock),
            None => Err(GameResourceError::location_not_found(location.id).into()),
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

    fn location_service(&self) -> Arc<dyn LocationServiceInterface> {
        self.service_provider.location_service()
    }

    fn pond_service(&self) -> Arc<dyn PondServiceInterface> {
        self.service_provider.pond_service()
    }

    fn species_service(&self) -> Arc<dyn SpeciesServiceInterface> {
        self.service_provider.species_service()
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
