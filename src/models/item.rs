use crate::config::ConfigInterface;
use crate::game::errors::item_event::GameItemEventError;
use crate::models::item::attributes::ItemAttributesContainerInterface;
use crate::models::item::components::{ItemComponent, ItemComponentType};
use crate::models::item::properties::{ItemProperties, ItemPropertiesInterface};
use crate::traits::model::Model;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub mod attributes;
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

pub type ItemEventResult = Result<ItemEventSuccess, GameItemEventError>;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ItemEventSuccess {
    pub consume: bool,
}

impl ItemEventSuccess {
    pub fn new(consume: bool) -> Self {
        Self { consume }
    }
}

impl Item {
    pub fn use_as_rod(&mut self, config: Arc<dyn ConfigInterface>) -> ItemEventResult {
        let attributes = self
            .attributes(config)
            .ok_or(GameItemEventError::invalid_item_type(self.type_id))?;

        if !attributes.is_rod() {
            Err(GameItemEventError::not_a_rod(self.type_id))
        } else {
            self.properties.on_use();
            Ok(ItemEventSuccess::new(false))
        }
    }
}

pub trait ItemInterface: ItemPropertiesInterface {
    fn attributes(
        &self,
        config: Arc<dyn ConfigInterface>,
    ) -> Option<Arc<dyn ItemAttributesContainerInterface>>;
}

impl ItemInterface for Item {
    fn attributes(
        &self,
        config: Arc<dyn ConfigInterface>,
    ) -> Option<Arc<dyn ItemAttributesContainerInterface>> {
        config
            .get_item_data(self.type_id)
            .map(|data| data as Arc<dyn ItemAttributesContainerInterface>)
    }
}

impl ItemPropertiesInterface for Item {
    fn get_components(&self) -> &HashMap<ItemComponentType, ItemComponent> {
        self.properties.get_components()
    }

    fn get_components_mut(&mut self) -> &mut HashMap<ItemComponentType, ItemComponent> {
        self.properties.get_components_mut()
    }
}
