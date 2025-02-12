use crate::config::{Config, ConfigBuilderInterface, ConfigInterface};
use crate::data::encounter_data::EncounterData;
use crate::data::location_data::LocationData;
use crate::data::species_data::SpeciesData;
use crate::models::item::attributes::ItemAttributesType;
use crate::models::item::attributes_container::ItemAttributesContainerInterface;
use crate::models::item::properties::ItemPropertiesType;
use crate::models::item::properties_container::ItemPropertiesContainerInterface;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

fn mock_file_config() -> Arc<dyn ConfigInterface> {
    let locations_json_file = Path::new("./example_data/locations.json");
    let species_json_file = Path::new("./example_data/species_data.json");
    let settings_json_file = Path::new("./example_data/settings.json");
    let items_json_file = Path::new("./example_data/items.json");

    Config::builder()
        .locations_json_file(locations_json_file)
        .unwrap()
        .species_json_file(species_json_file)
        .unwrap()
        .settings_json_file(settings_json_file)
        .unwrap()
        .items_json_file(items_json_file)
        .unwrap()
        .build()
        .unwrap()
}

#[test]
fn test_building() {
    let config = mock_file_config();

    assert_eq!(config.species().get(&1).unwrap().name, "Salmon");
    assert_eq!(config.settings().time_speed_multiplier as u64, 1);

    let item1 = config.get_item_data(1).unwrap();
    assert!(item1.is_bait());
    assert!(item1.is_purchasable());
    assert_eq!(item1.get_bait_level(), Some(1));
    assert_eq!(item1.get_cost(), Some(50));
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

#[test]
fn test_mapping_items_by_attributes_and_properties() {
    let config = mock_file_config();

    let shop_items = config
        .get_items_by_attributes_type(ItemAttributesType::Purchasable)
        .unwrap();

    assert_eq!(shop_items.len(), 1);
    let bait = shop_items[0].clone();
    assert!(bait.is_bait());
    assert!(bait.is_purchasable());
    assert_eq!(bait.name, "Bob");

    let usable_items = config
        .get_items_by_properties_type(ItemPropertiesType::Usage)
        .unwrap();

    assert_eq!(usable_items.len(), 1);
    let rod = usable_items[0].clone();
    assert!(rod.is_rod());
    assert!(rod.has_usage());
    assert_eq!(rod.name, "Bobber");
}
