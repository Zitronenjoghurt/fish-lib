use crate::game::repositories::specimen_repository::SpecimenRepository;
use crate::game::services::specimen_service::SpecimenService;
use crate::game::services::user_service::UserService;
use crate::setup_test;
use crate::traits::repository::Repository;

#[test]
fn test_find_by_user() {
    setup_test();
    let user = UserService::create_and_save_user(1337).unwrap();

    let _ = SpecimenService::generate_and_save_specimen(&user, 1).unwrap();
    let _ = SpecimenService::generate_and_save_specimen(&user, 1).unwrap();
    let _ = SpecimenService::generate_and_save_specimen(&user, 1).unwrap();
    let _ = SpecimenService::generate_and_save_specimen(&user, 1).unwrap();

    let all_user_specimen = SpecimenRepository::find_by_user(&user).unwrap();
    assert_eq!(all_user_specimen.len(), 4);

    let first_specimen = all_user_specimen.first().unwrap();
    assert_eq!(first_specimen.species_id, 1);
    assert_eq!(first_specimen.user_id, user.id);
}

#[test]
fn test_find() {
    setup_test();
    let user = UserService::create_and_save_user(1337).unwrap();

    let specimen = SpecimenService::generate_and_save_specimen(&user, 1).unwrap();
    let found_specimen = SpecimenRepository::find(specimen.id).unwrap().unwrap();
    assert_eq!(specimen, found_specimen);
}

#[test]
fn test_save() {
    setup_test();
    let user = UserService::create_and_save_user(1337).unwrap();
    let mut specimen = SpecimenService::generate_and_save_specimen(&user, 1).unwrap();
    specimen.species_id = 2;

    SpecimenRepository::save(specimen.clone()).unwrap();
    let found_specimen = SpecimenRepository::find(specimen.id).unwrap().unwrap();
    assert_eq!(found_specimen.species_id, 2);
    assert_eq!(specimen.created_at, found_specimen.created_at);
}

#[test]
fn test_delete() {
    setup_test();
    let user = UserService::create_and_save_user(1337).unwrap();
    let specimen = SpecimenService::generate_and_save_specimen(&user, 1).unwrap();

    let found_specimen = SpecimenRepository::find(specimen.id).unwrap().unwrap();
    assert_eq!(specimen, found_specimen);

    SpecimenRepository::delete(&found_specimen).unwrap();
    assert_eq!(SpecimenRepository::find(found_specimen.id).unwrap(), None);
}
