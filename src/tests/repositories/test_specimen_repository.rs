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
    species_data_map.insert(2, test_data);

    Config::builder().species(species_data_map).build().unwrap()
}

#[test]
fn test_find_by_user() {
    let sp = mock_service_provider(mock_config());
    let species = sp.species_service().get_species_data(2).unwrap();

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let _ = sp
        .specimen_service()
        .generate_and_save_specimen(&user, species.clone())
        .unwrap();
    let _ = sp
        .specimen_service()
        .generate_and_save_specimen(&user, species.clone())
        .unwrap();
    let _ = sp
        .specimen_service()
        .generate_and_save_specimen(&user, species.clone())
        .unwrap();
    let _ = sp
        .specimen_service()
        .generate_and_save_specimen(&user, species.clone())
        .unwrap();

    let all_user_specimen = sp.specimen_repository().find_by_user(&user).unwrap();
    assert_eq!(all_user_specimen.len(), 4);

    let first_specimen = all_user_specimen.first().unwrap();
    assert_eq!(first_specimen.species_id, 2);
    assert_eq!(first_specimen.user_id, user.id);
}

#[test]
fn test_find() {
    let sp = mock_service_provider(mock_config());
    let species = sp.species_service().get_species_data(2).unwrap();

    let user = sp.user_service().create_and_save_user(1337).unwrap();

    let specimen = sp
        .specimen_service()
        .generate_and_save_specimen(&user, species)
        .unwrap();
    let found_specimen = sp.specimen_repository().find(specimen.id).unwrap().unwrap();
    assert_eq!(specimen, found_specimen);
}

#[test]
fn test_save() {
    let sp = mock_service_provider(mock_config());
    let species = sp.species_service().get_species_data(2).unwrap();

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let mut specimen = sp
        .specimen_service()
        .generate_and_save_specimen(&user, species)
        .unwrap();
    specimen.species_id = 2;

    sp.specimen_repository().save(specimen.clone()).unwrap();
    let found_specimen = sp.specimen_repository().find(specimen.id).unwrap().unwrap();
    assert_eq!(found_specimen.species_id, 2);
    assert_eq!(specimen.created_at, found_specimen.created_at);
}

#[test]
fn test_delete() {
    let sp = mock_service_provider(mock_config());
    let species = sp.species_service().get_species_data(2).unwrap();

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let specimen = sp
        .specimen_service()
        .generate_and_save_specimen(&user, species)
        .unwrap();

    let found_specimen = sp.specimen_repository().find(specimen.id).unwrap().unwrap();
    assert_eq!(specimen, found_specimen);

    let found_specimen_id = found_specimen.id;
    sp.specimen_repository().delete(found_specimen).unwrap();
    assert_eq!(
        sp.specimen_repository().find(found_specimen_id).unwrap(),
        None
    );
}
