use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    /// How much faster game speed is compared to real-time
    #[serde(default = "default_time_speed_multiplier")]
    pub time_speed_multiplier: f32,
    #[serde(default)]
    pub weather_seed: u32,
}

fn default_time_speed_multiplier() -> f32 {
    1.0
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            time_speed_multiplier: default_time_speed_multiplier(),
            weather_seed: Default::default(),
        }
    }
}
