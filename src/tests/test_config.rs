use crate::config::{Config, ConfigBuilderInterface, ConfigInterface};
use crate::data::encounter_data::EncounterData;
use crate::data::item_data::ItemData;
use crate::data::location_data::LocationData;
use crate::data::species_data::SpeciesData;
use crate::enums::item_category::ItemCategory;
use crate::models::item::attributes::ItemAttributesType;
use crate::models::item::attributes_container::ItemAttributesContainerInterface;
use crate::models::item::properties::ItemPropertiesType;
use crate::models::item::properties_container::{
    ItemPropertiesContainer, ItemPropertiesContainerInterface,
};
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
    assert!(config.is_item_in_category(item1.clone(), ItemCategory::Bait));
    assert!(config.is_item_in_category(item1.clone(), ItemCategory::Shop));
    assert!(!config.is_item_in_category(item1.clone(), ItemCategory::Rod));

    let item2 = config.get_item_data(2).unwrap();
    assert!(item2.is_rod());
    assert_eq!(item2.get_times_used(), Some(0));
    assert!(!config.is_item_in_category(item2.clone(), ItemCategory::Bait));
    assert!(!config.is_item_in_category(item2.clone(), ItemCategory::Shop));
    assert!(config.is_item_in_category(item2.clone(), ItemCategory::Rod));
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

    let location_data = LocationData {
        required_locations_unlocked: vec![800],
        required_species_caught: vec![700],
        ..Default::default()
    };

    let item_data = ItemData {
        max_count: 0,
        ..Default::default()
    };

    let item_data2 = ItemData {
        max_count: 2,
        default_properties: ItemPropertiesContainer::new().with_stackable(0),
        ..Default::default()
    };

    let species_data_map = HashMap::from([(4, species_data)]);
    let locations_data_map = HashMap::from([(5, location_data)]);
    let items_data_map = HashMap::from([(1, item_data), (2, item_data2)]);

    let validation_report = Config::builder()
        .locations(locations_data_map)
        .species(species_data_map)
        .items(items_data_map)
        .build()
        .unwrap_err();

    let errors = validation_report.errors();
    assert_eq!(errors.len(), 5);

    assert!(errors.iter().any(|e| {
        e.is_species_encounter_location()
            && e.get_source_species_id() == Some(4)
            && e.get_target_location_id() == Some(67)
    }));

    assert!(errors.iter().any(|e| {
        e.is_location_required_location()
            && e.get_source_location_id() == Some(5)
            && e.get_target_location_id() == Some(800)
    }));

    assert!(errors.iter().any(|e| {
        e.is_location_required_species()
            && e.get_source_location_id() == Some(5)
            && e.get_target_species_id() == Some(700)
    }));

    assert!(errors
        .iter()
        .any(|e| { e.is_item_invalid_max_count() && e.get_source_item_id() == Some(1) }));

    assert!(errors
        .iter()
        .any(|e| { e.is_item_non_unique_not_stackable() && e.get_source_item_id() == Some(2) }));
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
