use crate::models::item::properties::{ItemProperties, ItemPropertiesInterface, RodProperties};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ItemData {
    #[serde(skip, default)]
    pub id: i32,
    pub name: String,
    #[serde(default)]
    pub default_properties: ItemProperties,
}

impl ItemPropertiesInterface for ItemData {
    fn is_none(&self) -> bool {
        self.default_properties.is_none()
    }

    fn is_rod(&self) -> bool {
        self.default_properties.is_rod()
    }

    fn as_rod(&self) -> Option<&RodProperties> {
        self.default_properties.as_rod()
    }

    fn get_times_used(&self) -> Option<i64> {
        self.default_properties.get_times_used()
    }

    fn increment_times_used(&mut self) {}
}
