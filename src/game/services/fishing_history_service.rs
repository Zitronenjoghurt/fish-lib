use crate::game::repositories::fishing_history_entry_repository::FishingHistoryEntryRepository;
use crate::get_config;
use crate::models::fish::Fish;
use crate::models::fishing_history_entry::{FishingHistoryEntry, NewFishingHistoryEntry};
use crate::traits::repository::Repository;
use std::error::Error;

pub struct FishingHistoryService;

impl FishingHistoryService {
    pub fn register_catch(fish: &Fish) -> Result<FishingHistoryEntry, Box<dyn Error>> {
        let existing_entry = FishingHistoryEntryRepository::find_by_user_and_species_id(
            fish.user_id,
            fish.species_id,
        )?;

        let time_multiplier = get_config().settings.time_speed_multiplier;

        if let Some(mut entry) = existing_entry {
            let size_mm = fish.get_size_mm(time_multiplier);
            entry.register_catch(size_mm);
            let saved_entry = FishingHistoryEntryRepository::save(entry)?;
            Ok(saved_entry)
        } else {
            let size_mm = fish.get_size_mm(time_multiplier);
            let new_entry = NewFishingHistoryEntry {
                user_id: fish.user_id,
                species_id: fish.species_id,
                caught_count: 1,
                sold_count: 0,
                smallest_catch_mm: size_mm,
                largest_catch_mm: size_mm,
            };
            let saved_entry = FishingHistoryEntryRepository::create(new_entry)?;
            Ok(saved_entry)
        }
    }
}
