use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    /// How much faster game speed is compared to real-time
    #[serde(default = "default_time_speed_multiplier")]
    pub time_speed_multiplier: f32,
    /// How much rarer the rarest fish will be
    /// The rarest fish will be 1 in 255^(rarity_exponent)
    /// 255^(rarity_exponent) shouldn't exceed 1.7976931348623157e+308
    #[serde(default = "default_rarity_exponent")]
    pub rarity_exponent: f64,
}

fn default_time_speed_multiplier() -> f32 {
    1.0
}

fn default_rarity_exponent() -> f64 {
    2.5
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            time_speed_multiplier: default_time_speed_multiplier(),
            rarity_exponent: default_rarity_exponent(),
        }
    }
}
