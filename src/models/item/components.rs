use crate::models::item::components::usage_count::UsageComponent;
use serde::{Deserialize, Serialize};

pub mod usage_count;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemComponentType {
    Usage,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemComponent {
    Usage(UsageComponent),
}

impl ItemComponent {
    pub fn usage(count: u64) -> Self {
        Self::Usage(UsageComponent::new(count))
    }
}
