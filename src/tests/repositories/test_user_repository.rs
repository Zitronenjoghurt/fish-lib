use crate::tests::mock::mock_default_service_provider;

#[test]
fn test_find_by_external_id() {
    let sp = mock_default_service_provider();

    let user = sp.user_service().create_and_save_user(1337).unwrap();

    let found_user = sp
        .user_repository()
        .find_by_external_id(1337)
        .unwrap()
        .unwrap();
    assert_eq!(user, found_user);
}

#[test]
fn test_unlock_location() {
    let sp = mock_default_service_provider();

    let user = sp.user_service().create_and_save_user(1337).unwrap();

    let user_location = sp.user_repository().unlock_location(user.id, 68).unwrap();

    let unlocked_locations = sp
        .user_repository()
        .find_unlocked_locations(user.id)
        .unwrap();
    assert_eq!(unlocked_locations.len(), 1);
    assert_eq!(unlocked_locations[0], user_location);
}

#[test]
fn test_unlocked_location_ids() {
    let sp = mock_default_service_provider();

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let _ = sp.user_repository().unlock_location(user.id, 68).unwrap();

    let unlocked_location_ids = sp
        .user_repository()
        .find_unlocked_location_ids(user.id)
        .unwrap();
    assert_eq!(unlocked_location_ids.len(), 1);
    assert_eq!(unlocked_location_ids[0], 68);
}

#[test]
fn test_find() {
    let sp = mock_default_service_provider();

    let user = sp.user_service().create_and_save_user(1337).unwrap();

    let found_user = sp.user_repository().find(user.id).unwrap().unwrap();
    assert_eq!(user, found_user);
}

#[test]
fn test_save() {
    let sp = mock_default_service_provider();

    let mut user = sp.user_service().create_and_save_user(1337).unwrap();
    user.external_id = 1338;

    sp.user_repository().save(user.clone()).unwrap();
    assert_eq!(
        sp.user_repository().find_by_external_id(1337).unwrap(),
        None
    );
    let found_user = sp
        .user_repository()
        .find_by_external_id(1338)
        .unwrap()
        .unwrap();
    assert_eq!(user.created_at, found_user.created_at);
    assert_eq!(found_user.credits, 0);
}

#[test]
fn test_delete() {
    let sp = mock_default_service_provider();

    let user = sp.user_service().create_and_save_user(1337).unwrap();

    let found_user = sp.user_repository().find(user.id).unwrap().unwrap();
    assert_eq!(user, found_user);

    let found_user_id = found_user.id;
    sp.user_repository().delete(found_user).unwrap();
    assert_eq!(sp.user_repository().find(found_user_id).unwrap(), None);
}
