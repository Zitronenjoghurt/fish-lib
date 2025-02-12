use crate::models::item::components::stackable::StackableComponent;
use crate::models::item::components::usage_count::UsageComponent;
use serde::{Deserialize, Serialize};

pub mod stackable;
pub mod usage_count;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemComponentType {
    Stackable,
    Usage,
}

impl ItemComponentType {
    pub fn get_default_component(&self) -> ItemComponent {
        match self {
            Self::Stackable => ItemComponent::Stackable(StackableComponent::default()),
            Self::Usage => ItemComponent::Usage(UsageComponent::default()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemComponent {
    Stackable(StackableComponent),
    Usage(UsageComponent),
}

impl ItemComponent {
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
