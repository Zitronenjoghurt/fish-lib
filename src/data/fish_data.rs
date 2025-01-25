use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct FishData {
    pub name: String,
    pub min_size_baby_mm: u32,
    pub max_size_baby_mm: u32,
    pub min_size_adult_mm: u32,
    pub max_size_adult_mm: u32,
    pub min_lifespan_days: u32,
    pub max_lifespan_days: u32,
    /// Time when fish becomes adult (0 to 1)
    #[serde(default = "default_lifespan_adult_ratio")]
    pub lifespan_adult_ratio: f32,
}

fn default_lifespan_adult_ratio() -> f32 {
    0.35
}
