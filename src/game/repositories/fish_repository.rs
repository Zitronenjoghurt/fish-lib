use crate::get_db_connection;
use crate::models::fish::{Fish, NewFish};
use crate::models::user::User;
use crate::schema::fish_fishes;
use crate::traits::repository::Repository;
use diesel::prelude::*;
use std::error::Error;

pub struct FishRepository;

impl FishRepository {
    pub fn create_from_user(user: &User) -> Result<Fish, Box<dyn Error>> {
        let fish = NewFish { user_id: user.id };
        Self::create(fish)
    }
}

impl Repository<Fish> for FishRepository {
    fn create(new_entity: NewFish) -> Result<Fish, Box<dyn Error>> {
        let mut connection = get_db_connection()?;

        let new_result = diesel::insert_into(fish_fishes::table)
            .values(new_entity)
            .get_result::<Fish>(&mut connection)?;

        Ok(new_result)
    }

    fn find(id: i64) -> Result<Option<Fish>, Box<dyn Error>> {
        let mut connection = get_db_connection()?;
        let user = fish_fishes::table
            .find(id)
            .first::<Fish>(&mut connection)
            .optional()?;
        Ok(user)
    }
}
