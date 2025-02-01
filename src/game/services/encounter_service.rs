use crate::data::location_data::LocationData;
use crate::game::services::weather_service::WeatherService;
use crate::game::systems::encounter_system::{EncounterSystem, EncounterWeather};
use crate::game::systems::weather_system::weather::Weather;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    static ref ENCOUNTER_SERVICE: Arc<EncounterService> = Arc::new(EncounterService::new());
}

pub struct EncounterService {
    system: EncounterSystem,
}

impl EncounterService {
    pub fn new() -> Self {
        Self {
            system: EncounterSystem::new(),
        }
    }

    pub fn get_instance() -> Arc<EncounterService> {
        ENCOUNTER_SERVICE.clone()
    }

    pub fn roll_encounter_current(
        &self,
        location_id: i32,
        location_data: Arc<LocationData>,
    ) -> Option<i32> {
        let location_time = Utc::now().with_timezone(&location_data.timezone);
        let location_weather =
            WeatherService::get_instance().get_weather(location_id, location_time)?;
        self.roll_encounter(location_time, location_weather, location_id)
    }

    pub fn roll_encounter(
        &self,
        time: DateTime<Tz>,
        weather: Weather,
        location_id: i32,
    ) -> Option<i32> {
        let encounter_weather = if weather.is_raining {
            EncounterWeather::Rain
        } else {
            EncounterWeather::Any
        };

        self.system
            .roll_encounter(time, encounter_weather, location_id)
    }
}
