use crate::game::repositories::specimen_repository::SpecimenRepository;
use crate::game::services::specimen_service::SpecimenService;
use crate::game::services::user_service::UserService;
use crate::setup_test;
use crate::traits::repository::Repository;

#[test]
fn test_generate_and_save_specimen() {
    setup_test();

    let user = UserService::create_and_save_user(1337).unwrap();
    let specimen = SpecimenService::generate_and_save_specimen(&user, 1).unwrap();

    let found_specimen = SpecimenRepository::find(specimen.id).unwrap().unwrap();
    assert_eq!(found_specimen, specimen);
}

#[test]
fn test_process_catch() {
    setup_test();

    let user = UserService::create_and_save_user(1337).unwrap();
    let (specimen, entry) = SpecimenService::process_catch(&user, 1).unwrap();

    let user_specimens = SpecimenRepository::find_by_user(&user).unwrap();
    let first_specimen = user_specimens.first().unwrap();
    assert_eq!(*first_specimen, specimen);

    assert_eq!(entry.user_id, user.id);
    assert_eq!(entry.species_id, specimen.species_id);
    assert_eq!(entry.caught_count, 1);
    assert_eq!(entry.sold_count, 0);
}
