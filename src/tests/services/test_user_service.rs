use crate::config::{Config, ConfigBuilderInterface, ConfigInterface};
use crate::data::location_data::LocationData;
use crate::data::species_data::SpeciesData;
use crate::tests::mock::{mock_default_service_provider, mock_service_provider};
use std::collections::HashMap;
use std::sync::Arc;

fn mock_config() -> Arc<dyn ConfigInterface> {
    let location_data1 = LocationData {
        required_locations_unlocked: vec![2, 5],
        required_species_caught: vec![1, 4],
        ..Default::default()
    };
    let location_data2 = LocationData::default();
    let location_data3 = LocationData::default();

    let mut location_data_map = HashMap::new();
    location_data_map.insert(1, location_data1);
    location_data_map.insert(2, location_data2);
    location_data_map.insert(5, location_data3);

    let species_data1 = SpeciesData::default();
    let species_data2 = SpeciesData::default();
    let species_data_map = HashMap::from([(1, species_data1), (4, species_data2)]);

    Config::builder()
        .locations(location_data_map)
        .species(species_data_map)
        .build()
        .unwrap()
}

#[test]
fn test_create_and_save_user() {
    let sp = mock_default_service_provider();

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let found_user = sp.user_repository().find(user.id).unwrap().unwrap();
    assert_eq!(user, found_user);
}

#[test]
fn test_get_unmet_location_unlock_requirements() {
    let sp = mock_service_provider(mock_config());

    let user = sp.user_service().create_and_save_user(1337).unwrap();

    let location1 = sp.location_service().get_location_data(1).unwrap();
    let location2 = sp.location_service().get_location_data(2).unwrap();
    let species = sp.species_service().get_species_data(1).unwrap();

    let unmet_requirements1 = sp
        .user_service()
        .get_unmet_location_unlock_requirements(&user, location1.clone())
        .unwrap();
    assert!(!unmet_requirements1.is_empty());
    assert_eq!(unmet_requirements1.locations_unlocked.len(), 2);
    assert!(unmet_requirements1.locations_unlocked.contains(&2));
    assert!(unmet_requirements1.locations_unlocked.contains(&5));
    assert_eq!(unmet_requirements1.species_caught.len(), 2);
    assert!(unmet_requirements1.species_caught.contains(&1));
    assert!(unmet_requirements1.species_caught.contains(&4));

    sp.user_service()
        .unlock_location(&user, location2.clone())
        .unwrap();
    let specimen = sp.specimen_service().process_catch(&user, species).unwrap();
    sp.fishing_history_service()
        .register_catch(&specimen)
        .unwrap();

    let unmet_requirements2 = sp
        .user_service()
        .get_unmet_location_unlock_requirements(&user, location1)
        .unwrap();
    assert!(!unmet_requirements2.is_empty());
    assert_eq!(unmet_requirements2.locations_unlocked.len(), 1);
    assert!(unmet_requirements2.locations_unlocked.contains(&5));
    assert_eq!(unmet_requirements2.species_caught.len(), 1);
    assert!(unmet_requirements2.species_caught.contains(&4));

    let unmet_requirements3 = sp
        .user_service()
        .get_unmet_location_unlock_requirements(&user, location2)
        .unwrap();
    assert!(unmet_requirements3.is_empty());
}

#[test]
fn test_unlock_location() {
    let sp = mock_service_provider(mock_config());
    let location1 = sp.config().get_location_data(1).unwrap();
    let location2 = sp.config().get_location_data(2).unwrap();

    let user = sp.user_service().create_and_save_user(1337).unwrap();

    let unlock_error = sp
        .user_service()
        .unlock_location(&user, location1.clone())
        .unwrap_err();
    assert!(unlock_error.is_unmet_requirements());
    if let Some(resource_error) = unlock_error.as_resource_error() {
        assert!(resource_error.is_unmet_location_unlock_requirements());
        assert_eq!(resource_error.get_location_id(), Some(location1.id));
    } else {
        panic!(
            "Expected UnmetLocationUnlockRequirements, got {:?}",
            unlock_error
        );
    }

    let user_location = sp
        .user_service()
        .unlock_location(&user, location2.clone())
        .unwrap();

    let user_locations = sp.user_service().get_unlocked_locations(&user).unwrap();
    assert_eq!(user_locations.len(), 1);
    assert_eq!(user_locations[0], user_location);

    let error = sp
        .user_service()
        .unlock_location(&user, location2.clone())
        .unwrap_err();
    assert!(error.is_already_exists());
    if let Some(resource_error) = error.as_resource_error() {
        assert!(resource_error.is_location_already_unlocked());
        assert_eq!(resource_error.get_location_id(), Some(location2.id));
        assert_eq!(resource_error.get_external_id(), Some(user.external_id));
    } else {
        panic!("Expected LocationAlreadyUnlocked, got {:?}", error);
    }
}

#[test]
fn test_get_unlocked_location_ids() {
    let sp = mock_service_provider(mock_config());
    let location_data = sp.config().get_location_data(2).unwrap();

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let _ = sp
        .user_service()
        .unlock_location(&user, location_data.clone())
        .unwrap();

    let user_location_ids = sp.user_service().get_unlocked_location_ids(&user).unwrap();
    assert_eq!(user_location_ids.len(), 1);
    assert_eq!(user_location_ids[0], location_data.id);
}
