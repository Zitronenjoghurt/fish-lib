use crate::config::ConfigInterface;
use crate::game::errors::item_event::GameItemEventError;
use crate::models::item::attributes::ItemAttributesContainerInterface;
use crate::models::item::components::{ItemComponent, ItemComponentType};
use crate::models::item::properties::{ItemProperties, ItemPropertiesInterface};
use crate::traits::model::Model;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
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
    pub fn migrate_properties(&mut self, config: Arc<dyn ConfigInterface>) -> ItemEventResult {
        let item_data = config
            .get_item_data(self.type_id)
            .ok_or(GameItemEventError::invalid_item_type(self.type_id))?;

        let required_property_types: HashSet<ItemComponentType> =
            item_data.default_properties.get_component_types();

        let existing_property_types: HashSet<ItemComponentType> =
            self.properties.get_component_types();

        let properties_to_add: HashSet<ItemComponentType> = required_property_types
            .difference(&existing_property_types)
            .copied()
            .collect();

        let properties_to_remove: HashSet<ItemComponentType> = existing_property_types
            .difference(&required_property_types)
            .copied()
            .collect();

        properties_to_add.iter().for_each(|component_type| {
            let component = component_type.get_default_component();
            self.properties.add_component(component);
        });

        properties_to_remove
            .iter()
            .for_each(|component_type| self.properties.remove_component(*component_type));

        Ok(ItemEventSuccess::new(self.should_consume()))
    }

    pub fn use_as_rod(&mut self, config: Arc<dyn ConfigInterface>) -> ItemEventResult {
        let attributes = self
            .attributes(config)
            .ok_or(GameItemEventError::invalid_item_type(self.type_id))?;

        if !attributes.is_rod() {
            Err(GameItemEventError::not_a_rod(self.type_id))
        } else {
            self.properties.on_use(1);
            Ok(ItemEventSuccess::new(self.should_consume()))
        }
    }

    pub fn add(&mut self, amount: u64) -> ItemEventResult {
        self.on_add(amount);
        Ok(ItemEventSuccess::new(self.should_consume()))
    }

    pub fn remove(&mut self, amount: u64) -> ItemEventResult {
        self.on_remove(amount);
        Ok(ItemEventSuccess::new(self.should_consume()))
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
