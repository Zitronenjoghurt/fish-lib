use crate::game::repositories::user_repository::UserRepository;
use crate::tests::setup_test;
use crate::traits::repository::Repository;

#[test]
fn test_find() {
    setup_test();
    let user = UserRepository::create_from(1337).unwrap();

    let found_user = UserRepository::find(user.id).unwrap().unwrap();
    assert_eq!(user, found_user);
}

#[test]
fn test_find_by_external_id() {
    setup_test();
    let user = UserRepository::create_from(1337).unwrap();

    let found_user = UserRepository::find_by_external_id(1337).unwrap().unwrap();
    assert_eq!(user, found_user);
}
