use crate::models::item::Item;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    pub fn new(items: Vec<Item>) -> Self {
        Self { items }
    }

    pub fn get_items(&self) -> &[Item] {
        &self.items
    }
}
