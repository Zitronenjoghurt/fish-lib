use crate::game::errors::GameResult;
use crate::game::repositories::pond_repository::PondRepositoryInterface;
use crate::models::pond::{NewPond, Pond};
use crate::models::user::User;
use std::sync::Arc;

pub trait PondServiceInterface: Send + Sync {
    fn create_and_save_pond(&self, owner_user: &User, capacity: i32) -> GameResult<Pond>;
}

pub struct PondService {
    pond_repository: Arc<dyn PondRepositoryInterface>,
}

impl PondService {
    pub fn new(pond_repository: Arc<dyn PondRepositoryInterface>) -> PondService {
        PondService { pond_repository }
    }
}

impl PondServiceInterface for PondService {
    fn create_and_save_pond(&self, owner_user: &User, capacity: i32) -> GameResult<Pond> {
        let pond = NewPond {
            user_id: owner_user.id,
            capacity,
        };
        Ok(self.pond_repository.create(pond)?)
    }
}
