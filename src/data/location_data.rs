use crate::data::season_data::SeasonData;
use crate::enums::season::Season;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

const SECONDS_PER_YEAR: f64 = 31_556_925.190_8;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct LocationData {
    pub name: String,
    pub timezone: Tz,
    pub weather_seed: u32,
    pub spring: SeasonData,
    pub summer: SeasonData,
    pub autumn: SeasonData,
    pub winter: SeasonData,
}

impl LocationData {
    fn season_index_and_progress(&self, time: DateTime<Tz>, time_multiplier: f32) -> (usize, f64) {
        let year_progress = (time.timestamp() as f64 * time_multiplier as f64) % SECONDS_PER_YEAR
            / SECONDS_PER_YEAR;

        let current_season = year_progress * 4.0;
        let season_index = current_season.floor() as usize;
        let season_progress = current_season % 1.0;

        (season_index, season_progress)
    }

    pub fn current_season_data(&self, time: DateTime<Tz>, time_multiplier: f32) -> SeasonData {
        let (index, progress) = self.season_index_and_progress(time, time_multiplier);

        let (prev_data, current_data, next_data) = match index {
            0 => (&self.winter, &self.spring, &self.summer),
            1 => (&self.spring, &self.summer, &self.autumn),
            2 => (&self.summer, &self.autumn, &self.winter),
            3 => (&self.autumn, &self.winter, &self.spring),
            _ => unreachable!(),
        };

        if progress < 0.5 {
            let adjusted_progress = progress * 2.0;
            let start = prev_data.interpolate(current_data, 0.5);
            start.interpolate(current_data, adjusted_progress as f32)
        } else {
            let adjusted_progress = (progress - 0.5) * 2.0;
            let start = current_data.interpolate(next_data, 0.5);
            current_data.interpolate(&start, adjusted_progress as f32)
        }
    }

    pub fn current_season(&self, time: DateTime<Tz>, time_multiplier: f32) -> (Season, f64) {
        let (index, progress) = self.season_index_and_progress(time, time_multiplier);
        let season = Season::from_index(index);
        (season, progress)
    }

    pub fn full_season_information(
        &self,
        time: DateTime<Tz>,
        time_multiplier: f32,
    ) -> (SeasonData, Season, f64) {
        let data = self.current_season_data(time, time_multiplier);
        let (season, progress) = self.current_season(time, time_multiplier);
        (data, season, progress)
    }

    pub fn get_local_time(&self) -> DateTime<Tz> {
        Utc::now().with_timezone(&self.timezone)
    }
}
