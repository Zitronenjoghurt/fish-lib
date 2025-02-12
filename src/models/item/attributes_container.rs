use crate::models::item::attributes::bait::BaitAttributes;
use crate::models::item::attributes::purchasable::PurchasableAttributes;
use crate::models::item::attributes::rod::RodAttributes;
use crate::models::item::attributes::{ItemAttributes, ItemAttributesType};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub trait ItemAttributesContainerInterface {
    fn get_attributes(&self) -> &HashMap<ItemAttributesType, ItemAttributes>;

    fn get_bait_attributes(&self) -> Option<&BaitAttributes> {
        match self.get_attributes().get(&ItemAttributesType::Bait) {
            Some(ItemAttributes::Bait(bait)) => Some(bait),
            Some(_) | None => None,
        }
    }

    fn get_purchasable_attributes(&self) -> Option<&PurchasableAttributes> {
        match self.get_attributes().get(&ItemAttributesType::Purchasable) {
            Some(ItemAttributes::Purchasable(purchasable)) => Some(purchasable),
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

    fn is_purchasable(&self) -> bool {
        self.get_purchasable_attributes().is_some()
    }

    fn is_rod(&self) -> bool {
        self.get_rod_attributes().is_some()
    }

    // Attribute specific values
    fn get_bait_level(&self) -> Option<u64> {
        self.get_bait_attributes().map(|bait| bait.get_level())
    }

    fn get_cost(&self) -> Option<u32> {
        self.get_purchasable_attributes()
            .map(|purchasable| purchasable.get_cost())
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

    pub fn get_attributes_types(&self) -> HashSet<ItemAttributesType> {
        self.components.keys().copied().collect()
    }

    pub fn add_component(&mut self, component: ItemAttributes) {
        match component {
            ItemAttributes::Bait(_) => self.components.insert(ItemAttributesType::Bait, component),
            ItemAttributes::Purchasable(_) => self
                .components
                .insert(ItemAttributesType::Purchasable, component),
            ItemAttributes::Rod(_) => self.components.insert(ItemAttributesType::Rod, component),
        };
    }

    pub fn with_bait(mut self, level: u64) -> Self {
        let component = ItemAttributes::bait(level);
        self.add_component(component);
        self
    }

    pub fn with_purchasable(mut self, cost: u32) -> Self {
        let component = ItemAttributes::purchasable(cost);
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
                "Bait" => {
                    let bait =
                        serde_json::from_value(attr_value).map_err(serde::de::Error::custom)?;
                    (ItemAttributesType::Bait, ItemAttributes::Bait(bait))
                }
                "Purchasable" => {
                    let purchasable =
                        serde_json::from_value(attr_value).map_err(serde::de::Error::custom)?;
                    (
                        ItemAttributesType::Purchasable,
                        ItemAttributes::Purchasable(purchasable),
                    )
                }
                "Rod" => {
                    let rod =
                        serde_json::from_value(attr_value).map_err(serde::de::Error::custom)?;
                    (ItemAttributesType::Rod, ItemAttributes::Rod(rod))
                }
                _ => continue,
            };

            container.components.insert(attr_type, attribute);
        }

        Ok(container)
    }
}
