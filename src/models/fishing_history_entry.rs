use crate::traits::model::Model;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};

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
    pub smallest_catch_mm: f32,
    pub largest_catch_mm: f32,
}

impl FishingHistoryEntry {
    pub fn register_catch(&mut self, size_mm: f32) {
        if size_mm < self.smallest_catch_mm {
            self.smallest_catch_mm = size_mm;
        } else if size_mm > self.largest_catch_mm {
            self.largest_catch_mm = size_mm;
        }
        self.caught_count = self.caught_count.saturating_add(1);
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
    pub smallest_catch_mm: f32,
    pub largest_catch_mm: f32,
}
