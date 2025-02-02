use crate::enums::season::Season;
use chrono::DateTime;
use chrono_tz::Tz;

#[derive(Debug)]
pub struct Weather {
    pub location_name: String,
    pub time: DateTime<Tz>,
    pub season: Season,
    pub season_progress: f64,
    pub temperature_c: f32,
    pub min_possible_temp_c: f32,
    pub max_possible_temp_c: f32,
    pub humidity: f32,
    pub light_level: f32,
    pub cloudiness: f32,
    pub cloud_brightness: f32,
    pub is_raining: bool,
    pub rain_strength: f32,
}
