use chrono::{DateTime, Timelike, Utc};
use noise::{NoiseFn, Perlin};

#[derive(Debug)]
pub enum Cloudiness {
    Clear,
    VeryLight,
    Light,
    Scattered,
    Moderate,
    Considerable,
    Heavy,
    Overcast,
}

#[derive(Debug)]
pub enum CloudBrightness {
    Dark,
    VeryDim,
    Dim,
    SlightlyDim,
    Neutral,
    SlightlyBright,
    Bright,
    VeryBright,
}

#[derive(Debug)]
pub enum Moisture {
    Arid,
    VeryDry,
    Dry,
    SlightlyDry,
    Moderate,
    SlightlyHumid,
    Humid,
    Saturated,
}

#[derive(Debug)]
pub enum WindPresence {
    Still,
    VeryLight,
    Light,
    Moderate,
    Fresh,
    Strong,
    VeryStrong,
    Severe,
}

#[derive(Debug)]
pub enum WindStrength {
    Calm,
    Light,
    Gentle,
    Moderate,
    Fresh,
    Strong,
    VeryStrong,
    Storm,
}

#[derive(Debug)]
pub enum Temperature {
    Freezing,
    VeryCold,
    Cold,
    Cool,
    Mild,
    Warm,
    Hot,
    VeryHot,
}

#[derive(Debug)]
pub enum Light {
    Dark,
    VeryDim,
    Dim,
    SlightlyDim,
    Moderate,
    Bright,
    VeryBright,
    Intense,
}

#[derive(Default, Debug)]
pub struct WeatherAttributes {
    pub cloudiness: f32,
    pub cloud_brightness: f32,
    pub moisture: f32,
    pub wind_presence: f32,
    pub wind_strength: f32,
    pub temperature: f32,
    pub light: f32,
    pub rain_intensity: f32,
}

impl WeatherAttributes {
    pub fn cloudiness_scale(&self) -> Cloudiness {
        if self.cloudiness > 0.875f32 {
            Cloudiness::Overcast
        } else if self.cloudiness > 0.75f32 {
            Cloudiness::Heavy
        } else if self.cloudiness > 0.625f32 {
            Cloudiness::Considerable
        } else if self.cloudiness > 0.5f32 {
            Cloudiness::Moderate
        } else if self.cloudiness > 0.375f32 {
            Cloudiness::Scattered
        } else if self.cloudiness > 0.25f32 {
            Cloudiness::Light
        } else if self.cloudiness > 0.125f32 {
            Cloudiness::VeryLight
        } else {
            Cloudiness::Clear
        }
    }

    pub fn cloud_brightness_scale(&self) -> CloudBrightness {
        if self.cloud_brightness > 0.875f32 {
            CloudBrightness::VeryBright
        } else if self.cloud_brightness > 0.75f32 {
            CloudBrightness::Bright
        } else if self.cloud_brightness > 0.625f32 {
            CloudBrightness::SlightlyBright
        } else if self.cloud_brightness > 0.5f32 {
            CloudBrightness::Neutral
        } else if self.cloud_brightness > 0.375f32 {
            CloudBrightness::SlightlyDim
        } else if self.cloud_brightness > 0.25f32 {
            CloudBrightness::Dim
        } else if self.cloud_brightness > 0.125f32 {
            CloudBrightness::VeryDim
        } else {
            CloudBrightness::Dark
        }
    }

    pub fn moisture_scale(&self) -> Moisture {
        if self.moisture > 0.875f32 {
            Moisture::Saturated
        } else if self.moisture > 0.75f32 {
            Moisture::Humid
        } else if self.moisture > 0.625f32 {
            Moisture::SlightlyHumid
        } else if self.moisture > 0.5f32 {
            Moisture::Moderate
        } else if self.moisture > 0.375f32 {
            Moisture::SlightlyDry
        } else if self.moisture > 0.25f32 {
            Moisture::Dry
        } else if self.moisture > 0.125f32 {
            Moisture::VeryDry
        } else {
            Moisture::Arid
        }
    }

    pub fn wind_presence_scale(&self) -> WindPresence {
        if self.wind_presence > 0.875f32 {
            WindPresence::Severe
        } else if self.wind_presence > 0.75f32 {
            WindPresence::VeryStrong
        } else if self.wind_presence > 0.625f32 {
            WindPresence::Strong
        } else if self.wind_presence > 0.5f32 {
            WindPresence::Fresh
        } else if self.wind_presence > 0.375f32 {
            WindPresence::Moderate
        } else if self.wind_presence > 0.25f32 {
            WindPresence::Light
        } else if self.wind_presence > 0.125f32 {
            WindPresence::VeryLight
        } else {
            WindPresence::Still
        }
    }

    pub fn wind_strength_scale(&self) -> WindStrength {
        if self.wind_strength > 0.875f32 {
            WindStrength::Storm
        } else if self.wind_strength > 0.75f32 {
            WindStrength::VeryStrong
        } else if self.wind_strength > 0.625f32 {
            WindStrength::Strong
        } else if self.wind_strength > 0.5f32 {
            WindStrength::Fresh
        } else if self.wind_strength > 0.375f32 {
            WindStrength::Moderate
        } else if self.wind_strength > 0.25f32 {
            WindStrength::Gentle
        } else if self.wind_strength > 0.125f32 {
            WindStrength::Light
        } else {
            WindStrength::Calm
        }
    }

    pub fn temperature_scale(&self) -> Temperature {
        if self.temperature > 0.875f32 {
            Temperature::VeryHot
        } else if self.temperature > 0.75f32 {
            Temperature::Hot
        } else if self.temperature > 0.625f32 {
            Temperature::Warm
        } else if self.temperature > 0.5f32 {
            Temperature::Mild
        } else if self.temperature > 0.375f32 {
            Temperature::Cool
        } else if self.temperature > 0.25f32 {
            Temperature::Cold
        } else if self.temperature > 0.125f32 {
            Temperature::VeryCold
        } else {
            Temperature::Freezing
        }
    }

    pub fn light_scale(&self) -> Light {
        if self.light > 0.875f32 {
            Light::Intense
        } else if self.light > 0.75f32 {
            Light::VeryBright
        } else if self.light > 0.625f32 {
            Light::Bright
        } else if self.light > 0.5f32 {
            Light::Moderate
        } else if self.light > 0.375f32 {
            Light::SlightlyDim
        } else if self.light > 0.25f32 {
            Light::Dim
        } else if self.light > 0.125f32 {
            Light::VeryDim
        } else {
            Light::Dark
        }
    }
}

pub struct WeatherSystemConfig {
    pub cloud_brightness_light_block_factor: f32,
}

impl Default for WeatherSystemConfig {
    fn default() -> Self {
        Self {
            cloud_brightness_light_block_factor: 0.7,
        }
    }
}

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
    pub fn new(seed: u32, config: WeatherSystemConfig) -> Self {
        Self {
            cloudiness: Perlin::new(seed),
            cloud_brightness: Perlin::new(seed + 1_000_000),
            moisture: Perlin::new(seed + 2_000_000),
            wind_presence: Perlin::new(seed + 3_000_000),
            wind_strength: Perlin::new(seed + 4_000_000),
            temperature: Perlin::new(seed + 5_000_000),
            rain_intensity: Perlin::new(seed + 6_000_000),
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
}
