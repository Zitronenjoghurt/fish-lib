use crate::models::item::attributes::bait::BaitAttributes;
use crate::models::item::attributes::purchasable::PurchasableAttributes;
use crate::models::item::attributes::rod::RodAttributes;
use serde::{Deserialize, Serialize};

pub mod bait;
pub mod purchasable;
pub mod rod;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemAttributesType {
    Bait,
    Purchasable,
    Rod,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemAttributes {
    Bait(BaitAttributes),
    Purchasable(PurchasableAttributes),
    Rod(RodAttributes),
}

impl ItemAttributes {
    pub fn bait(level: u64) -> Self {
        Self::Bait(BaitAttributes::new(level))
    }

    pub fn purchasable(cost: u32) -> Self {
        Self::Purchasable(PurchasableAttributes::new(cost))
    }

    pub fn rod(level: u64) -> Self {
        Self::Rod(RodAttributes::new(level))
    }
}
