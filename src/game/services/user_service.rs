use crate::data::location_data::LocationData;
use crate::game::errors::resource::GameResourceError;
use crate::game::errors::GameResult;
use crate::game::repositories::user_repository::UserRepositoryInterface;
use crate::models::user::{NewUser, User};
use crate::models::user_location::UserLocation;
use std::sync::Arc;

pub trait UserServiceInterface: Send + Sync {
    fn create_and_save_user(&self, external_id: i64) -> GameResult<User>;
    fn unlock_location(
        &self,
        user: &User,
        location_data: Arc<LocationData>,
    ) -> GameResult<UserLocation>;
    fn get_unlocked_locations(&self, user: &User) -> GameResult<Vec<UserLocation>>;
    fn get_unlocked_location_ids(&self, user: &User) -> GameResult<Vec<i32>>;
}

pub struct UserService {
    user_repository: Arc<dyn UserRepositoryInterface>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepositoryInterface>) -> UserService {
        UserService { user_repository }
    }
}

impl UserServiceInterface for UserService {
    fn create_and_save_user(&self, external_id: i64) -> GameResult<User> {
        let user = NewUser { external_id };
        Ok(self.user_repository.create(user)?)
    }

    fn unlock_location(
        &self,
        user: &User,
        location_data: Arc<LocationData>,
    ) -> GameResult<UserLocation> {
        self.user_repository
            .unlock_location(user.id, location_data.id)
            .map_err(|e| match e.get_database_error() {
                Some(db_error) if db_error.is_unique_constraint_violation() => {
                    GameResourceError::location_already_unlocked(user.external_id, location_data.id)
                        .into()
                }
                _ => e.into(),
            })
    }

    fn get_unlocked_locations(&self, user: &User) -> GameResult<Vec<UserLocation>> {
        Ok(self.user_repository.find_unlocked_locations(user.id)?)
    }

    fn get_unlocked_location_ids(&self, user: &User) -> GameResult<Vec<i32>> {
        Ok(self.user_repository.find_unlocked_location_ids(user.id)?)
    }
}
