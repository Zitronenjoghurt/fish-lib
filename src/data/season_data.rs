use crate::utils::math::float_interpolate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct SeasonData {
    pub min_temp_c: f32,
    pub max_temp_c: f32,
}

impl SeasonData {
    pub fn interpolate(&self, next_data: &SeasonData, progress: f32) -> Self {
        Self {
            min_temp_c: float_interpolate(self.min_temp_c, next_data.min_temp_c, progress),
            max_temp_c: float_interpolate(self.max_temp_c, next_data.max_temp_c, progress),
        }
    }
}
