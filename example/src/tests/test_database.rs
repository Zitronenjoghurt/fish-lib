use crate::tests::mock_game;
use fish_lib::game::service_provider::ServiceProviderInterface;

#[test]
fn test_database() {
    let game = mock_game();

    let user = game.user_service().create_and_save_user(1337).unwrap();
    let fish = game
        .specimen_service()
        .generate_and_save_specimen(&user, 1)
        .unwrap();

    assert_eq!(user.external_id, 1337);
    assert_eq!(fish.user_id, user.id);
}
