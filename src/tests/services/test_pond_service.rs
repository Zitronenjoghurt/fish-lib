use crate::tests::mock::mock_default_service_provider;

#[test]
fn test_create_and_save_pond() {
    let sp = mock_default_service_provider();

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let pond = sp.pond_service().create_and_save_pond(&user, 50).unwrap();

    let found_pond = sp.pond_repository().find(pond.id).unwrap().unwrap();
    assert_eq!(pond, found_pond);
}
