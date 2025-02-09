use crate::config::{Config, ConfigBuilderInterface, ConfigInterface};
use crate::data::species_data::SpeciesData;
use crate::tests::mock::mock_service_provider;
use std::collections::HashMap;
use std::sync::Arc;

fn mock_config() -> Arc<dyn ConfigInterface> {
    let test_data = SpeciesData {
        ..Default::default()
    };

    let mut species_data_map = HashMap::new();
    species_data_map.insert(1, test_data);

    Config::builder().species(species_data_map).build()
}

#[test]
fn test_generate_and_save_specimen() {
    let sp = mock_service_provider(mock_config());
    let species = sp.species_service().get_species_data(1).unwrap();

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let specimen = sp
        .specimen_service()
        .generate_and_save_specimen(&user, species)
        .unwrap();

    let found_specimen = sp.specimen_repository().find(specimen.id).unwrap().unwrap();
    assert_eq!(found_specimen, specimen);
}

#[test]
fn test_process_catch() {
    let sp = mock_service_provider(mock_config());
    let species = sp.species_service().get_species_data(1).unwrap();

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let specimen = sp.specimen_service().process_catch(&user, species).unwrap();

    let user_specimens = sp.specimen_repository().find_by_user(&user).unwrap();
    let first_specimen = user_specimens.first().unwrap();
    assert_eq!(*first_specimen, specimen);
}
