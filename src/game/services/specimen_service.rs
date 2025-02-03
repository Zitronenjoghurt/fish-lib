use crate::game::repositories::specimen_repository::SpecimenRepository;
use crate::game::services::fishing_history_service::FishingHistoryService;
use crate::models::fishing_history_entry::FishingHistoryEntry;
use crate::models::specimen::{NewSpecimen, Specimen};
use crate::models::user::User;
use crate::traits::repository::Repository;
use std::error::Error;

pub struct SpecimenService;

impl SpecimenService {
    pub fn generate_and_save_specimen(
        owner_user: &User,
        species_id: i32,
    ) -> Result<Specimen, Box<dyn Error>> {
        let new_fish = NewSpecimen::generate(owner_user.id, species_id)?;
        SpecimenRepository::create(new_fish)
    }

    pub fn process_catch(
        user: &User,
        species_id: i32,
    ) -> Result<(Specimen, FishingHistoryEntry), Box<dyn Error>> {
        let fish = Self::generate_and_save_specimen(user, species_id)?;
        let history_entry = FishingHistoryService::register_catch(&fish)?;
        Ok((fish, history_entry))
    }
}
