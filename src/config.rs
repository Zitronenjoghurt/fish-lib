use crate::data::location_data::LocationData;
use crate::data::settings::Settings;
use crate::data::species_data::SpeciesData;
use crate::traits::into_arc_map::IntoArcMap;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

pub trait ConfigInterface: Send + Sync {
    fn species(&self) -> Arc<HashMap<i32, Arc<SpeciesData>>>;
    fn locations(&self) -> Arc<HashMap<i32, Arc<LocationData>>>;
    fn settings(&self) -> Arc<Settings>;
    fn specimen_names(&self) -> Arc<HashMap<i32, String>>;
    fn location_names(&self) -> Arc<HashMap<i32, String>>;
    fn get_species_data(&self, species_id: i32) -> Option<Arc<SpeciesData>>;
}

#[derive(Debug, Default, PartialEq)]
pub struct Config {
    /// Mapping specimen to their species ID
    pub species: Arc<HashMap<i32, Arc<SpeciesData>>>,
    pub locations: Arc<HashMap<i32, Arc<LocationData>>>,
    pub settings: Arc<Settings>,
    pub specimen_names: Arc<HashMap<i32, String>>,
    pub location_names: Arc<HashMap<i32, String>>,
}

impl ConfigInterface for Config {
    fn species(&self) -> Arc<HashMap<i32, Arc<SpeciesData>>> {
        self.species.clone()
    }

    fn locations(&self) -> Arc<HashMap<i32, Arc<LocationData>>> {
        self.locations.clone()
    }

    fn settings(&self) -> Arc<Settings> {
        self.settings.clone()
    }

    fn specimen_names(&self) -> Arc<HashMap<i32, String>> {
        self.specimen_names.clone()
    }

    fn location_names(&self) -> Arc<HashMap<i32, String>> {
        self.location_names.clone()
    }

    fn get_species_data(&self, species_id: i32) -> Option<Arc<SpeciesData>> {
        self.species.get(&species_id).cloned()
    }
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

pub trait ConfigBuilderInterface: Send + Sync {
    fn build(self) -> Arc<dyn ConfigInterface>;
}

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilderInterface for ConfigBuilder {
    fn build(self) -> Arc<dyn ConfigInterface> {
        Arc::new(self.config)
    }
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn species(mut self, specimens: HashMap<i32, SpeciesData>) -> Self {
        let species = specimens.into_arc_map();
        self.config.species = Arc::new(species);
        self
    }

    pub fn species_json(self, json_string: &str) -> Result<Self, serde_json::Error> {
        let specimens: HashMap<i32, SpeciesData> = serde_json::from_str(json_string)?;
        Ok(self.species(specimens))
    }

    pub fn species_json_file(
        self,
        json_file_path: &Path,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(json_file_path)?;
        let specimens: HashMap<i32, SpeciesData> = serde_json::from_reader(file)?;
        Ok(self.species(specimens))
    }

    pub fn locations(mut self, locations: HashMap<i32, LocationData>) -> Self {
        let locations = locations.into_arc_map();
        self.config.locations = Arc::new(locations);
        let location_names: HashMap<i32, String> = self
            .config
            .locations
            .iter()
            .map(|(id, data)| (*id, data.name.clone()))
            .collect();
        self.config.location_names = Arc::new(location_names);
        self
    }

    pub fn locations_json(self, json_string: &str) -> Result<Self, serde_json::Error> {
        let locations: HashMap<i32, LocationData> = serde_json::from_str(json_string)?;
        Ok(self.locations(locations))
    }

    pub fn locations_json_file(
        self,
        json_file_path: &Path,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(json_file_path)?;
        let locations: HashMap<i32, LocationData> = serde_json::from_reader(file)?;
        Ok(self.locations(locations))
    }

    pub fn settings(mut self, settings: Settings) -> Self {
        self.config.settings = Arc::new(settings);
        self
    }

    pub fn settings_json(self, json_string: &str) -> Result<Self, serde_json::Error> {
        let settings: Settings = serde_json::from_str(json_string)?;
        Ok(self.settings(settings))
    }

    pub fn settings_json_file(
        self,
        json_file_path: &Path,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(json_file_path)?;
        let settings: Settings = serde_json::from_reader(file)?;
        Ok(self.settings(settings))
    }
}
