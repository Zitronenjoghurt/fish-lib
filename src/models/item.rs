use crate::enums::item_type::ItemType;
use crate::models::item::components::usage_count::UsageComponent;
use crate::models::item::properties::{ItemProperties, ItemPropertiesInterface};
use crate::traits::model::Model;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

pub mod components;
pub mod properties;

#[derive(
    Debug, Default, Serialize, Deserialize, Clone, PartialEq, Queryable, Selectable, AsChangeset,
)]
#[diesel(table_name = crate::schema::fish_items)]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub type_id: i32,
    pub count: i64,
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
    pub count: i64,
    pub properties: ItemProperties,
}

impl ItemPropertiesInterface for Item {
    fn get_item_type(&self) -> ItemType {
        self.properties.get_item_type()
    }

    fn get_usage_component(&self) -> Option<&UsageComponent> {
        self.properties.get_usage_component()
    }

    fn get_usage_component_mut(&mut self) -> Option<&mut UsageComponent> {
        self.properties.get_usage_component_mut()
    }
}
