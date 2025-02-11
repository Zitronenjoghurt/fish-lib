use crate::models::item::attributes::rod::RodAttributes;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod rod;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemAttributesType {
    Rod,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemAttributes {
    Rod(RodAttributes),
}

impl ItemAttributes {
    pub fn rod(level: u64) -> Self {
        Self::Rod(RodAttributes::new(level))
    }
}

pub trait ItemAttributesContainerInterface {
    fn get_attributes(&self) -> &HashMap<ItemAttributesType, ItemAttributes>;
    fn get_rod_attributes(&self) -> Option<&RodAttributes> {
        match self.get_attributes().get(&ItemAttributesType::Rod) {
            Some(ItemAttributes::Rod(rod)) => Some(rod),
            None => None,
        }
    }
    fn is_rod(&self) -> bool {
        self.get_rod_attributes().is_some()
    }

    // Attribute specific values
    fn get_rod_level(&self) -> Option<u64> {
        self.get_rod_attributes().map(|rod| rod.get_level())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemAttributesContainer {
    components: HashMap<ItemAttributesType, ItemAttributes>,
}

impl ItemAttributesContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_component(&mut self, component: ItemAttributes) {
        match component {
            ItemAttributes::Rod(_) => self.components.insert(ItemAttributesType::Rod, component),
        };
    }

    pub fn with_rod(mut self, level: u64) -> Self {
        let component = ItemAttributes::rod(level);
        self.add_component(component);
        self
    }
}

impl ItemAttributesContainerInterface for ItemAttributesContainer {
    fn get_attributes(&self) -> &HashMap<ItemAttributesType, ItemAttributes> {
        &self.components
    }
}
