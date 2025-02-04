use crate::game::errors::GameResult;
use crate::game::repositories::user_repository::UserRepositoryInterface;
use crate::models::user::{NewUser, User};
use std::sync::Arc;

pub trait UserServiceInterface: Send + Sync {
    fn create_and_save_user(&self, external_id: i64) -> GameResult<User>;
}

pub struct UserService {
    user_repository: Arc<dyn UserRepositoryInterface>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepositoryInterface>) -> UserService {
        UserService { user_repository }
    }
}

impl UserServiceInterface for UserService {
    fn create_and_save_user(&self, external_id: i64) -> GameResult<User> {
        let user = NewUser { external_id };
        Ok(self.user_repository.create(user)?)
    }
}
