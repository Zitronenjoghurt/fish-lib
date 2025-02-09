#[derive(Debug, Default, Clone, PartialEq)]
pub struct LocationUnlockRequirements {
    pub locations_unlocked: Vec<i32>,
    pub species_caught: Vec<i32>,
}

impl LocationUnlockRequirements {
    pub fn is_empty(&self) -> bool {
        self.locations_unlocked.is_empty() && self.species_caught.is_empty()
    }
}
