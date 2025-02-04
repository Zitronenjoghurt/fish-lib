use crate::config::ConfigInterface;
use crate::data::location_data::LocationData;
use crate::game::errors::resource::GameResourceError;
use crate::game::errors::GameResult;
use crate::game::services::weather_service::WeatherServiceInterface;
use crate::game::systems::encounter_system::{EncounterSystem, EncounterWeather};
use crate::game::systems::weather_system::weather::Weather;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use std::sync::Arc;

pub trait EncounterServiceInterface: Send + Sync {
    fn roll_encounter_current(
        &self,
        location_id: i32,
        location_data: Arc<LocationData>,
    ) -> GameResult<i32>;

    fn roll_encounter(
        &self,
        time: DateTime<Tz>,
        weather: Weather,
        location_id: i32,
    ) -> GameResult<i32>;
}

pub struct EncounterService {
    system: EncounterSystem,
    weather_service: Arc<dyn WeatherServiceInterface>,
}

impl EncounterService {
    pub fn new(
        config: Arc<dyn ConfigInterface>,
        weather_service: Arc<dyn WeatherServiceInterface>,
    ) -> Self {
        let system = EncounterSystem::new(config.species(), config.settings().rarity_exponent);

        Self {
            system,
            weather_service,
        }
    }
}

impl EncounterServiceInterface for EncounterService {
    fn roll_encounter_current(
        &self,
        location_id: i32,
        location_data: Arc<LocationData>,
    ) -> GameResult<i32> {
        let location_time = Utc::now().with_timezone(&location_data.timezone);
        let location_weather = self
            .weather_service
            .get_weather(location_id, location_time)?;
        self.roll_encounter(location_time, location_weather, location_id)
    }

    fn roll_encounter(
        &self,
        time: DateTime<Tz>,
        weather: Weather,
        location_id: i32,
    ) -> GameResult<i32> {
        let encounter_weather = if weather.is_raining {
            EncounterWeather::Rain
        } else {
            EncounterWeather::Any
        };

        self.system
            .roll_encounter(time, encounter_weather, location_id)
            .ok_or_else(|| GameResourceError::no_available_encounters().into())
    }
}
