use crate::data::location_data::LocationData;
use std::sync::Arc;

#[derive(Debug)]
pub struct WeatherSystemConfig {
    pub cloud_brightness_light_block_factor: f32,
    pub location_data: Arc<LocationData>,
}

impl Default for WeatherSystemConfig {
    fn default() -> Self {
        Self {
            cloud_brightness_light_block_factor: 0.7,
            location_data: Arc::new(LocationData::default()),
        }
    }
}

impl WeatherSystemConfig {
    pub fn builder() -> WeatherSystemConfigBuilder {
        WeatherSystemConfigBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct WeatherSystemConfigBuilder {
    config: WeatherSystemConfig,
}

impl WeatherSystemConfigBuilder {
    pub fn with_cloud_brightness_light_block_factor(mut self, value: f32) -> Self {
        self.config.cloud_brightness_light_block_factor = value;
        self
    }

    pub fn with_location_data(mut self, value: Arc<LocationData>) -> Self {
        self.config.location_data = value;
        self
    }

    pub fn build(self) -> WeatherSystemConfig {
        self.config
    }
}
