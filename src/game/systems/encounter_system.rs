use crate::get_config;
use chrono::{DateTime, Timelike};
use chrono_tz::Tz;
use rand::seq::SliceRandom;
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
    cached_weights: HashMap<RarityLevel, u64>,
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

        let rarity_exponent = get_config().settings.rarity_exponent;
        let cached_weights = (0..=255)
            .map(|level| (level, Self::rarity_level_weight(level, rarity_exponent)))
            .collect();

        Self {
            encounters,
            cached_weights,
        }
    }

    fn rarity_level_weight(rarity_level: RarityLevel, rarity_exponent: f64) -> u64 {
        ((255 - rarity_level) as f64).powf(rarity_exponent) as u64 + 1
    }

    fn roll_rarity_level(&self, available_rarities: &[RarityLevel]) -> Option<RarityLevel> {
        if available_rarities.is_empty() {
            return None;
        }

        let cumulative_weights: Vec<u64> = available_rarities
            .iter()
            .scan(0u64, |sum, &rarity| {
                *sum += self.cached_weights[&rarity];
                Some(*sum)
            })
            .collect();

        let total = cumulative_weights.last()?;

        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(0..*total);

        let index = cumulative_weights.partition_point(|&weight| weight <= roll);
        Some(available_rarities[index])
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

    pub fn roll_encounter(
        &self,
        time: DateTime<Tz>,
        weather: EncounterWeather,
        location_id: LocationId,
    ) -> Option<SpeciesId> {
        let possible_rarity_encounters =
            self.get_possible_rarity_encounters(time, weather, location_id)?;

        let valid_rarity_levels: Vec<RarityLevel> =
            possible_rarity_encounters.keys().copied().collect();
        let rarity = self.roll_rarity_level(&valid_rarity_levels)?;

        let mut rng = rand::thread_rng();
        let possible_species = possible_rarity_encounters.get(&rarity)?;
        possible_species.choose(&mut rng).copied()
    }
}
