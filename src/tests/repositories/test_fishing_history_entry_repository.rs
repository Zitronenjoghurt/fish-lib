use crate::game::service_provider::ServiceProviderInterface;
use crate::models::fishing_history_entry::{FishingHistoryEntry, NewFishingHistoryEntry};
use crate::models::user::User;
use crate::tests::mock::mock_default_service_provider;
use std::sync::Arc;

fn new_user_and_entry(sp: &Arc<dyn ServiceProviderInterface>) -> (User, FishingHistoryEntry) {
    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let new_entry = NewFishingHistoryEntry {
        user_id: user.id,
        species_id: 1,
        caught_count: 1,
        sold_count: 0,
        smallest_catch_size_ratio: 0.5,
        largest_catch_size_ratio: 0.75,
    };
    let entry = sp
        .fishing_history_entry_repository()
        .create(new_entry)
        .unwrap();
    (user, entry)
}

#[test]
fn test_find_by_user_and_species_id() {
    let sp = mock_default_service_provider();

    let (user, entry) = new_user_and_entry(&sp);
    let found_entry = sp
        .fishing_history_entry_repository()
        .find_by_user_and_species_id(user.id, entry.species_id)
        .unwrap()
        .unwrap();
    assert_eq!(found_entry, entry);
}

#[test]
fn test_find_caught_species_ids_by_user() {
    let sp = mock_default_service_provider();

    let (user, entry) = new_user_and_entry(&sp);
    let caught_species_ids = sp
        .fishing_history_entry_repository()
        .find_caught_species_ids_by_user(user.id)
        .unwrap();
    assert_eq!(caught_species_ids.len(), 1);
    assert_eq!(caught_species_ids[0], entry.species_id);
}

#[test]
fn test_find() {
    let sp = mock_default_service_provider();

    let (_, entry) = new_user_and_entry(&sp);
    let found_entry = sp
        .fishing_history_entry_repository()
        .find(entry.id)
        .unwrap()
        .unwrap();
    assert_eq!(entry, found_entry);
}

#[test]
fn test_save() {
    let sp = mock_default_service_provider();

    let (_, mut entry) = new_user_and_entry(&sp);
    entry.sold_count = 69;

    sp.fishing_history_entry_repository()
        .save(entry.clone())
        .unwrap();
    let found_entry = sp
        .fishing_history_entry_repository()
        .find(entry.id)
        .unwrap()
        .unwrap();
    assert_eq!(found_entry.sold_count, 69);
    assert_eq!(entry.created_at, found_entry.created_at);
}

#[test]
fn test_delete() {
    let sp = mock_default_service_provider();

    let (_, entry) = new_user_and_entry(&sp);

    let found_entry = sp
        .fishing_history_entry_repository()
        .find(entry.id)
        .unwrap()
        .unwrap();
    assert_eq!(entry, found_entry);

    let found_entry_id = found_entry.id;
    sp.fishing_history_entry_repository()
        .delete(found_entry)
        .unwrap();
    assert_eq!(
        sp.fishing_history_entry_repository()
            .find(found_entry_id)
            .unwrap(),
        None
    );
}
