use crate::models::item::properties::{ItemProperties, ItemPropertiesInterface, RodProperties};
use crate::traits::model::Model;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

pub mod properties;

#[derive(
    Debug, Default, Serialize, Deserialize, Clone, PartialEq, Queryable, Selectable, AsChangeset,
)]
#[diesel(table_name = crate::schema::fish_items)]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub type_id: i32,
    pub properties: ItemProperties,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Model for Item {
    type Table = crate::schema::fish_items::table;
    type PrimaryKeyType = i64;
    type InsertType = NewItem;

    fn table() -> Self::Table {
        crate::schema::fish_items::table
    }

    fn id(&self) -> Self::PrimaryKeyType {
        self.id
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::fish_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewItem {
    pub user_id: i64,
    pub type_id: i32,
    pub properties: ItemProperties,
}

impl ItemPropertiesInterface for Item {
    fn is_none(&self) -> bool {
        self.properties.is_none()
    }

    fn is_rod(&self) -> bool {
        self.properties.is_rod()
    }

    fn as_rod(&self) -> Option<&RodProperties> {
        self.properties.as_rod()
    }

    fn get_times_used(&self) -> Option<i64> {
        self.properties.get_times_used()
    }

    fn increment_times_used(&mut self) {
        self.properties.increment_times_used();
    }
}
