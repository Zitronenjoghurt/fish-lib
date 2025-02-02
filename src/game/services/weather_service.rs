use crate::game::services::location_service::LocationService;
use crate::game::systems::weather_system::config::WeatherSystemConfig;
use crate::game::systems::weather_system::weather::Weather;
use crate::game::systems::weather_system::WeatherSystem;
use crate::get_config;
use chrono::DateTime;
use chrono_tz::Tz;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Arc;

lazy_static! {
    static ref WEATHER_SERVICE: Arc<WeatherService> = Arc::new(WeatherService::new());
}

pub struct WeatherService {
    // Weather systems by location id
    weather_systems: HashMap<i32, WeatherSystem>,
}

impl WeatherService {
    fn new() -> Self {
        let mut systems = HashMap::new();
        get_config()
            .locations
            .iter()
            .for_each(|(location_id, location_data)| {
                let config = WeatherSystemConfig::builder()
                    .with_location_data(location_data.clone())
                    .build();
                let system = WeatherSystem::new(config);
                systems.insert(*location_id, system);
            });

        Self {
            weather_systems: systems,
        }
    }

    pub fn get_instance() -> Arc<WeatherService> {
        WEATHER_SERVICE.clone()
    }

    pub fn get_weather(&self, location_id: i32, time: DateTime<Tz>) -> Option<Weather> {
        let time_multiplier = get_config().settings.time_speed_multiplier;
        self.weather_systems
            .get(&location_id)
            .map(|system| system.get_weather(time, time_multiplier))
    }

    pub fn get_current_weather(&self, location_id: i32) -> Option<Weather> {
        let location_data = LocationService::get_location_data(location_id)?;
        let time_now = location_data.get_local_time();
        self.get_weather(location_id, time_now)
    }
}
