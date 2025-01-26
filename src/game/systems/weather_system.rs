use chrono::{DateTime, Timelike, Utc};
use noise::{NoiseFn, Perlin};

pub enum Weather {
    Sunny,
    Clear,
    Cloudy,
    Rainy,
    Stormy,
}

#[derive(Default, Debug)]
pub struct WeatherAttributes {
    pub cloudiness: f32,
    pub moisture: f32,
    pub wind: f32,
    pub temperature: f32,
}

pub struct WeatherSystem {
    cloudiness: Perlin,
    moisture: Perlin,
    wind: Perlin,
    temperature: Perlin,
}

impl WeatherSystem {
    pub fn new(seed: u32) -> Self {
        Self {
            cloudiness: Perlin::new(seed),
            moisture: Perlin::new(seed + 1_000_000),
            wind: Perlin::new(seed + 2_000_000),
            temperature: Perlin::new(seed + 3_000_000),
        }
    }

    fn time_to_noise_input(time: DateTime<Utc>) -> f64 {
        time.timestamp() as f64 / 1_000_000.0
    }

    fn normalize_noise(noise: f64) -> f32 {
        (noise as f32 + 1.0) / 2.0
    }

    /// Ensures its hottest at the middle of the day
    pub fn temperature_multiplier(time: DateTime<Utc>) -> f32 {
        let hour = time.hour() as f32 + (time.minute() as f32 / 60.0);
        let multiplier = (((hour - 6.0) * std::f32::consts::PI / 12.0).sin() * 0.45) + 0.55;
        multiplier.clamp(0.1, 1.0)
    }

    pub fn get_weather_attributes(&self, time: DateTime<Utc>) -> WeatherAttributes {
        let t = Self::time_to_noise_input(time);

        let cloudiness_noise = self.cloudiness.get([t * 5.5, 0.0]);
        let moisture_noise = self.moisture.get([t * 3.25, 1_000_000.0]);
        let wind_noise = self.wind.get([t * 40.0, t * 50.0]);
        let temperature_noise = self.temperature.get([t, 2_000_000.0]);

        WeatherAttributes {
            cloudiness: Self::normalize_noise(cloudiness_noise),
            moisture: Self::normalize_noise(moisture_noise),
            wind: Self::normalize_noise(wind_noise),
            temperature: Self::normalize_noise(temperature_noise)
                * Self::temperature_multiplier(time),
        }
    }
}
