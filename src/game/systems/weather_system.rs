use crate::game::systems::weather_system::attributes::WeatherAttributes;
use crate::game::systems::weather_system::config::WeatherSystemConfig;
use crate::game::systems::weather_system::weather::Weather;
use crate::utils::math::float_interpolate;
use chrono::{DateTime, Timelike, Utc};
use noise::{NoiseFn, Perlin};

pub mod attributes;
pub mod config;
pub mod weather;

pub struct WeatherSystem {
    cloudiness: Perlin,
    cloud_brightness: Perlin,
    moisture: Perlin,
    wind_presence: Perlin,
    wind_strength: Perlin,
    temperature: Perlin,
    rain_intensity: Perlin,
    config: WeatherSystemConfig,
}

impl WeatherSystem {
    pub fn new(config: WeatherSystemConfig) -> Self {
        let seed = config.location_data.weather_seed;
        Self {
            cloudiness: Perlin::new(seed),
            cloud_brightness: Perlin::new(seed * 2),
            moisture: Perlin::new(seed * 3),
            wind_presence: Perlin::new(seed * 4),
            wind_strength: Perlin::new(seed * 5),
            temperature: Perlin::new(seed * 6),
            rain_intensity: Perlin::new(seed * 7),
            config,
        }
    }

    fn time_to_noise_input(time: DateTime<Utc>) -> f64 {
        time.timestamp() as f64 / 1_000_000.0
    }

    fn normalize_noise(noise: f64) -> f32 {
        (noise as f32 + 1.0) / 2.0
    }

    /// Ensures its hottest at the middle of the day
    pub fn light_level(time: DateTime<Utc>) -> f32 {
        let hour = time.hour() as f32 + (time.minute() as f32 / 60.0);
        let multiplier = (((hour - 6.0) * std::f32::consts::PI / 12.0).sin() * 0.45) + 0.55;
        multiplier.clamp(0.1, 1.0)
    }

    pub fn get_weather_attributes(&self, time: DateTime<Utc>) -> WeatherAttributes {
        let t = Self::time_to_noise_input(time);

        let cloudiness_noise = self.cloudiness.get([t * 5.5, 0.0]);
        let cloud_brightness_noise = self.cloud_brightness.get([t * 2.5, 1.0]);
        let moisture_noise = self.moisture.get([t * 3.25, 1_000_000.0]);
        let wind_presence_noise = self.wind_presence.get([t * 40.0, t * 50.0]);
        let wind_strength_noise = self.wind_strength.get([t * 4.5, 5_000_000.0]);
        let temperature_noise = self.temperature.get([t, 2_000_000.0]);
        let rain_intensity_noise = self.rain_intensity.get([t, 0.0]);

        let moisture = Self::normalize_noise(moisture_noise);
        let rain_intensity = Self::normalize_noise(rain_intensity_noise);

        let wind_strength = Self::normalize_noise(wind_strength_noise);
        let wind_presence = Self::normalize_noise(wind_presence_noise);

        let cloudiness = Self::normalize_noise(cloudiness_noise);
        let cloud_brightness_raw = Self::normalize_noise(cloud_brightness_noise);

        // Ensure that lower cloud brightness only occurs with high cloudiness
        let cloud_brightness =
            (cloud_brightness_raw * (1.0 - cloudiness) + 1.0 * (1.0 - cloudiness)).clamp(0.0, 1.0);

        let raw_light = Self::light_level(time);

        let cloud_light_blocking =
            cloudiness * (1.0 - cloud_brightness * self.config.cloud_brightness_light_block_factor);
        let light = raw_light * (1.0 - cloud_light_blocking);
        let temperature = Self::normalize_noise(temperature_noise) * raw_light;

        WeatherAttributes {
            cloudiness,
            cloud_brightness,
            moisture,
            wind_presence,
            wind_strength,
            temperature,
            light,
            rain_intensity,
        }
    }

    pub fn get_weather(&self, time: DateTime<Utc>, time_multiplier: f32) -> Weather {
        let attributes = self.get_weather_attributes(time);
        let (season_data, season, season_progress) = self
            .config
            .location_data
            .full_season_information(time, time_multiplier);

        let raining_rain_intensity_met =
            attributes.rain_intensity > season_data.rain_intensity_raining_threshold;
        let raining_moisture_met = attributes.moisture > season_data.moisture_raining_threshold;
        let raining_cloudiness_met =
            attributes.cloudiness > season_data.cloudiness_raining_threshold;
        let is_raining =
            raining_cloudiness_met && raining_moisture_met && raining_rain_intensity_met;
        let rain_strength = if is_raining {
            (attributes.rain_intensity - season_data.rain_intensity_raining_threshold)
                / (1.0 - season_data.rain_intensity_raining_threshold)
        } else {
            0.0
        };

        Weather {
            location_name: self.config.location_data.name.clone(),
            season,
            season_progress,
            temperature_c: float_interpolate(
                season_data.min_temp_c,
                season_data.max_temp_c,
                attributes.temperature,
            ),
            min_possible_temp_c: season_data.min_temp_c,
            max_possible_temp_c: season_data.max_temp_c,
            humidity: attributes.moisture,
            light_level: attributes.light,
            cloudiness: attributes.cloudiness,
            cloud_brightness: attributes.cloud_brightness,
            is_raining,
            rain_strength,
        }
    }
}
