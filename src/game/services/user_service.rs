use crate::data::location_data::LocationData;
use crate::dto::location_unlock_requirements::LocationUnlockRequirements;
use crate::game::errors::resource::GameResourceError;
use crate::game::errors::GameResult;
use crate::game::repositories::fishing_history_entry_repository::FishingHistoryEntryRepositoryInterface;
use crate::game::repositories::user_repository::UserRepositoryInterface;
use crate::models::user::{NewUser, User};
use crate::models::user_location::UserLocation;
use std::sync::Arc;

pub trait UserServiceInterface: Send + Sync {
    fn create_and_save_user(&self, external_id: i64) -> GameResult<User>;
    fn get_unmet_location_unlock_requirements(
        &self,
        user: &User,
        location: Arc<LocationData>,
    ) -> GameResult<LocationUnlockRequirements>;
    fn unlock_location(
        &self,
        user: &User,
        location_data: Arc<LocationData>,
    ) -> GameResult<UserLocation>;
    fn get_unlocked_locations(&self, user: &User) -> GameResult<Vec<UserLocation>>;
    fn get_unlocked_location_ids(&self, user: &User) -> GameResult<Vec<i32>>;
}

pub struct UserService {
    fishing_history_entry_repository: Arc<dyn FishingHistoryEntryRepositoryInterface>,
    user_repository: Arc<dyn UserRepositoryInterface>,
}

impl UserService {
    pub fn new(
        fishing_history_entry_repository: Arc<dyn FishingHistoryEntryRepositoryInterface>,
        user_repository: Arc<dyn UserRepositoryInterface>,
    ) -> UserService {
        UserService {
            fishing_history_entry_repository,
            user_repository,
        }
    }
}

impl UserServiceInterface for UserService {
    fn create_and_save_user(&self, external_id: i64) -> GameResult<User> {
        let user = NewUser { external_id };
        Ok(self.user_repository.create(user)?)
    }

    fn get_unmet_location_unlock_requirements(
        &self,
        user: &User,
        location: Arc<LocationData>,
    ) -> GameResult<LocationUnlockRequirements> {
        let unlocked_location_ids = self.get_unlocked_location_ids(user)?;
        let missing_location_ids: Vec<i32> = location
            .required_locations_unlocked
            .iter()
            .copied()
            .filter(|required_location_id| !unlocked_location_ids.contains(required_location_id))
            .collect();

        let caught_species_ids = self
            .fishing_history_entry_repository
            .find_caught_species_ids_by_user(user.id)?;
        let missing_species_ids: Vec<i32> = location
            .required_species_caught
            .iter()
            .copied()
            .filter(|required_species_id| !caught_species_ids.contains(required_species_id))
            .collect();

        Ok(LocationUnlockRequirements {
            locations_unlocked: missing_location_ids,
            species_caught: missing_species_ids,
        })
    }

    fn unlock_location(
        &self,
        user: &User,
        location_data: Arc<LocationData>,
    ) -> GameResult<UserLocation> {
        let unmet_unlock_requirements =
            self.get_unmet_location_unlock_requirements(user, location_data.clone())?;
        if !unmet_unlock_requirements.is_empty() {
            return Err(
                GameResourceError::unmet_location_unlock_requirements(location_data.id).into(),
            );
        }

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
