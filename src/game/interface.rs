use crate::game::errors::GameResult;
use crate::models::user::User;

pub trait GameInterface {
    fn register_user(external_id: i64) -> GameResult<User>;
    fn find_user(external_id: i64) -> GameResult<User>;
}
