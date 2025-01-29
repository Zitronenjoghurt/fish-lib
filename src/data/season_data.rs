use crate::utils::math::float_interpolate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct SeasonData {
    pub min_temp_c: f32,
    pub max_temp_c: f32,
    /// How big the rain intensity has to be for rain to start
    #[serde(default = "default_rain_intensity_raining_threshold")]
    pub rain_intensity_raining_threshold: f32,
    /// How big the humidity has to be for rain to start
    #[serde(default = "default_moisture_raining_threshold")]
    pub moisture_raining_threshold: f32,
    /// How big the cloudiness has to be for rain to start
    #[serde(default = "default_cloudiness_raining_threshold")]
    pub cloudiness_raining_threshold: f32,
}

fn default_rain_intensity_raining_threshold() -> f32 {
    0.4
}

fn default_moisture_raining_threshold() -> f32 {
    0.75
}

fn default_cloudiness_raining_threshold() -> f32 {
    0.6
}

impl SeasonData {
    pub fn interpolate(&self, next_data: &SeasonData, progress: f32) -> Self {
        Self {
            min_temp_c: float_interpolate(self.min_temp_c, next_data.min_temp_c, progress),
            max_temp_c: float_interpolate(self.max_temp_c, next_data.max_temp_c, progress),
            rain_intensity_raining_threshold: float_interpolate(
                self.rain_intensity_raining_threshold,
                next_data.rain_intensity_raining_threshold,
                progress,
            ),
            moisture_raining_threshold: float_interpolate(
                self.moisture_raining_threshold,
                next_data.moisture_raining_threshold,
                progress,
            ),
            cloudiness_raining_threshold: float_interpolate(
                self.cloudiness_raining_threshold,
                next_data.cloudiness_raining_threshold,
                progress,
            ),
        }
    }
}
