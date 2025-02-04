use crate::tests::mock::mock_default_service_provider;

#[test]
fn test_find_by_user() {
    let sp = mock_default_service_provider();

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let _ = sp
        .specimen_service()
        .generate_and_save_specimen(&user, 1)
        .unwrap();
    let _ = sp
        .specimen_service()
        .generate_and_save_specimen(&user, 1)
        .unwrap();
    let _ = sp
        .specimen_service()
        .generate_and_save_specimen(&user, 1)
        .unwrap();
    let _ = sp
        .specimen_service()
        .generate_and_save_specimen(&user, 1)
        .unwrap();

    let all_user_specimen = sp.specimen_repository().find_by_user(&user).unwrap();
    assert_eq!(all_user_specimen.len(), 4);

    let first_specimen = all_user_specimen.first().unwrap();
    assert_eq!(first_specimen.species_id, 1);
    assert_eq!(first_specimen.user_id, user.id);
}

#[test]
fn test_find() {
    let sp = mock_default_service_provider();

    let user = sp.user_service().create_and_save_user(1337).unwrap();

    let specimen = sp
        .specimen_service()
        .generate_and_save_specimen(&user, 1)
        .unwrap();
    let found_specimen = sp.specimen_repository().find(specimen.id).unwrap().unwrap();
    assert_eq!(specimen, found_specimen);
}

#[test]
fn test_save() {
    let sp = mock_default_service_provider();

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let mut specimen = sp
        .specimen_service()
        .generate_and_save_specimen(&user, 1)
        .unwrap();
    specimen.species_id = 2;

    sp.specimen_repository().save(specimen.clone()).unwrap();
    let found_specimen = sp.specimen_repository().find(specimen.id).unwrap().unwrap();
    assert_eq!(found_specimen.species_id, 2);
    assert_eq!(specimen.created_at, found_specimen.created_at);
}

#[test]
fn test_delete() {
    let sp = mock_default_service_provider();

    let user = sp.user_service().create_and_save_user(1337).unwrap();
    let specimen = sp
        .specimen_service()
        .generate_and_save_specimen(&user, 1)
        .unwrap();

    let found_specimen = sp.specimen_repository().find(specimen.id).unwrap().unwrap();
    assert_eq!(specimen, found_specimen);

    sp.specimen_repository().delete(&found_specimen).unwrap();
    assert_eq!(
        sp.specimen_repository().find(found_specimen.id).unwrap(),
        None
    );
}
