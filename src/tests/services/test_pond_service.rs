use crate::game::repositories::pond_repository::PondRepository;
use crate::game::services::pond_service::PondService;
use crate::game::services::user_service::UserService;
use crate::setup_test;
use crate::traits::repository::Repository;

#[test]
fn test_create_and_save_pond() {
    setup_test();

    let user = UserService::create_and_save_user(1337).unwrap();
    let pond = PondService::create_and_save_pond(&user, 50).unwrap();

    let found_pond = PondRepository::find(pond.id).unwrap().unwrap();
    assert_eq!(pond, found_pond);
}
