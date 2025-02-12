use crate::models::item::attributes::bait::BaitAttributes;
use crate::models::item::attributes::rod::RodAttributes;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod bait;
pub mod rod;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemAttributesType {
    Bait,
    Rod,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemAttributes {
    Bait(BaitAttributes),
    Rod(RodAttributes),
}

impl ItemAttributes {
    pub fn bait(level: u64) -> Self {
        Self::Bait(BaitAttributes::new(level))
    }

    pub fn rod(level: u64) -> Self {
        Self::Rod(RodAttributes::new(level))
    }
}

pub trait ItemAttributesContainerInterface {
    fn get_attributes(&self) -> &HashMap<ItemAttributesType, ItemAttributes>;

    fn get_bait_attributes(&self) -> Option<&BaitAttributes> {
        match self.get_attributes().get(&ItemAttributesType::Bait) {
            Some(ItemAttributes::Bait(bait)) => Some(bait),
            Some(_) | None => None,
        }
    }

    fn get_rod_attributes(&self) -> Option<&RodAttributes> {
        match self.get_attributes().get(&ItemAttributesType::Rod) {
            Some(ItemAttributes::Rod(rod)) => Some(rod),
            Some(_) | None => None,
        }
    }

    fn is_bait(&self) -> bool {
        self.get_bait_attributes().is_some()
    }

    fn is_rod(&self) -> bool {
        self.get_rod_attributes().is_some()
    }

    // Attribute specific values
    fn get_bait_level(&self) -> Option<u64> {
        self.get_bait_attributes().map(|bait| bait.get_level())
    }
    fn get_rod_level(&self) -> Option<u64> {
        self.get_rod_attributes().map(|rod| rod.get_level())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct ItemAttributesContainer {
    components: HashMap<ItemAttributesType, ItemAttributes>,
}

impl ItemAttributesContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_component(&mut self, component: ItemAttributes) {
        match component {
            ItemAttributes::Bait(_) => self.components.insert(ItemAttributesType::Bait, component),
            ItemAttributes::Rod(_) => self.components.insert(ItemAttributesType::Rod, component),
        };
    }

    pub fn with_bait(mut self, level: u64) -> Self {
        let component = ItemAttributes::bait(level);
        self.add_component(component);
        self
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

// Serde
impl<'de> Deserialize<'de> for ItemAttributesContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            components: HashMap<String, serde_json::Value>,
        }

        let helper = Helper::deserialize(deserializer)?;
        let mut container = ItemAttributesContainer::new();

        for (type_str, attr_value) in helper.components {
            let (attr_type, attribute) = match type_str.as_str() {
                "Rod" => {
                    let rod =
                        serde_json::from_value(attr_value).map_err(serde::de::Error::custom)?;
                    (ItemAttributesType::Rod, ItemAttributes::Rod(rod))
                }
                "Bait" => {
                    let bait =
                        serde_json::from_value(attr_value).map_err(serde::de::Error::custom)?;
                    (ItemAttributesType::Bait, ItemAttributes::Bait(bait))
                }
                _ => continue,
            };

            container.components.insert(attr_type, attribute);
        }

        Ok(container)
    }
}
