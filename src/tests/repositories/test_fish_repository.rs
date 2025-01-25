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

#[test]
fn test_find_by_user() {
    setup_test();
    let user = UserRepository::create_from(1337).unwrap();

    let _ = FishRepository::create_from(&user, 1).unwrap();
    let _ = FishRepository::create_from(&user, 1).unwrap();
    let _ = FishRepository::create_from(&user, 1).unwrap();
    let _ = FishRepository::create_from(&user, 1).unwrap();

    let all_user_fish = FishRepository::find_by_user(&user).unwrap();
    assert_eq!(all_user_fish.len(), 4);

    let first_fish = all_user_fish.first().unwrap();
    assert_eq!(first_fish.data_id, 1);
    assert_eq!(first_fish.user_id, user.id);
}

#[test]
fn test_save() {
    setup_test();
    let user = UserRepository::create_from(1337).unwrap();
    let mut fish = FishRepository::create_from(&user, 1).unwrap();
    fish.data_id = 2;

    FishRepository::save(fish.clone()).unwrap();
    let found_fish = FishRepository::find(fish.id).unwrap().unwrap();
    assert_eq!(found_fish.data_id, 2);
    assert_eq!(fish.created_at, found_fish.created_at);
}

#[test]
fn test_delete() {
    setup_test();
    let user = UserRepository::create_from(1337).unwrap();
    let fish = FishRepository::create_from(&user, 1).unwrap();

    let found_fish = FishRepository::find(fish.id).unwrap().unwrap();
    assert_eq!(fish, found_fish);

    FishRepository::delete(&found_fish).unwrap();
    assert_eq!(FishRepository::find(found_fish.id).unwrap(), None);
}
