use crate::tests::mock::mock_default_service_provider;

#[test]
fn test_create_and_save_user() {
    let sp = mock_default_service_provider();

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let found_user = sp.user_repository().find(user.id).unwrap().unwrap();
    assert_eq!(user, found_user);
}
