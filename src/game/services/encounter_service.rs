use crate::config::ConfigInterface;
use crate::game::errors::resource::GameResourceError;
use crate::game::errors::GameResult;
use crate::game::systems::encounter_system::{EncounterSystem, EncounterWeather};
use crate::game::systems::weather_system::weather::Weather;
use chrono::DateTime;
use chrono_tz::Tz;
use std::sync::Arc;

pub trait EncounterServiceInterface: Send + Sync {
    fn roll_encounter(
        &self,
        time: DateTime<Tz>,
        weather: Weather,
        location_id: i32,
    ) -> GameResult<i32>;
}

pub struct EncounterService {
    system: EncounterSystem,
}

impl EncounterService {
    pub fn new(config: Arc<dyn ConfigInterface>) -> Self {
        let system = EncounterSystem::new(config.species(), config.settings().rarity_exponent);

        Self { system }
    }
}

impl EncounterServiceInterface for EncounterService {
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
