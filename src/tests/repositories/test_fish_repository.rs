use crate::game::repositories::fish_repository::FishRepository;
use crate::game::repositories::user_repository::UserRepository;
use crate::tests::setup_test;
use crate::traits::repository::Repository;

#[test]
fn test_find() {
    setup_test();
    let user = UserRepository::create_from(1337).unwrap();

    let fish = FishRepository::create_from(&user, 1).unwrap();
    let found_fish = FishRepository::find(fish.id).unwrap().unwrap();
    assert_eq!(fish, found_fish);
}
