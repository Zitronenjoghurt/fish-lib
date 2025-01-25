use crate::tests::setup_test;
use fish_lib::game::repositories::fish_repository::FishRepository;
use fish_lib::game::repositories::user_repository::UserRepository;

#[test]
fn test_database() {
    setup_test();

    let user = UserRepository::create_from(1337).unwrap();
    let fish = FishRepository::create_from(&user, 1).unwrap();

    assert_eq!(user.external_id, 1337);
    assert_eq!(fish.user_id, user.id);
}
