use crate::game::repositories::fish_repository::FishRepository;
use crate::game::repositories::fishing_history_entry_repository::FishingHistoryEntryRepository;
use crate::game::services::fishing_history_service::FishingHistoryService;
use crate::game::services::user_service::UserService;
use crate::models::fish::NewFish;
use crate::tests::setup_test;
use crate::traits::repository::Repository;
use chrono::Utc;

#[test]
fn test_register_catch() {
    setup_test();

    let user = UserService::create_and_save_user(1337).unwrap();
    let new_fish = NewFish {
        user_id: user.id,
        species_id: 1,
        size_baby_ratio: 0.5,
        size_adult_ratio: 0.5,
        lifespan_days_ratio: 0.5,
        catch_age: 1.0,
    };
    let fish = FishRepository::create(new_fish).unwrap();

    let no_entry_yet =
        FishingHistoryEntryRepository::find_by_user_and_species_id(user.id, fish.species_id)
            .unwrap();
    assert_eq!(no_entry_yet, None);

    let entry = FishingHistoryService::register_catch(&fish).unwrap();
    assert_eq!(entry.user_id, user.id);
    assert_eq!(entry.species_id, fish.species_id);
    assert_eq!(entry.caught_count, 1);
    assert_eq!(entry.sold_count, 0);
    assert_eq!(entry.smallest_catch_size_ratio, 0.8913044f32);
    assert_eq!(entry.largest_catch_size_ratio, 0.8913044f32);
    assert_eq!(entry.last_catch, entry.created_at);

    let found_entry =
        FishingHistoryEntryRepository::find_by_user_and_species_id(user.id, fish.species_id)
            .unwrap()
            .unwrap();
    assert_eq!(found_entry, entry);

    let new_fish2 = NewFish {
        user_id: user.id,
        species_id: 1,
        size_baby_ratio: 0.75,
        size_adult_ratio: 0.75,
        lifespan_days_ratio: 0.5,
        catch_age: 1.0,
    };
    let fish2 = FishRepository::create(new_fish2).unwrap();
    let entry2 = FishingHistoryService::register_catch(&fish2).unwrap();
    assert_eq!(entry2.user_id, user.id);
    assert_eq!(entry2.species_id, fish.species_id);
    assert_eq!(entry2.caught_count, 2);
    assert_eq!(entry2.sold_count, 0);
    assert_eq!(entry2.smallest_catch_size_ratio, 0.8913044f32);
    assert_eq!(entry2.largest_catch_size_ratio, 0.9456522f32);
    assert_eq!(entry2.last_catch, fish2.created_at);

    let found_entry2 =
        FishingHistoryEntryRepository::find_by_user_and_species_id(user.id, fish.species_id)
            .unwrap()
            .unwrap();
    assert_eq!(found_entry2, entry2);

    let new_fish3 = NewFish {
        user_id: user.id,
        species_id: 1,
        size_baby_ratio: 0.25,
        size_adult_ratio: 0.25,
        lifespan_days_ratio: 0.5,
        catch_age: 1.0,
    };
    let fish3 = FishRepository::create(new_fish3).unwrap();
    let entry3 = FishingHistoryService::register_catch(&fish3).unwrap();
    assert_eq!(entry3.user_id, user.id);
    assert_eq!(entry3.species_id, fish.species_id);
    assert_eq!(entry3.caught_count, 3);
    assert_eq!(entry3.sold_count, 0);
    assert_eq!(entry3.smallest_catch_size_ratio, 0.8369565f32);
    assert_eq!(entry3.largest_catch_size_ratio, 0.9456522f32);
    assert_eq!(entry3.last_catch, fish3.created_at);

    let found_entry3 =
        FishingHistoryEntryRepository::find_by_user_and_species_id(user.id, fish.species_id)
            .unwrap()
            .unwrap();
    assert_eq!(found_entry3, entry3);
}

#[test]
fn test_register_sell() {
    setup_test();

    let user = UserService::create_and_save_user(1337).unwrap();
    let new_fish = NewFish {
        user_id: user.id,
        species_id: 1,
        size_baby_ratio: 0.5,
        size_adult_ratio: 0.5,
        lifespan_days_ratio: 0.5,
        catch_age: 1.0,
    };
    let fish = FishRepository::create(new_fish).unwrap();

    let sell_time = Utc::now();
    let result = FishingHistoryService::register_sell(&fish, sell_time);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        format!(
            "Can't register selling of a fish that wasn't caught yet. user_id: {}, species_id: {}",
            user.id, fish.species_id
        )
    );

    FishingHistoryService::register_catch(&fish).unwrap();
    FishingHistoryService::register_sell(&fish, sell_time).unwrap();

    let entry =
        FishingHistoryEntryRepository::find_by_user_and_species_id(user.id, fish.species_id)
            .unwrap()
            .unwrap();
    assert_eq!(entry.user_id, user.id);
    assert_eq!(entry.species_id, fish.species_id);
    assert_eq!(entry.caught_count, 1);
    assert_eq!(entry.sold_count, 1);
    assert_eq!(entry.smallest_catch_size_ratio, 0.8913044f32);
    assert_eq!(entry.largest_catch_size_ratio, 0.8913044f32);
    assert_eq!(entry.last_catch, entry.created_at);
    assert_eq!(entry.first_sell, entry.last_sell);

    let sell_time2 = Utc::now();
    let entry2 = FishingHistoryService::register_sell(&fish, sell_time2).unwrap();
    assert_eq!(entry2.user_id, user.id);
    assert_eq!(entry2.species_id, fish.species_id);
    assert_eq!(entry2.caught_count, 1);
    assert_eq!(entry2.sold_count, 2);
    assert_eq!(entry2.smallest_catch_size_ratio, 0.8913044f32);
    assert_eq!(entry2.largest_catch_size_ratio, 0.8913044f32);
    assert_eq!(entry2.last_catch, entry.created_at);
    assert_eq!(entry2.first_sell, entry.first_sell);
    assert_ne!(entry2.last_sell, entry.first_sell);
    assert!(entry2.first_sell < entry2.last_sell);
}
