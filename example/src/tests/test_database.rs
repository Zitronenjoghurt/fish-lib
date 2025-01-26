use crate::tests::setup_test;
use fish_lib::game::services::fish_service::FishService;
use fish_lib::game::services::user_service::UserService;

#[test]
fn test_database() {
    setup_test();

    let user = UserService::create_and_save_user(1337).unwrap();
    let fish = FishService::generate_and_save_fish(&user, 1).unwrap();

    assert_eq!(user.external_id, 1337);
    assert_eq!(fish.user_id, user.id);
}
