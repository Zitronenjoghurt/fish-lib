use crate::get_config;
use chrono::{DateTime, Timelike};
use chrono_tz::Tz;
use rand::Rng;
use std::collections::HashMap;

pub type SpeciesId = i32;
pub type LocationId = i32;
pub type RarityLevel = u8;

pub type RarityEncounters = HashMap<RarityLevel, Vec<SpeciesId>>;
pub type LocationEncounters = HashMap<LocationId, RarityEncounters>;
pub type WeatherEncounters = HashMap<EncounterWeather, LocationEncounters>;
pub type HourlyEncounters = HashMap<u8, WeatherEncounters>;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum EncounterWeather {
    Any,
    Rain,
}

pub struct EncounterSystem {
    /// Hour -> Weather -> Location ID -> (Species ID, Rarity Level)
    encounters: HourlyEncounters,
}

impl EncounterSystem {
    pub fn new() -> Self {
        let mut encounters: HourlyEncounters = HashMap::new();

        let config = get_config();
        for (species_id, species_data) in &config.fish {
            for encounter in &species_data.encounters {
                let weather = if encounter.needs_rain {
                    EncounterWeather::Rain
                } else {
                    EncounterWeather::Any
                };

                for hour in encounter.get_hours() {
                    encounters
                        .entry(hour)
                        .or_default()
                        .entry(weather)
                        .or_default()
                        .entry(encounter.location_id)
                        .or_default()
                        .entry(encounter.rarity_level)
                        .or_default()
                        .push(*species_id);
                }
            }
        }

        Self { encounters }
    }

    fn roll_rarity_level() -> u8 {
        let mut rng = rand::thread_rng();
        let rarity_exponent = &get_config().settings.rarity_exponent;

        let max_weight = 255_f64.powf(*rarity_exponent) as u64;
        let roll = rng.gen_range(0..max_weight);

        255 - (roll as f64).powf(1.0 / *rarity_exponent) as u8
    }

    fn get_possible_rarity_encounters(
        &self,
        time: DateTime<Tz>,
        weather: EncounterWeather,
        location_id: i32,
    ) -> Option<&RarityEncounters> {
        self.encounters
            .get(&(time.hour() as u8))?
            .get(&weather)?
            .get(&location_id)
    }
}
