use crate::config::ConfigInterface;
use crate::game::errors::resource::GameResourceError;
use crate::game::errors::GameResult;
use crate::game::repositories::fishing_history_entry_repository::FishingHistoryEntryRepositoryInterface;
use crate::models::fishing_history_entry::{FishingHistoryEntry, NewFishingHistoryEntry};
use crate::models::specimen::Specimen;
use chrono::{DateTime, Utc};
use std::sync::Arc;

pub trait FishingHistoryServiceInterface: Send + Sync {
    fn register_catch(&self, fish: &Specimen) -> GameResult<FishingHistoryEntry>;
    fn register_sell(
        &self,
        fish: &Specimen,
        sell_time: DateTime<Utc>,
    ) -> GameResult<FishingHistoryEntry>;
}

pub struct FishingHistoryService {
    config: Arc<dyn ConfigInterface>,
    fishing_history_entry_repository: Arc<dyn FishingHistoryEntryRepositoryInterface>,
}

impl FishingHistoryService {
    pub fn new(
        config: Arc<dyn ConfigInterface>,
        fishing_history_entry_repository: Arc<dyn FishingHistoryEntryRepositoryInterface>,
    ) -> Self {
        Self {
            config,
            fishing_history_entry_repository,
        }
    }
}

impl FishingHistoryServiceInterface for FishingHistoryService {
    fn register_catch(&self, fish: &Specimen) -> GameResult<FishingHistoryEntry> {
        let config = self.config.clone();

        let existing_entry = self
            .fishing_history_entry_repository
            .find_by_user_and_species_id(fish.user_id, fish.species_id)?;

        let time_multiplier = self.config.settings().time_speed_multiplier;

        if let Some(mut entry) = existing_entry {
            let total_size_ratio = fish.get_total_size_ratio(config, time_multiplier)?;
            entry.register_catch(total_size_ratio, fish.created_at);
            let saved_entry = self.fishing_history_entry_repository.save(entry)?;
            Ok(saved_entry)
        } else {
            let total_size_ratio = fish.get_total_size_ratio(config, time_multiplier)?;
            let new_entry = NewFishingHistoryEntry {
                user_id: fish.user_id,
                species_id: fish.species_id,
                caught_count: 1,
                sold_count: 0,
                smallest_catch_size_ratio: total_size_ratio,
                largest_catch_size_ratio: total_size_ratio,
            };
            let saved_entry = self.fishing_history_entry_repository.create(new_entry)?;
            Ok(saved_entry)
        }
    }

    fn register_sell(
        &self,
        fish: &Specimen,
        sell_time: DateTime<Utc>,
    ) -> GameResult<FishingHistoryEntry> {
        let mut existing_entry = self
            .fishing_history_entry_repository
            .find_by_user_and_species_id(fish.user_id, fish.species_id)?
            .ok_or_else(|| {
                GameResourceError::fishing_history_not_found(fish.user_id, fish.species_id)
            })?;

        existing_entry.register_sell(sell_time);
        Ok(self.fishing_history_entry_repository.save(existing_entry)?)
    }
}
