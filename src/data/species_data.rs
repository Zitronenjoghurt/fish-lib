use crate::data::encounter_data::EncounterData;
use crate::utils::math::float_interpolate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct SpeciesData {
    #[serde(skip, default)]
    pub id: i32,
    pub name: String,
    /// Minimum possible size at age 0.0
    pub min_size_baby_mm: u32,
    /// Maximum possible size at age 0.0
    pub max_size_baby_mm: u32,
    /// Minimum possible size at age 1.0
    pub min_size_adult_mm: u32,
    /// Maximum possible size at age 1.0
    pub max_size_adult_mm: u32,
    /// Minimum possible weight at age 0.0
    pub min_weight_baby_g: u32,
    /// Maximum possible weight at age 0.0
    pub max_weight_baby_g: u32,
    /// Minimum possible weight at age 1.0
    pub min_weight_adult_g: u32,
    /// Maximum possible weight at age 1.0
    pub max_weight_adult_g: u32,
    /// Minimum possible lifespan in days
    pub min_lifespan_days: u32,
    /// Maximum possible lifespan in days
    pub max_lifespan_days: u32,
    /// Time when fish becomes adult (0 to 1)
    #[serde(default = "default_lifespan_adult_ratio")]
    pub lifespan_adult_ratio: f32,
    #[serde(default)]
    pub encounters: Vec<EncounterData>,
}

fn default_lifespan_adult_ratio() -> f32 {
    0.35
}

impl SpeciesData {
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
