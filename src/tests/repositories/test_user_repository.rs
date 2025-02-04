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
