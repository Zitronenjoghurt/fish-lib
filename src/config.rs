use crate::data::fish_data::FishData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Config {
    /// Maps fish to their ID
    pub fish: HashMap<i32, FishData>,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    pub fn get_fish_data(&self, data_id: i32) -> Option<&FishData> {
        self.fish.get(&data_id)
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

    pub fn fish(mut self, fish: HashMap<i32, FishData>) -> Self {
        self.config.fish = fish;
        self
    }

    pub fn fish_json(mut self, json_string: &str) -> Result<Self, serde_json::Error> {
        self.config.fish = serde_json::from_str(json_string)?;
        Ok(self)
    }

    pub fn fish_json_file(
        mut self,
        json_file_path: &Path,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(json_file_path)?;
        self.config.fish = serde_json::from_reader(file)?;
        Ok(self)
    }
}
