use crate::config::{Config, ConfigBuilderInterface, ConfigInterface};
use crate::data::item_data::ItemData;
use crate::models::item::attributes::ItemAttributesContainer;
use crate::models::item::properties::{ItemProperties, ItemPropertiesInterface};
use crate::models::item::{Item, ItemInterface};
use std::collections::HashMap;
use std::sync::Arc;

const NOTHING_ITEM: i32 = 1;
const ROD_ITEM: i32 = 2;

fn mock_config() -> Arc<dyn ConfigInterface> {
    let nothing_item = ItemData {
        name: "Nothing really".to_string(),
        attributes: ItemAttributesContainer::new(),
        default_properties: ItemProperties::new(),
        ..Default::default()
    };

    let rod_item = ItemData {
        name: "Rod".to_string(),
        attributes: ItemAttributesContainer::new().with_rod(1),
        default_properties: ItemProperties::new().with_usage(0),
        ..Default::default()
    };

    let item_data_map = HashMap::from([(NOTHING_ITEM, nothing_item), (ROD_ITEM, rod_item)]);
    Config::builder().items(item_data_map).build().unwrap()
}

fn create_non_existent_item() -> Item {
    Item {
        type_id: -1,
        ..Default::default()
    }
}

fn create_item(config: &Arc<dyn ConfigInterface>, type_id: i32) -> Item {
    let data = config.get_item_data(type_id).unwrap();
    Item {
        type_id,
        properties: data.default_properties.clone(),
        ..Default::default()
    }
}

#[test]
fn test_use_as_rod() {
    let config = mock_config();

    let mut rod = create_item(&config, ROD_ITEM);
    assert!(rod.attributes(config.clone()).unwrap().is_rod());
    assert_eq!(rod.get_times_used(), Some(0));
    rod.use_as_rod(config.clone()).unwrap();
    assert_eq!(rod.get_times_used(), Some(1));

    let mut nothing = create_item(&config, NOTHING_ITEM);
    assert!(!nothing.attributes(config.clone()).unwrap().is_rod());
    assert_eq!(nothing.get_times_used(), None);

    let use_error = nothing.use_as_rod(config.clone()).unwrap_err();
    assert!(use_error.is_not_a_rod());
    assert_eq!(use_error.get_type_id(), Some(NOTHING_ITEM));

    let mut none_existent = create_non_existent_item();
    let existence_error = none_existent.use_as_rod(config.clone()).unwrap_err();
    assert!(existence_error.is_invalid_item_type());
    assert_eq!(existence_error.get_type_id(), Some(-1));
}
