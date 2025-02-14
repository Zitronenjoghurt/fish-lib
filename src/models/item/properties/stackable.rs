use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct StackableComponent {
    #[serde(default = "default_one")]
    count: u64,
}

fn default_one() -> u64 {
    1
}

impl StackableComponent {
    pub fn new(count: u64) -> Self {
        Self { count }
    }

    pub fn get_count(&self) -> u64 {
        self.count
    }

    pub fn set_count(&mut self, count: u64) {
        self.count = count;
    }

    // Events
    pub fn on_use(&mut self, times: u64) {
        self.count = self.count.saturating_sub(times);
    }

    pub fn on_add(&mut self, amount: u64) {
        self.count = self.count.saturating_add(amount);
    }

    pub fn on_remove(&mut self, amount: u64) {
        self.count = self.count.saturating_sub(amount);
    }

    pub fn should_consume(&self) -> bool {
        self.count == 0
    }
}
