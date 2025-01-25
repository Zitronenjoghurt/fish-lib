use crate::tests::setup_test;
use fish_lib::game::repositories::fish_repository::add_fish;
use fish_lib::game::repositories::user_repository::add_user;

#[test]
fn test_database() {
    setup_test();

    let user = add_user(1337).unwrap();
    let fish = add_fish(&user).unwrap();

    assert_eq!(user.external_id, 1337);
    assert_eq!(fish.user_id, user.id);
}
