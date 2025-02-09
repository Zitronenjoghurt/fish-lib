use crate::config::{Config, ConfigBuilderInterface, ConfigInterface};
use crate::data::location_data::LocationData;
use crate::tests::mock::{mock_default_service_provider, mock_service_provider};
use std::collections::HashMap;
use std::sync::Arc;

fn mock_config() -> Arc<dyn ConfigInterface> {
    let test_data = LocationData::default();
    let mut location_data_map = HashMap::new();
    location_data_map.insert(1, test_data);
    Config::builder()
        .locations(location_data_map)
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
fn test_unlock_location() {
    let sp = mock_service_provider(mock_config());
    let location_data = sp.config().get_location_data(1).unwrap();

    let user = sp.user_service().create_and_save_user(1337).unwrap();

    let user_location = sp
        .user_service()
        .unlock_location(&user, location_data.clone())
        .unwrap();

    let user_locations = sp.user_service().get_unlocked_locations(&user).unwrap();
    assert_eq!(user_locations.len(), 1);
    assert_eq!(user_locations[0], user_location);

    let error = sp
        .user_service()
        .unlock_location(&user, location_data.clone())
        .unwrap_err();
    assert!(error.is_already_exists());
    if let Some(resource_error) = error.as_resource_error() {
        assert!(resource_error.is_location_already_unlocked());
        assert_eq!(resource_error.get_location_id(), Some(location_data.id));
        assert_eq!(resource_error.get_external_id(), Some(user.external_id));
    } else {
        panic!("Expected LocationAlreadyUnlocked, got {:?}", error);
    }
}

#[test]
fn test_get_unlocked_location_ids() {
    let sp = mock_service_provider(mock_config());
    let location_data = sp.config().get_location_data(1).unwrap();

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let _ = sp
        .user_service()
        .unlock_location(&user, location_data.clone())
        .unwrap();

    let user_location_ids = sp.user_service().get_unlocked_location_ids(&user).unwrap();
    assert_eq!(user_location_ids.len(), 1);
    assert_eq!(user_location_ids[0], location_data.id);
}
