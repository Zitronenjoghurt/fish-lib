use crate::config::ConfigInterface;
use crate::enums::item_category::ItemCategory;
use crate::models::item::Item;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Inventory {
    config: Arc<dyn ConfigInterface>,
    items: Vec<Item>,
}

impl Inventory {
    pub fn new(config: Arc<dyn ConfigInterface>, items: Vec<Item>) -> Self {
        Self { config, items }
    }

    pub fn get_items(&self) -> &[Item] {
        &self.items
    }

    pub fn get_items_by_category(&self, category: ItemCategory) -> Vec<&Item> {
        self.items
            .iter()
            .filter(|item| self.config.is_item_id_in_category(item.type_id, category))
            .collect()
    }
}

impl PartialEq for Inventory {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}
