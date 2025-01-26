use crate::game::repositories::fish_repository::FishRepository;
use crate::game::repositories::fishing_history_entry_repository::FishingHistoryEntryRepository;
use crate::game::repositories::user_repository::UserRepository;
use crate::game::services::fishing_history_service::FishingHistoryService;
use crate::models::fish::NewFish;
use crate::traits::repository::Repository;

#[test]
fn test_register_catch() {
    let user = UserRepository::create_from(1337).unwrap();
    let new_fish = NewFish {
        user_id: user.id,
        species_id: 1,
        size_baby_mm: 20.0,
        size_adult_mm: 20.0,
        lifespan_days: 1.0,
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
    assert_eq!(entry.smallest_catch_mm, 20.0);
    assert_eq!(entry.largest_catch_mm, 20.0);

    let found_entry =
        FishingHistoryEntryRepository::find_by_user_and_species_id(user.id, fish.species_id)
            .unwrap()
            .unwrap();
    assert_eq!(found_entry, entry);

    let new_fish2 = NewFish {
        user_id: user.id,
        species_id: 1,
        size_baby_mm: 30.0,
        size_adult_mm: 30.0,
        lifespan_days: 1.0,
        catch_age: 1.0,
    };
    let fish2 = FishRepository::create(new_fish2).unwrap();
    let entry2 = FishingHistoryService::register_catch(&fish2).unwrap();
    assert_eq!(entry2.user_id, user.id);
    assert_eq!(entry2.species_id, fish.species_id);
    assert_eq!(entry2.caught_count, 2);
    assert_eq!(entry2.sold_count, 0);
    assert_eq!(entry2.smallest_catch_mm, 20.0);
    assert_eq!(entry2.largest_catch_mm, 30.0);

    let found_entry2 =
        FishingHistoryEntryRepository::find_by_user_and_species_id(user.id, fish.species_id)
            .unwrap()
            .unwrap();
    assert_eq!(found_entry2, entry2);

    let new_fish3 = NewFish {
        user_id: user.id,
        species_id: 1,
        size_baby_mm: 10.0,
        size_adult_mm: 10.0,
        lifespan_days: 1.0,
        catch_age: 1.0,
    };
    let fish3 = FishRepository::create(new_fish3).unwrap();
    let entry3 = FishingHistoryService::register_catch(&fish3).unwrap();
    assert_eq!(entry3.user_id, user.id);
    assert_eq!(entry3.species_id, fish.species_id);
    assert_eq!(entry3.caught_count, 3);
    assert_eq!(entry3.sold_count, 0);
    assert_eq!(entry3.smallest_catch_mm, 10.0);
    assert_eq!(entry3.largest_catch_mm, 30.0);

    let found_entry3 =
        FishingHistoryEntryRepository::find_by_user_and_species_id(user.id, fish.species_id)
            .unwrap()
            .unwrap();
    assert_eq!(found_entry3, entry3);
}
