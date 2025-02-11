use crate::models::item::attributes::{
    ItemAttributes, ItemAttributesContainer, ItemAttributesContainerInterface, ItemAttributesType,
};
use crate::models::item::components::{ItemComponent, ItemComponentType};
use crate::models::item::properties::{ItemProperties, ItemPropertiesInterface};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ItemData {
    #[serde(skip, default)]
    pub id: i32,
    pub name: String,
    pub stackable: bool,
    #[serde(default)]
    pub attributes: ItemAttributesContainer,
    #[serde(default)]
    pub default_properties: ItemProperties,
}

impl ItemPropertiesInterface for ItemData {
    fn get_components(&self) -> &HashMap<ItemComponentType, ItemComponent> {
        self.default_properties.get_components()
    }

    fn get_components_mut(&mut self) -> &mut HashMap<ItemComponentType, ItemComponent> {
        self.default_properties.get_components_mut()
    }
}

impl ItemAttributesContainerInterface for ItemData {
    fn get_attributes(&self) -> &HashMap<ItemAttributesType, ItemAttributes> {
        self.attributes.get_attributes()
    }
}
