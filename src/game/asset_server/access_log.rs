use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::time::Instant;

#[derive(Debug, Default)]
pub struct AccessLog {
    entries: PriorityQueue<String, Reverse<Instant>>,
}

impl AccessLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_entry(&mut self, entry: String) {
        self.entries.push(entry, Reverse(Instant::now()));
    }

    pub fn has_entry(&self, entry: &str) -> bool {
        self.entries.get(entry).is_some()
    }

    pub fn pop_oldest_entry(&mut self) -> Option<String> {
        self.entries.pop().map(|(key, _)| key.clone())
    }
}
