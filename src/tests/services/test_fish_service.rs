use crate::game::repositories::fish_repository::FishRepository;
use crate::game::services::fish_service::FishService;
use crate::game::services::user_service::UserService;
use crate::setup_test;
use crate::traits::repository::Repository;

#[test]
fn test_generate_and_save_fish() {
    setup_test();

    let user = UserService::create_and_save_user(1337).unwrap();
    let fish = FishService::generate_and_save_fish(&user, 1).unwrap();

    let found_fish = FishRepository::find(fish.id).unwrap().unwrap();
    assert_eq!(found_fish, fish);
}

#[test]
fn test_process_catch() {
    setup_test();

    let user = UserService::create_and_save_user(1337).unwrap();
    let (fish, entry) = FishService::process_catch(&user, 1).unwrap();

    let user_fishes = FishRepository::find_by_user(&user).unwrap();
    let first_fish = user_fishes.first().unwrap();
    assert_eq!(*first_fish, fish);

    assert_eq!(entry.user_id, user.id);
    assert_eq!(entry.species_id, fish.species_id);
    assert_eq!(entry.caught_count, 1);
    assert_eq!(entry.sold_count, 0);
}
