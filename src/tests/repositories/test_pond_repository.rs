use crate::game::repositories::pond_repository::PondRepository;
use crate::game::repositories::user_repository::UserRepository;
use crate::tests::setup_test;
use crate::traits::repository::Repository;

#[test]
fn test_find_by_user() {
    setup_test();
    let user = UserRepository::create_from(1337).unwrap();

    let _ = PondRepository::create_from(&user, 50).unwrap();
    let _ = PondRepository::create_from(&user, 50).unwrap();
    let _ = PondRepository::create_from(&user, 50).unwrap();
    let _ = PondRepository::create_from(&user, 50).unwrap();

    let all_user_ponds = PondRepository::find_by_user(&user).unwrap();
    assert_eq!(all_user_ponds.len(), 4);

    let first_pond = all_user_ponds.first().unwrap();
    assert_eq!(first_pond.capacity, 50);
    assert_eq!(first_pond.user_id, user.id);
}

#[test]
fn test_find() {
    setup_test();
    let user = UserRepository::create_from(1337).unwrap();

    let pond = PondRepository::create_from(&user, 50).unwrap();
    let found_pond = PondRepository::find(pond.id).unwrap().unwrap();
    assert_eq!(pond, found_pond);
}

#[test]
fn test_save() {
    setup_test();
    let user = UserRepository::create_from(1337).unwrap();
    let mut pond = PondRepository::create_from(&user, 50).unwrap();
    pond.capacity = 51;

    PondRepository::save(pond.clone()).unwrap();
    let found_pond = PondRepository::find(pond.id).unwrap().unwrap();
    assert_eq!(found_pond.capacity, 51);
    assert_eq!(pond.created_at, found_pond.created_at);
}

#[test]
fn test_delete() {
    setup_test();
    let user = UserRepository::create_from(1337).unwrap();
    let pond = PondRepository::create_from(&user, 50).unwrap();

    let found_pond = PondRepository::find(pond.id).unwrap().unwrap();
    assert_eq!(pond, found_pond);

    PondRepository::delete(&found_pond).unwrap();
    assert_eq!(PondRepository::find(found_pond.id).unwrap(), None);
}
