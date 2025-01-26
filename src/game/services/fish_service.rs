use crate::game::repositories::fish_repository::FishRepository;
use crate::game::services::fishing_history_service::FishingHistoryService;
use crate::models::fish::{Fish, NewFish};
use crate::models::fishing_history_entry::FishingHistoryEntry;
use crate::models::user::User;
use crate::traits::repository::Repository;
use std::error::Error;

pub struct FishService;

impl FishService {
    pub fn generate_and_save_fish(
        owner_user: &User,
        species_id: i32,
    ) -> Result<Fish, Box<dyn Error>> {
        let new_fish = NewFish::generate(owner_user.id, species_id)?;
        FishRepository::create(new_fish)
    }

    pub fn process_catch(
        user: &User,
        species_id: i32,
    ) -> Result<(Fish, FishingHistoryEntry), Box<dyn Error>> {
        let fish = Self::generate_and_save_fish(user, species_id)?;
        let history_entry = FishingHistoryService::register_catch(&fish)?;
        Ok((fish, history_entry))
    }
}
