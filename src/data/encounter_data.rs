use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct EncounterData {
    pub location_id: i32,
    /// Minimum and maximum local time (24h-format) this fish can be encountered at
    /// 16-16 means a fish will only appear from 4:00pm to 4:59pm
    /// 16-17 means a fish will only appear from 4:00pm to 5:59pm
    pub min_time_hour: u8,
    pub max_time_hour: u8,
    /// The higher, the rarer
    pub rarity_level: u8,
    #[serde(default = "default_false")]
    pub needs_rain: bool,
}

fn default_false() -> bool {
    false
}

impl EncounterData {
    pub fn get_hours(&self) -> Vec<u8> {
        (self.min_time_hour..=self.max_time_hour).collect()
    }
}
