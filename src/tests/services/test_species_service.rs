use crate::config::{Config, ConfigBuilderInterface, ConfigInterface};
use crate::data::species_data::SpeciesData;
use crate::tests::mock::mock_service_provider;
use std::collections::HashMap;
use std::sync::Arc;

fn mock_config() -> Arc<dyn ConfigInterface> {
    let salmon_data = SpeciesData {
        name: "salmon".to_string(),
        min_size_baby_mm: 40,
        max_size_baby_mm: 50,
        min_size_adult_mm: 400,
        max_size_adult_mm: 500,
        min_weight_baby_g: 20,
        max_weight_baby_g: 35,
        min_weight_adult_g: 220,
        max_weight_adult_g: 350,
        min_lifespan_days: 480,
        max_lifespan_days: 720,
        lifespan_adult_ratio: 0.35,
        ..Default::default()
    };

    let shark_data = SpeciesData {
        name: "shark".to_string(),
        ..Default::default()
    };

    let mut species_data_map = HashMap::new();
    species_data_map.insert(1, salmon_data);
    species_data_map.insert(37, shark_data);

    Config::builder().species(species_data_map).build().unwrap()
}

#[test]
fn test_get_species_names() {
    let sp = mock_service_provider(mock_config());

    let species_names = sp.species_service().get_species_names();
    assert_eq!(species_names.len(), 2);
    assert_eq!(species_names[&1], "salmon");
    assert_eq!(species_names[&37], "shark");
}

#[test]
fn test_get_species_data() {
    let sp = mock_service_provider(mock_config());

    let salmon_data = sp.species_service().get_species_data(1).unwrap();
    assert_eq!(salmon_data.id, 1);
    assert_eq!(salmon_data.name, "salmon");
    assert_eq!(salmon_data.min_size_baby_mm, 40);
    assert_eq!(salmon_data.max_size_baby_mm, 50);
    assert_eq!(salmon_data.min_size_adult_mm, 400);
    assert_eq!(salmon_data.max_size_adult_mm, 500);
    assert_eq!(salmon_data.min_weight_baby_g, 20);
    assert_eq!(salmon_data.max_weight_baby_g, 35);
    assert_eq!(salmon_data.min_weight_adult_g, 220);
    assert_eq!(salmon_data.max_weight_adult_g, 350);
    assert_eq!(salmon_data.min_lifespan_days, 480);
    assert_eq!(salmon_data.max_lifespan_days, 720);
    assert_eq!(salmon_data.lifespan_adult_ratio, 0.35);

    let shark_data = sp.species_service().get_species_data(37).unwrap();
    assert_eq!(shark_data.id, 37);
    assert_eq!(shark_data.name, "shark");
}
