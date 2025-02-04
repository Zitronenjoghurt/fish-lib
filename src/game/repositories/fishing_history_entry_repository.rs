use crate::database::DatabaseInterface;
use crate::game::errors::repository::GameRepositoryError;
use crate::models::fishing_history_entry::{FishingHistoryEntry, NewFishingHistoryEntry};
use crate::schema::fish_fishing_history_entries;
use crate::traits::repository::Repository;
use chrono::Utc;
use diesel::prelude::*;
use std::sync::{Arc, RwLock};

pub trait FishingHistoryEntryRepositoryInterface:
    Repository<FishingHistoryEntry> + Send + Sync
{
    fn find_by_user_and_species_id(
        &self,
        user_id: i64,
        species_id: i32,
    ) -> Result<Option<FishingHistoryEntry>, GameRepositoryError>;
}

pub struct FishingHistoryEntryRepository {
    db: Arc<RwLock<dyn DatabaseInterface>>,
}

impl FishingHistoryEntryRepository {
    pub fn new(db: Arc<RwLock<dyn DatabaseInterface>>) -> Self {
        Self { db }
    }
}

impl FishingHistoryEntryRepositoryInterface for FishingHistoryEntryRepository {
    fn find_by_user_and_species_id(
        &self,
        user_id: i64,
        species_id: i32,
    ) -> Result<Option<FishingHistoryEntry>, GameRepositoryError> {
        let mut connection = self.get_connection()?;
        let entry = fish_fishing_history_entries::table
            .filter(
                fish_fishing_history_entries::user_id
                    .eq(user_id)
                    .and(fish_fishing_history_entries::species_id.eq(species_id)),
            )
            .first::<FishingHistoryEntry>(&mut connection)
            .optional()?;
        Ok(entry)
    }
}

impl Repository<FishingHistoryEntry> for FishingHistoryEntryRepository {
    fn get_db(&self) -> Arc<RwLock<dyn DatabaseInterface>> {
        self.db.clone()
    }

    fn create(
        &self,
        new_entity: NewFishingHistoryEntry,
    ) -> Result<FishingHistoryEntry, GameRepositoryError> {
        let mut connection = self.get_connection()?;

        let new_result = diesel::insert_into(fish_fishing_history_entries::table)
            .values(new_entity)
            .get_result::<FishingHistoryEntry>(&mut connection)?;

        Ok(new_result)
    }

    fn find(&self, id: i64) -> Result<Option<FishingHistoryEntry>, GameRepositoryError> {
        let mut connection = self.get_connection()?;
        let result = fish_fishing_history_entries::table
            .find(id)
            .first::<FishingHistoryEntry>(&mut connection)
            .optional()?;
        Ok(result)
    }

    fn save(
        &self,
        mut entity: FishingHistoryEntry,
    ) -> Result<FishingHistoryEntry, GameRepositoryError> {
        let mut connection = self.get_connection()?;
        entity.updated_at = Utc::now();

        let update_result = diesel::update(fish_fishing_history_entries::table)
            .filter(fish_fishing_history_entries::id.eq(entity.id))
            .set(entity)
            .get_result::<FishingHistoryEntry>(&mut connection)?;

        Ok(update_result)
    }

    fn delete(&self, entity: FishingHistoryEntry) -> Result<bool, GameRepositoryError> {
        let mut connection = self.get_connection()?;

        let deleted_count = diesel::delete(fish_fishing_history_entries::table)
            .filter(fish_fishing_history_entries::id.eq(entity.id))
            .execute(&mut connection)?;

        Ok(deleted_count > 0)
    }
}
