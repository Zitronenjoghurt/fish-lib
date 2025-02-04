use crate::config::ConfigInterface;
use crate::game::services::location_service::LocationServiceInterface;
use crate::game::systems::weather_system::config::WeatherSystemConfig;
use crate::game::systems::weather_system::weather::Weather;
use crate::game::systems::weather_system::WeatherSystem;
use chrono::DateTime;
use chrono_tz::Tz;
use std::collections::HashMap;
use std::sync::Arc;

pub trait WeatherServiceInterface: Send + Sync {
    fn get_weather(&self, location_id: i32, time: DateTime<Tz>) -> Option<Weather>;
    fn get_current_weather(&self, location_id: i32) -> Option<Weather>;
}

pub struct WeatherService {
    time_multiplier: f32,
    // Weather systems by location id
    weather_systems: HashMap<i32, WeatherSystem>,
    location_service: Arc<dyn LocationServiceInterface>,
}

impl WeatherService {
    pub fn new(
        config: Arc<dyn ConfigInterface>,
        location_service: Arc<dyn LocationServiceInterface>,
    ) -> Self {
        let mut systems = HashMap::new();
        config
            .locations()
            .iter()
            .for_each(|(location_id, location_data)| {
                let config = WeatherSystemConfig::builder()
                    .with_location_data(location_data.clone())
                    .build();
                let system = WeatherSystem::new(config);
                systems.insert(*location_id, system);
            });

        Self {
            time_multiplier: config.settings().time_speed_multiplier,
            weather_systems: systems,
            location_service,
        }
    }
}

impl WeatherServiceInterface for WeatherService {
    fn get_weather(&self, location_id: i32, time: DateTime<Tz>) -> Option<Weather> {
        self.weather_systems
            .get(&location_id)
            .map(|system| system.get_weather(time, self.time_multiplier))
    }

    fn get_current_weather(&self, location_id: i32) -> Option<Weather> {
        let location_data = self.location_service.get_location_data(location_id)?;
        let time_now = location_data.get_local_time();
        self.get_weather(location_id, time_now)
    }
}
