use crate::enums::item_type::ItemType;
use crate::models::item::components::usage_count::UsageComponent;
use crate::models::item::properties::{ItemProperties, ItemPropertiesInterface};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ItemData {
    #[serde(skip, default)]
    pub id: i32,
    pub name: String,
    pub stackable: bool,
    #[serde(default)]
    pub default_properties: ItemProperties,
}

impl ItemPropertiesInterface for ItemData {
    fn get_item_type(&self) -> ItemType {
        self.default_properties.get_item_type()
    }

    fn get_usage_component(&self) -> Option<&UsageComponent> {
        self.default_properties.get_usage_component()
    }

    fn get_usage_component_mut(&mut self) -> Option<&mut UsageComponent> {
        self.default_properties.get_usage_component_mut()
    }
}
