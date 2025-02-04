use crate::tests::mock::mock_default_service_provider;
use chrono::Utc;
use chrono_tz::Tz;

#[test]
fn test_user_timezone() {
    let sp = mock_default_service_provider();

    let mut user = sp.user_service().create_and_save_user(1337).unwrap();
    assert_eq!(user.get_timezone(), Tz::UTC);

    user.set_timezone(Tz::Europe__Berlin);
    sp.user_repository().save(user).unwrap();

    let user = sp
        .user_repository()
        .find_by_external_id(1337)
        .unwrap()
        .unwrap();
    assert_eq!(user.get_timezone(), Tz::Europe__Berlin);

    let utc_now = Utc::now();
    let berlin_now = user.get_local_time();
    assert!(berlin_now > utc_now);
}
