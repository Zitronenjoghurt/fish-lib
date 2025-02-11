use crate::config::{Config, ConfigBuilderInterface};
use crate::data::encounter_data::EncounterData;
use crate::data::location_data::LocationData;
use crate::data::species_data::SpeciesData;
use crate::models::item::attributes::ItemAttributesContainerInterface;
use crate::models::item::properties::ItemPropertiesInterface;
use std::collections::HashMap;
use std::path::Path;

#[test]
fn test_building() {
    let locations_json_file = Path::new("./example_data/locations.json");
    let species_json_file = Path::new("./example_data/species_data.json");
    let settings_json_file = Path::new("./example_data/settings.json");
    let items_json_file = Path::new("./example_data/items.json");

    let config = Config::builder()
        .locations_json_file(locations_json_file)
        .unwrap()
        .species_json_file(species_json_file)
        .unwrap()
        .settings_json_file(settings_json_file)
        .unwrap()
        .items_json_file(items_json_file)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.species().get(&1).unwrap().name, "Salmon");
    assert_eq!(config.settings().time_speed_multiplier as u64, 1);

    let item1 = config.get_item_data(1).unwrap();
    assert_eq!(item1.name, "Bob");

    let item2 = config.get_item_data(2).unwrap();
    assert!(item2.is_rod());
    assert_eq!(item2.get_times_used(), Some(0));
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

    let location_data = LocationData {
        required_locations_unlocked: vec![800],
        required_species_caught: vec![700],
        ..Default::default()
    };

    let mut locations_data_map = HashMap::new();
    locations_data_map.insert(5, location_data);

    let validation_report = Config::builder()
        .locations(locations_data_map)
        .species(species_data_map)
        .build()
        .unwrap_err();

    let errors = validation_report.errors();
    assert_eq!(errors.len(), 3);

    let species_encounter_location_error = &errors[0];
    assert!(species_encounter_location_error.is_species_encounter_location());
    assert_eq!(
        species_encounter_location_error.get_source_species_id(),
        Some(4)
    );
    assert_eq!(
        species_encounter_location_error.get_target_location_id(),
        Some(67)
    );

    let location_required_location_error = &errors[1];
    assert!(location_required_location_error.is_location_required_location());
    assert_eq!(
        location_required_location_error.get_source_location_id(),
        Some(5)
    );
    assert_eq!(
        location_required_location_error.get_target_location_id(),
        Some(800)
    );

    let location_required_species_error = &errors[2];
    assert!(location_required_species_error.is_location_required_species());
    assert_eq!(
        location_required_species_error.get_source_location_id(),
        Some(5)
    );
    assert_eq!(
        location_required_species_error.get_target_species_id(),
        Some(700)
    );
}
