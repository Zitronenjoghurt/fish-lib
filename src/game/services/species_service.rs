use crate::config::ConfigInterface;
use crate::data::species_data::SpeciesData;
use std::collections::HashMap;
use std::sync::Arc;

pub trait SpeciesServiceInterface: Send + Sync {
    fn get_species_names(&self) -> Arc<HashMap<i32, String>>;
    fn get_species_data(&self, species_id: i32) -> Option<Arc<SpeciesData>>;
}

pub struct SpeciesService {
    config: Arc<dyn ConfigInterface>,
}

impl SpeciesService {
    pub fn new(config: Arc<dyn ConfigInterface>) -> SpeciesService {
        SpeciesService { config }
    }
}

impl SpeciesServiceInterface for SpeciesService {
    fn get_species_names(&self) -> Arc<HashMap<i32, String>> {
        self.config.species_names().clone()
    }

    fn get_species_data(&self, species_id: i32) -> Option<Arc<SpeciesData>> {
        self.config.get_species_data(species_id)
    }
}
