use crate::utils::math::float_interpolate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct FishData {
    pub name: String,
    pub min_size_baby_mm: u32,
    pub max_size_baby_mm: u32,
    pub min_size_adult_mm: u32,
    pub max_size_adult_mm: u32,
    pub min_weight_baby_g: u32,
    pub max_weight_baby_g: u32,
    pub min_weight_adult_g: u32,
    pub max_weight_adult_g: u32,
    pub min_lifespan_days: u32,
    pub max_lifespan_days: u32,
    /// Time when fish becomes adult (0 to 1)
    #[serde(default = "default_lifespan_adult_ratio")]
    pub lifespan_adult_ratio: f32,
}

fn default_lifespan_adult_ratio() -> f32 {
    0.35
}

impl FishData {
    pub fn get_baby_size_by_ratio(&self, ratio: f32) -> f32 {
        float_interpolate(
            self.min_size_baby_mm as f32,
            self.max_size_baby_mm as f32,
            ratio,
        )
    }

    pub fn get_adult_size_by_ratio(&self, ratio: f32) -> f32 {
        float_interpolate(
            self.min_size_adult_mm as f32,
            self.max_size_adult_mm as f32,
            ratio,
        )
    }

    pub fn get_baby_weight_by_ratio(&self, ratio: f32) -> f32 {
        float_interpolate(
            self.min_weight_baby_g as f32,
            self.max_weight_baby_g as f32,
            ratio,
        )
    }

    pub fn get_adult_weight_by_ratio(&self, ratio: f32) -> f32 {
        float_interpolate(
            self.min_weight_adult_g as f32,
            self.max_weight_adult_g as f32,
            ratio,
        )
    }

    pub fn get_lifespan_days_by_ratio(&self, ratio: f32) -> f32 {
        float_interpolate(
            self.min_lifespan_days as f32,
            self.max_lifespan_days as f32,
            ratio,
        )
    }
}
