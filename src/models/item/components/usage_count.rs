use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsageComponent {
    times_used: u64,
}

impl UsageComponent {
    pub fn new(times_used: u64) -> Self {
        Self { times_used }
    }

    pub fn get_times_used(&self) -> u64 {
        self.times_used
    }

    // Events
    pub fn on_use(&mut self) {
        self.times_used = self.times_used.saturating_add(1);
    }
}
