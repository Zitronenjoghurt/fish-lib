use crate::game::repositories::user_repository::UserRepository;
use crate::game::services::user_service::UserService;
use crate::setup_test;
use crate::traits::repository::Repository;
use chrono::Utc;
use chrono_tz::Tz;

#[test]
fn test_user_timezone() {
    setup_test();

    let mut user = UserService::create_and_save_user(1337).unwrap();
    assert_eq!(user.get_timezone(), Tz::UTC);

    user.set_timezone(Tz::Europe__Berlin);
    UserRepository::save(user).unwrap();

    let user = UserRepository::find_by_external_id(1337).unwrap().unwrap();
    assert_eq!(user.get_timezone(), Tz::Europe__Berlin);

    let utc_now = Utc::now();
    let berlin_now = user.get_local_time();
    assert!(berlin_now > utc_now);
}
