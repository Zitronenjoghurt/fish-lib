use crate::game::repositories::fishing_history_entry_repository::FishingHistoryEntryRepository;
use crate::game::services::user_service::UserService;
use crate::models::fishing_history_entry::{FishingHistoryEntry, NewFishingHistoryEntry};
use crate::models::user::User;
use crate::tests::setup_test;
use crate::traits::repository::Repository;

fn new_user_and_entry() -> (User, FishingHistoryEntry) {
    let user = UserService::create_and_save_user(1337).unwrap();
    let new_entry = NewFishingHistoryEntry {
        user_id: user.id,
        species_id: 1,
        caught_count: 1,
        sold_count: 0,
        smallest_catch_size_ratio: 0.5,
        largest_catch_size_ratio: 0.75,
    };
    let entry = FishingHistoryEntryRepository::create(new_entry).unwrap();
    (user, entry)
}

#[test]
fn test_find_by_user_and_species_id() {
    setup_test();
    let (user, entry) = new_user_and_entry();
    let found_entry =
        FishingHistoryEntryRepository::find_by_user_and_species_id(user.id, entry.species_id)
            .unwrap()
            .unwrap();
    assert_eq!(found_entry, entry);
}

#[test]
fn test_find() {
    setup_test();
    let (_, entry) = new_user_and_entry();
    let found_entry = FishingHistoryEntryRepository::find(entry.id)
        .unwrap()
        .unwrap();
    assert_eq!(entry, found_entry);
}

#[test]
fn test_save() {
    setup_test();
    let (_, mut entry) = new_user_and_entry();
    entry.sold_count = 69;

    FishingHistoryEntryRepository::save(entry.clone()).unwrap();
    let found_entry = FishingHistoryEntryRepository::find(entry.id)
        .unwrap()
        .unwrap();
    assert_eq!(found_entry.sold_count, 69);
    assert_eq!(entry.created_at, found_entry.created_at);
}

#[test]
fn test_delete() {
    setup_test();
    let (_, entry) = new_user_and_entry();

    let found_entry = FishingHistoryEntryRepository::find(entry.id)
        .unwrap()
        .unwrap();
    assert_eq!(entry, found_entry);

    FishingHistoryEntryRepository::delete(&found_entry).unwrap();
    assert_eq!(
        FishingHistoryEntryRepository::find(found_entry.id).unwrap(),
        None
    );
}
