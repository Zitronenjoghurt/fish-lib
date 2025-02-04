use crate::config::ConfigInterface;
use crate::game::errors::resource::GameResourceError;
use crate::game::errors::GameResult;
use crate::traits::model::Model;
use crate::utils::math::float_interpolate;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::fish_fishing_history_entries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FishingHistoryEntry {
    pub id: i64,
    pub user_id: i64,
    pub species_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub caught_count: i32,
    pub sold_count: i32,
    pub smallest_catch_size_ratio: f32,
    pub largest_catch_size_ratio: f32,
    pub last_catch: DateTime<Utc>,
    pub first_sell: Option<DateTime<Utc>>,
    pub last_sell: Option<DateTime<Utc>>,
}

impl FishingHistoryEntry {
    pub fn get_first_catch(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn register_catch(&mut self, total_size_ratio: f32, catch_time: DateTime<Utc>) {
        if total_size_ratio < self.smallest_catch_size_ratio {
            self.smallest_catch_size_ratio = total_size_ratio;
        } else if total_size_ratio > self.largest_catch_size_ratio {
            self.largest_catch_size_ratio = total_size_ratio;
        }
        self.last_catch = catch_time;
        self.caught_count = self.caught_count.saturating_add(1);
    }

    pub fn register_sell(&mut self, sell_time: DateTime<Utc>) {
        if self.sold_count == 0 {
            self.first_sell = Some(sell_time);
        }
        self.last_sell = Some(sell_time);
        self.sold_count = self.sold_count.saturating_add(1);
    }

    pub fn get_smallest_size_mm(&self, config: Arc<dyn ConfigInterface>) -> GameResult<f32> {
        let data = config
            .get_species_data(self.species_id)
            .ok_or_else(|| GameResourceError::species_not_found(self.species_id))?;
        Ok(float_interpolate(
            data.min_size_baby_mm as f32,
            data.max_size_adult_mm as f32,
            self.smallest_catch_size_ratio,
        ))
    }

    pub fn get_largest_size_mm(&self, config: Arc<dyn ConfigInterface>) -> GameResult<f32> {
        let data = config
            .get_species_data(self.species_id)
            .ok_or_else(|| GameResourceError::species_not_found(self.species_id))?;
        Ok(float_interpolate(
            data.min_size_baby_mm as f32,
            data.max_size_adult_mm as f32,
            self.largest_catch_size_ratio,
        ))
    }

    pub fn get_smallest_weight_g(&self, config: Arc<dyn ConfigInterface>) -> GameResult<f32> {
        let data = config
            .get_species_data(self.species_id)
            .ok_or_else(|| GameResourceError::species_not_found(self.species_id))?;
        Ok(float_interpolate(
            data.min_weight_baby_g as f32,
            data.max_weight_adult_g as f32,
            self.smallest_catch_size_ratio,
        ))
    }

    pub fn get_largest_weight_g(&self, config: Arc<dyn ConfigInterface>) -> GameResult<f32> {
        let data = config
            .get_species_data(self.species_id)
            .ok_or_else(|| GameResourceError::species_not_found(self.species_id))?;
        Ok(float_interpolate(
            data.min_weight_baby_g as f32,
            data.max_weight_adult_g as f32,
            self.largest_catch_size_ratio,
        ))
    }
}

impl Model for FishingHistoryEntry {
    type Table = crate::schema::fish_fishing_history_entries::table;
    type PrimaryKeyType = i64;
    type InsertType = NewFishingHistoryEntry;

    fn table() -> Self::Table {
        crate::schema::fish_fishing_history_entries::table
    }

    fn id(&self) -> Self::PrimaryKeyType {
        self.id
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::fish_fishing_history_entries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewFishingHistoryEntry {
    pub user_id: i64,
    pub species_id: i32,
    pub caught_count: i32,
    pub sold_count: i32,
    pub smallest_catch_size_ratio: f32,
    pub largest_catch_size_ratio: f32,
}
