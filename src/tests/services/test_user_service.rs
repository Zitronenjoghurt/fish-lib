use crate::game::repositories::user_repository::UserRepository;
use crate::game::services::user_service::UserService;
use crate::setup_test;
use crate::traits::repository::Repository;

#[test]
fn test_create_and_save_user() {
    setup_test();

    let user = UserService::create_and_save_user(1337).unwrap();
    let found_user = UserRepository::find(user.id).unwrap().unwrap();
    assert_eq!(user, found_user);
}
