use crate::game::repositories::user_repository::UserRepository;
use crate::tests::setup_test;
use crate::traits::repository::Repository;

#[test]
fn test_find_by_external_id() {
    setup_test();
    let user = UserRepository::create_from(1337).unwrap();

    let found_user = UserRepository::find_by_external_id(1337).unwrap().unwrap();
    assert_eq!(user, found_user);
}

#[test]
fn test_find() {
    setup_test();
    let user = UserRepository::create_from(1337).unwrap();

    let found_user = UserRepository::find(user.id).unwrap().unwrap();
    assert_eq!(user, found_user);
}

#[test]
fn test_save() {
    setup_test();
    let mut user = UserRepository::create_from(1337).unwrap();
    user.external_id = 1338;

    UserRepository::save(user.clone()).unwrap();
    assert_eq!(UserRepository::find_by_external_id(1337).unwrap(), None);
    let found_user = UserRepository::find_by_external_id(1338).unwrap().unwrap();
    assert_eq!(user.created_at, found_user.created_at);
}

#[test]
fn test_delete() {
    setup_test();
    let user = UserRepository::create_from(1337).unwrap();

    let found_user = UserRepository::find(user.id).unwrap().unwrap();
    assert_eq!(user, found_user);

    UserRepository::delete(&found_user).unwrap();
    assert_eq!(UserRepository::find(found_user.id).unwrap(), None);
}
