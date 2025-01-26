use crate::game::repositories::user_repository::UserRepository;
use crate::models::user::{NewUser, User};
use crate::traits::repository::Repository;
use std::error::Error;

pub struct UserService;

impl UserService {
    pub fn create_and_save_user(external_id: i64) -> Result<User, Box<dyn Error>> {
        let user = NewUser { external_id };
        UserRepository::create(user)
    }
}
