use crate::get_db_connection;
use crate::models::fishing_history_entry::{FishingHistoryEntry, NewFishingHistoryEntry};
use crate::schema::fish_fishing_history_entries;
use crate::traits::repository::Repository;
use chrono::Utc;
use diesel::prelude::*;
use std::error::Error;

pub struct FishingHistoryEntryRepository;

impl FishingHistoryEntryRepository {
    pub fn find_by_user_and_species_id(
        user_id: i64,
        species_id: i32,
    ) -> Result<Option<FishingHistoryEntry>, Box<dyn Error>> {
        let mut connection = get_db_connection()?;
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
    fn create(new_entity: NewFishingHistoryEntry) -> Result<FishingHistoryEntry, Box<dyn Error>> {
        let mut connection = get_db_connection()?;

        let new_result = diesel::insert_into(fish_fishing_history_entries::table)
            .values(new_entity)
            .get_result::<FishingHistoryEntry>(&mut connection)?;

        Ok(new_result)
    }

    fn find(id: i64) -> Result<Option<FishingHistoryEntry>, Box<dyn Error>> {
        let mut connection = get_db_connection()?;
        let result = fish_fishing_history_entries::table
            .find(id)
            .first::<FishingHistoryEntry>(&mut connection)
            .optional()?;
        Ok(result)
    }

    fn save(mut entity: FishingHistoryEntry) -> Result<FishingHistoryEntry, Box<dyn Error>> {
        let mut connection = get_db_connection()?;
        entity.updated_at = Utc::now();

        let update_result = diesel::update(fish_fishing_history_entries::table)
            .filter(fish_fishing_history_entries::id.eq(entity.id))
            .set(entity)
            .get_result::<FishingHistoryEntry>(&mut connection)?;

        Ok(update_result)
    }

    fn delete(entity: &FishingHistoryEntry) -> Result<bool, Box<dyn Error>> {
        let mut connection = get_db_connection()?;

        let deleted_count = diesel::delete(fish_fishing_history_entries::table)
            .filter(fish_fishing_history_entries::id.eq(entity.id))
            .execute(&mut connection)?;

        Ok(deleted_count > 0)
    }
}
