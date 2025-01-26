use crate::game::systems::weather_system::{WeatherAttributes, WeatherSystem};
use crate::get_config;
use chrono::Utc;
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    static ref WEATHER_SERVICE: Arc<WeatherService> = Arc::new(WeatherService::new());
}

pub struct WeatherService {
    weather_system: WeatherSystem,
}

impl WeatherService {
    fn new() -> Self {
        let seed = get_config().settings.weather_seed;
        Self {
            weather_system: WeatherSystem::new(seed),
        }
    }

    pub fn get_instance() -> Arc<WeatherService> {
        WEATHER_SERVICE.clone()
    }

    pub fn get_current_weather(&self) -> WeatherAttributes {
        let time_now = Utc::now();
        self.weather_system.get_weather_attributes(time_now)
    }
}
