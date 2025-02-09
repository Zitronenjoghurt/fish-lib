use crate::config::{Config, ConfigBuilderInterface, ConfigInterface};
use crate::data::location_data::LocationData;
use crate::tests::mock::mock_service_provider;
use chrono_tz::Tz;
use std::collections::HashMap;
use std::sync::Arc;

fn mock_config() -> Arc<dyn ConfigInterface> {
    let island_data = LocationData {
        name: "island".to_string(),
        timezone: Tz::America__Costa_Rica,
        ..Default::default()
    };

    let oslo_data = LocationData {
        name: "oslo".to_string(),
        timezone: Tz::Europe__Oslo,
        ..Default::default()
    };

    let mut location_data_map = HashMap::new();
    location_data_map.insert(2, island_data);
    location_data_map.insert(41, oslo_data);

    Config::builder().locations(location_data_map).build()
}

#[test]
fn test_get_location_names() {
    let sp = mock_service_provider(mock_config());

    let location_names = sp.location_service().get_location_names();
    assert_eq!(location_names.len(), 2);
    assert_eq!(location_names[&2], "island");
    assert_eq!(location_names[&41], "oslo");
}

#[test]
fn test_get_species_data() {
    let sp = mock_service_provider(mock_config());

    let island_data = sp.location_service().get_location_data(2).unwrap();
    assert_eq!(island_data.id, 2);
    assert_eq!(island_data.name, "island");
    assert_eq!(island_data.timezone, Tz::America__Costa_Rica);

    let oslo_data = sp.location_service().get_location_data(41).unwrap();
    assert_eq!(oslo_data.id, 41);
    assert_eq!(oslo_data.name, "oslo");
    assert_eq!(oslo_data.timezone, Tz::Europe__Oslo);
}
