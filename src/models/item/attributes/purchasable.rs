use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchasableAttributes {
    cost: u32,
}

impl PurchasableAttributes {
    pub fn new(cost: u32) -> Self {
        Self { cost }
    }

    pub fn get_cost(&self) -> u32 {
        self.cost
    }
}
