use crate::game::repositories::pond_repository::PondRepository;
use crate::models::pond::{NewPond, Pond};
use crate::models::user::User;
use crate::traits::repository::Repository;
use std::error::Error;

pub struct PondService;

impl PondService {
    pub fn create_and_save_pond(owner_user: &User, capacity: i32) -> Result<Pond, Box<dyn Error>> {
        let pond = NewPond {
            user_id: owner_user.id,
            capacity,
        };
        PondRepository::create(pond)
    }
}
