use crate::tests::mock::mock_default_service_provider;

#[test]
fn test_generate_and_save_specimen() {
    let sp = mock_default_service_provider();

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let specimen = sp
        .specimen_service()
        .generate_and_save_specimen(&user, 1)
        .unwrap();

    let found_specimen = sp.specimen_repository().find(specimen.id).unwrap().unwrap();
    assert_eq!(found_specimen, specimen);
}

#[test]
fn test_process_catch() {
    let sp = mock_default_service_provider();

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let (specimen, entry) = sp.specimen_service().process_catch(&user, 1).unwrap();

    let user_specimens = sp.specimen_repository().find_by_user(&user).unwrap();
    let first_specimen = user_specimens.first().unwrap();
    assert_eq!(*first_specimen, specimen);

    assert_eq!(entry.user_id, user.id);
    assert_eq!(entry.species_id, specimen.species_id);
    assert_eq!(entry.caught_count, 1);
    assert_eq!(entry.sold_count, 0);
}
