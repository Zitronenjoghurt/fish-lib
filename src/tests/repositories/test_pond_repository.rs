use crate::tests::mock::mock_default_service_provider;

#[test]
fn test_find_by_user() {
    let sp = mock_default_service_provider();

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let _ = sp.pond_service().create_and_save_pond(&user, 50).unwrap();
    let _ = sp.pond_service().create_and_save_pond(&user, 50).unwrap();
    let _ = sp.pond_service().create_and_save_pond(&user, 50).unwrap();
    let _ = sp.pond_service().create_and_save_pond(&user, 50).unwrap();

    let all_user_ponds = sp.pond_repository().find_by_user(&user).unwrap();
    assert_eq!(all_user_ponds.len(), 4);

    let first_pond = all_user_ponds.first().unwrap();
    assert_eq!(first_pond.capacity, 50);
    assert_eq!(first_pond.user_id, user.id);
}

#[test]
fn test_find() {
    let sp = mock_default_service_provider();

    let user = sp.user_service().create_and_save_user(1337).unwrap();

    let pond = sp.pond_service().create_and_save_pond(&user, 50).unwrap();
    let found_pond = sp.pond_repository().find(pond.id).unwrap().unwrap();
    assert_eq!(pond, found_pond);
}

#[test]
fn test_save() {
    let sp = mock_default_service_provider();

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let mut pond = sp.pond_service().create_and_save_pond(&user, 50).unwrap();
    pond.capacity = 51;

    sp.pond_repository().save(pond.clone()).unwrap();
    let found_pond = sp.pond_repository().find(pond.id).unwrap().unwrap();
    assert_eq!(found_pond.capacity, 51);
    assert_eq!(pond.created_at, found_pond.created_at);
}

#[test]
fn test_delete() {
    let sp = mock_default_service_provider();

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let pond = sp.pond_service().create_and_save_pond(&user, 50).unwrap();

    let found_pond = sp.pond_repository().find(pond.id).unwrap().unwrap();
    assert_eq!(pond, found_pond);

    sp.pond_repository().delete(&found_pond).unwrap();
    assert_eq!(sp.pond_repository().find(found_pond.id).unwrap(), None);
}
