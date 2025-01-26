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
    pub last_catch: DateTime<Utc>,
    pub first_sell: Option<DateTime<Utc>>,
    pub last_sell: Option<DateTime<Utc>>,
}

impl FishingHistoryEntry {
    pub fn first_catch(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn register_catch(&mut self, size_mm: f32, catch_time: DateTime<Utc>) {
        if size_mm < self.smallest_catch_mm {
            self.smallest_catch_mm = size_mm;
        } else if size_mm > self.largest_catch_mm {
            self.largest_catch_mm = size_mm;
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
