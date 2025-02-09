use crate::config::{Config, ConfigBuilderInterface};
use crate::data::encounter_data::EncounterData;
use crate::data::species_data::SpeciesData;
use std::collections::HashMap;
use std::path::Path;

#[test]
fn test_building() {
    let locations_json_file = Path::new("./example_data/locations.json");
    let species_json_file = Path::new("./example_data/species_data.json");
    let settings_json_file = Path::new("./example_data/settings.json");

    let config = Config::builder()
        .locations_json_file(locations_json_file)
        .unwrap()
        .species_json_file(species_json_file)
        .unwrap()
        .settings_json_file(settings_json_file)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.species().get(&1).unwrap().name, "Salmon");
    assert_eq!(config.settings().time_speed_multiplier as u64, 1);
}

#[test]
fn test_validation() {
    let encounter_data = EncounterData {
        location_id: 67,
        ..Default::default()
    };

    let species_data = SpeciesData {
        encounters: vec![encounter_data],
        ..Default::default()
    };

    let mut species_data_map = HashMap::new();
    species_data_map.insert(4, species_data);

    let validation_report = Config::builder()
        .species(species_data_map)
        .build()
        .unwrap_err();

    let errors = validation_report.errors();

    let invalid_encounter_location_error = &errors[0];
    assert!(invalid_encounter_location_error.is_invalid_encounter_location());
    assert_eq!(invalid_encounter_location_error.get_location_id(), Some(67));
    assert_eq!(invalid_encounter_location_error.get_species_id(), Some(4));
}
