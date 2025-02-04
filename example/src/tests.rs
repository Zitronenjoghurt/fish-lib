use crate::init_game;
use fish_lib::game::service_provider::ServiceProviderInterface;
use fish_lib::game::Game;

mod test_database;

pub fn mock_game() -> Game {
    let game = init_game();
    game.database().read().unwrap().clear().unwrap();
    game
}
