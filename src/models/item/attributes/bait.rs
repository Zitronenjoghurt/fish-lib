use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaitAttributes {
    level: u64,
}

impl BaitAttributes {
    pub fn new(level: u64) -> Self {
        Self { level }
    }

    pub fn get_level(&self) -> u64 {
        self.level
    }
}
