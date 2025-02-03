use crate::game::errors::resource::GameResourceError;
use crate::game::errors::GameResult;
use crate::game::interface::GameInterface;
use crate::game::repositories::user_repository::UserRepository;
use crate::game::services::user_service::UserService;
use crate::models::user::User;

pub mod errors;
pub mod interface;
pub mod prelude;
pub mod repositories;
pub mod services;
pub mod systems;

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
    /// // Registering a new user
    /// let user = Game::register_user(1337).unwrap();
    /// assert_eq!(user.external_id, 1337);
    ///
    /// // Registering an already existing user
    /// let error = Game::register_user(1337).unwrap_err();
    /// if let Some(resource_error) = error.as_resource_error() {
    ///     assert!(resource_error.is_user_already_exists());
    ///     assert_eq!(resource_error.get_external_id(), Some(1337));
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
    /// // Finding an existing user
    /// let new_user = Game::register_user(1337).unwrap();
    /// let found_user = Game::find_user(1337).unwrap();
    /// assert_eq!(new_user, found_user);
    ///
    /// // Searching for a non-existent user
    /// let error = Game::find_user(1338).unwrap_err();
    /// if let Some(resource_error) = error.as_resource_error() {
    ///     assert!(resource_error.is_user_not_found());
    ///     assert_eq!(resource_error.get_external_id(), Some(1338));
    /// } else {
    ///     panic!("{:?}", error);
    /// }
    /// ```
    fn find_user(external_id: i64) -> GameResult<User> {
        match UserRepository::find_by_external_id(external_id)? {
            Some(user) => Ok(user),
            None => Err(GameResourceError::user_not_found(external_id).into()),
        }
    }
}
