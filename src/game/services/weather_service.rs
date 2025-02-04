use crate::config::ConfigInterface;
use crate::game::errors::resource::GameResourceError;
use crate::game::errors::GameResult;
use crate::game::services::location_service::LocationServiceInterface;
use crate::game::systems::weather_system::config::WeatherSystemConfig;
use crate::game::systems::weather_system::weather::Weather;
use crate::game::systems::weather_system::WeatherSystem;
use chrono::DateTime;
use chrono_tz::Tz;
use std::collections::HashMap;
use std::sync::Arc;

pub trait WeatherServiceInterface: Send + Sync {
    fn get_weather(&self, location_id: i32, time: DateTime<Tz>) -> GameResult<Weather>;
    fn get_current_weather(&self, location_id: i32) -> GameResult<Weather>;
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
    fn get_weather(&self, location_id: i32, time: DateTime<Tz>) -> GameResult<Weather> {
        self.weather_systems
            .get(&location_id)
            .map(|system| system.get_weather(time, self.time_multiplier))
            .ok_or_else(|| GameResourceError::location_not_found(location_id).into())
    }

    fn get_current_weather(&self, location_id: i32) -> GameResult<Weather> {
        if let Some(location_data) = self.location_service.get_location_data(location_id) {
            let time_now = location_data.get_local_time();
            self.get_weather(location_id, time_now)
        } else {
            Err(GameResourceError::location_not_found(location_id).into())
        }
    }
}
