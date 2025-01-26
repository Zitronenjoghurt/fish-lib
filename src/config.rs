use crate::data::fish_data::FishData;
use crate::data::settings::Settings;
use crate::traits::into_arc_map::IntoArcMap;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Default, PartialEq)]
pub struct Config {
    /// Mapping fish to their species ID
    pub fish: HashMap<i32, Arc<FishData>>,
    pub settings: Arc<Settings>,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    pub fn get_fish_data(&self, species_id: i32) -> Option<Arc<FishData>> {
        self.fish.get(&species_id).cloned()
    }
}

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Config {
        self.config
    }

    pub fn fish_json(mut self, json_string: &str) -> Result<Self, serde_json::Error> {
        let fish: HashMap<i32, FishData> = serde_json::from_str(json_string)?;
        self.config.fish = fish.into_arc_map();
        Ok(self)
    }

    pub fn fish_json_file(
        mut self,
        json_file_path: &Path,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(json_file_path)?;
        let fish: HashMap<i32, FishData> = serde_json::from_reader(file)?;
        self.config.fish = fish.into_arc_map();
        Ok(self)
    }

    pub fn settings_json(mut self, json_string: &str) -> Result<Self, serde_json::Error> {
        let settings: Settings = serde_json::from_str(json_string)?;
        self.config.settings = Arc::new(settings);
        Ok(self)
    }

    pub fn settings_json_file(
        mut self,
        json_file_path: &Path,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(json_file_path)?;
        let settings: Settings = serde_json::from_reader(file)?;
        self.config.settings = Arc::new(settings);
        Ok(self)
    }
}
