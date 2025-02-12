use crate::models::item::properties::stackable::StackableComponent;
use crate::models::item::properties::usage_count::UsageComponent;
use serde::{Deserialize, Serialize};

pub mod stackable;
pub mod usage_count;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemPropertiesType {
    Stackable,
    Usage,
}

impl ItemPropertiesType {
    pub fn get_default_properties(&self) -> ItemProperties {
        match self {
            Self::Stackable => ItemProperties::Stackable(StackableComponent::default()),
            Self::Usage => ItemProperties::Usage(UsageComponent::default()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemProperties {
    Stackable(StackableComponent),
    Usage(UsageComponent),
}

impl ItemProperties {
    pub fn stackable(count: u64) -> Self {
        Self::Stackable(StackableComponent::new(count))
    }

    pub fn usage(count: u64) -> Self {
        Self::Usage(UsageComponent::new(count))
    }

    // Events
    pub fn on_use(&mut self, times: u64) {
        match self {
            Self::Stackable(stackable) => stackable.on_use(times),
            Self::Usage(usage) => usage.on_use(times),
        }
    }

    pub fn on_add(&mut self, amount: u64) {
        if let Self::Stackable(stackable) = self {
            stackable.on_add(amount);
        }
    }

    pub fn on_remove(&mut self, amount: u64) {
        if let Self::Stackable(stackable) = self {
            stackable.on_remove(amount);
        }
    }

    pub fn should_consume(&self) -> bool {
        match self {
            Self::Stackable(stackable) => stackable.should_consume(),
            _ => false,
        }
    }
}
