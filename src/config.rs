use crate::config::validation_error::ConfigValidationError;
use crate::config::validation_report::ConfigValidationReport;
use crate::data::encounter_data::EncounterData;
use crate::data::item_data::ItemData;
use crate::data::location_data::LocationData;
use crate::data::settings::Settings;
use crate::data::species_data::SpeciesData;
use crate::models::item::attributes::ItemAttributesType;
use crate::models::item::properties::ItemPropertiesType;
use crate::models::item::properties_container::ItemPropertiesContainerInterface;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

mod validation_error;
mod validation_report;

pub trait ConfigInterface: Debug + Send + Sync {
    fn species(&self) -> Arc<HashMap<i32, Arc<SpeciesData>>>;
    fn locations(&self) -> Arc<HashMap<i32, Arc<LocationData>>>;
    fn items(&self) -> Arc<HashMap<i32, Arc<ItemData>>>;
    fn settings(&self) -> Arc<Settings>;
    fn species_names(&self) -> Arc<HashMap<i32, String>>;
    fn location_names(&self) -> Arc<HashMap<i32, String>>;
    fn item_names(&self) -> Arc<HashMap<i32, String>>;
    fn items_by_attributes(&self) -> Arc<HashMap<ItemAttributesType, Arc<Vec<Arc<ItemData>>>>>;
    fn items_by_properties(&self) -> Arc<HashMap<ItemPropertiesType, Arc<Vec<Arc<ItemData>>>>>;

    fn get_species_data(&self, species_id: i32) -> Option<Arc<SpeciesData>> {
        self.species().get(&species_id).cloned()
    }

    fn get_location_data(&self, location_id: i32) -> Option<Arc<LocationData>> {
        self.locations().get(&location_id).cloned()
    }

    fn get_item_data(&self, item_id: i32) -> Option<Arc<ItemData>> {
        self.items().get(&item_id).cloned()
    }

    fn get_items_by_attributes_type(
        &self,
        attributes_type: ItemAttributesType,
    ) -> Option<Arc<Vec<Arc<ItemData>>>> {
        self.items_by_attributes().get(&attributes_type).cloned()
    }

    fn get_items_by_properties_type(
        &self,
        properties_type: ItemPropertiesType,
    ) -> Option<Arc<Vec<Arc<ItemData>>>> {
        self.items_by_properties().get(&properties_type).cloned()
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Config {
    /// Mapping specimen to their species ID
    species: Arc<HashMap<i32, Arc<SpeciesData>>>,
    locations: Arc<HashMap<i32, Arc<LocationData>>>,
    items: Arc<HashMap<i32, Arc<ItemData>>>,
    settings: Arc<Settings>,
    species_names: Arc<HashMap<i32, String>>,
    location_names: Arc<HashMap<i32, String>>,
    item_names: Arc<HashMap<i32, String>>,
    items_by_attributes: Arc<HashMap<ItemAttributesType, Arc<Vec<Arc<ItemData>>>>>,
    items_by_properties: Arc<HashMap<ItemPropertiesType, Arc<Vec<Arc<ItemData>>>>>,
}

impl ConfigInterface for Config {
    fn species(&self) -> Arc<HashMap<i32, Arc<SpeciesData>>> {
        self.species.clone()
    }

    fn locations(&self) -> Arc<HashMap<i32, Arc<LocationData>>> {
        self.locations.clone()
    }

    fn items(&self) -> Arc<HashMap<i32, Arc<ItemData>>> {
        self.items.clone()
    }

    fn settings(&self) -> Arc<Settings> {
        self.settings.clone()
    }

    fn species_names(&self) -> Arc<HashMap<i32, String>> {
        self.species_names.clone()
    }

    fn location_names(&self) -> Arc<HashMap<i32, String>> {
        self.location_names.clone()
    }

    fn item_names(&self) -> Arc<HashMap<i32, String>> {
        self.item_names.clone()
    }

    fn items_by_attributes(&self) -> Arc<HashMap<ItemAttributesType, Arc<Vec<Arc<ItemData>>>>> {
        self.items_by_attributes.clone()
    }

    fn items_by_properties(&self) -> Arc<HashMap<ItemPropertiesType, Arc<Vec<Arc<ItemData>>>>> {
        self.items_by_properties.clone()
    }
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

pub trait ConfigBuilderInterface: Send + Sync {
    fn build(self) -> Result<Arc<dyn ConfigInterface>, ConfigValidationReport>;
}

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilderInterface for ConfigBuilder {
    fn build(self) -> Result<Arc<dyn ConfigInterface>, ConfigValidationReport> {
        let validation_report = self.validate();
        if validation_report.has_errors() {
            Err(validation_report)
        } else {
            Ok(Arc::new(self.config))
        }
    }
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn species(mut self, specimens: HashMap<i32, SpeciesData>) -> Self {
        let species = specimens
            .into_iter()
            .map(|(id, mut data)| {
                data.id = id;
                (id, Arc::new(data))
            })
            .collect();
        self.config.species = Arc::new(species);

        self.map_species_names();

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
        let locations = locations
            .into_iter()
            .map(|(id, mut data)| {
                data.id = id;
                (id, Arc::new(data))
            })
            .collect();
        self.config.locations = Arc::new(locations);

        self.map_location_names();

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

    pub fn items(mut self, items: HashMap<i32, ItemData>) -> Self {
        let items = items
            .into_iter()
            .map(|(id, mut data)| {
                data.id = id;
                (id, Arc::new(data))
            })
            .collect();
        self.config.items = Arc::new(items);

        self.map_item_names();
        self.map_items_by_attributes_and_properties_types();

        self
    }

    pub fn items_json(self, json_string: &str) -> Result<Self, serde_json::Error> {
        let items: HashMap<i32, ItemData> = serde_json::from_str(json_string)?;
        Ok(self.items(items))
    }

    pub fn items_json_file(
        self,
        json_file_path: &Path,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(json_file_path)?;
        let items: HashMap<i32, ItemData> = serde_json::from_reader(file)?;
        Ok(self.items(items))
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

    // Helper function for building config indices
    fn map_species_names(&mut self) {
        let species_names: HashMap<i32, String> = self
            .config
            .species
            .iter()
            .map(|(id, data)| (*id, data.name.clone()))
            .collect();
        self.config.species_names = Arc::new(species_names);
    }

    fn map_location_names(&mut self) {
        let location_names: HashMap<i32, String> = self
            .config
            .locations
            .iter()
            .map(|(id, data)| (*id, data.name.clone()))
            .collect();
        self.config.location_names = Arc::new(location_names);
    }

    fn map_item_names(&mut self) {
        let item_names: HashMap<i32, String> = self
            .config
            .items
            .iter()
            .map(|(id, data)| (*id, data.name.clone()))
            .collect();
        self.config.item_names = Arc::new(item_names);
    }

    fn map_items_by_attributes_and_properties_types(&mut self) {
        let mut by_attributes: HashMap<ItemAttributesType, Vec<Arc<ItemData>>> = HashMap::new();
        let mut by_properties: HashMap<ItemPropertiesType, Vec<Arc<ItemData>>> = HashMap::new();

        self.config.items.values().for_each(|item| {
            item.attributes
                .get_attributes_types()
                .iter()
                .for_each(|attr_type| {
                    by_attributes
                        .entry(*attr_type)
                        .or_default()
                        .push(item.clone());
                });

            item.default_properties
                .get_properties_types()
                .iter()
                .for_each(|prop_type| {
                    by_properties
                        .entry(*prop_type)
                        .or_default()
                        .push(item.clone());
                });
        });

        let by_attributes = by_attributes
            .into_iter()
            .map(|(k, v)| (k, Arc::new(v)))
            .collect();
        let by_properties = by_properties
            .into_iter()
            .map(|(k, v)| (k, Arc::new(v)))
            .collect();

        self.config.items_by_attributes = Arc::new(by_attributes);
        self.config.items_by_properties = Arc::new(by_properties);
    }

    // Validation
    pub fn validate(&self) -> ConfigValidationReport {
        let mut report = ConfigValidationReport::new();
        self.validate_species(&mut report);
        self.validate_locations(&mut report);
        self.validate_items(&mut report);
        report
    }

    fn validate_locations(&self, report: &mut ConfigValidationReport) {
        for location_data in self.config.locations.values() {
            for required_location_id in &location_data.required_locations_unlocked {
                self.validate_locations_required_locations(
                    report,
                    location_data,
                    *required_location_id,
                );
            }

            for required_species_id in &location_data.required_species_caught {
                self.validate_locations_required_species(
                    report,
                    location_data,
                    *required_species_id,
                )
            }
        }
    }

    fn validate_locations_required_locations(
        &self,
        report: &mut ConfigValidationReport,
        location_data: &Arc<LocationData>,
        required_location_id: i32,
    ) {
        if self
            .config
            .get_location_data(required_location_id)
            .is_none()
        {
            report.add_error(ConfigValidationError::location_required_location(
                location_data.id,
                required_location_id,
            ));
        }
    }

    fn validate_locations_required_species(
        &self,
        report: &mut ConfigValidationReport,
        location_data: &Arc<LocationData>,
        required_species_id: i32,
    ) {
        if self.config.get_species_data(required_species_id).is_none() {
            report.add_error(ConfigValidationError::location_required_species(
                location_data.id,
                required_species_id,
            ));
        }
    }

    fn validate_species(&self, report: &mut ConfigValidationReport) {
        for species_data in self.config.species.values() {
            for encounter in &species_data.encounters {
                self.validate_species_encounters(report, species_data, encounter);
            }
        }
    }

    fn validate_species_encounters(
        &self,
        report: &mut ConfigValidationReport,
        species_data: &Arc<SpeciesData>,
        encounter_data: &EncounterData,
    ) {
        let location_id = encounter_data.location_id;
        if self.config.get_location_data(location_id).is_none() {
            report.add_error(ConfigValidationError::species_encounter_location(
                species_data.id,
                location_id,
            ));
        }
    }

    fn validate_items(&self, report: &mut ConfigValidationReport) {
        for item_data in self.config.items.values() {
            if item_data.max_count < 1 {
                report.add_error(ConfigValidationError::item_invalid_max_count(item_data.id));
            }

            if item_data.is_stackable() && item_data.max_count != 1 {
                report.add_error(ConfigValidationError::item_non_unique_not_stackable(
                    item_data.id,
                ));
            }
        }
    }
}
