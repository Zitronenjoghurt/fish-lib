use crate::models::fish::{Fish, NewFish};
use crate::models::user::User;
use crate::schema::fish_fishes;
use crate::traits::repository::Repository;
use crate::get_db_connection;
use chrono::Utc;
use diesel::prelude::*;
use std::error::Error;

pub struct FishRepository;

impl FishRepository {
    pub fn create_from(owner_user: &User, species_id: i32) -> Result<Fish, Box<dyn Error>> {
        let fish = NewFish::generate(owner_user.id, species_id)?;
        Self::create(fish)
    }

    pub fn find_by_user(owner_user: &User) -> Result<Vec<Fish>, Box<dyn Error>> {
        let mut connection = get_db_connection()?;

        let fish = fish_fishes::table
            .filter(fish_fishes::user_id.eq(owner_user.id))
            .load::<Fish>(&mut connection)?;

        Ok(fish)
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
        let fish = fish_fishes::table
            .find(id)
            .first::<Fish>(&mut connection)
            .optional()?;
        Ok(fish)
    }

    fn save(mut entity: Fish) -> Result<Fish, Box<dyn Error>> {
        let mut connection = get_db_connection()?;
        entity.updated_at = Utc::now();

        let updated_fish = diesel::update(fish_fishes::table)
            .filter(fish_fishes::id.eq(entity.id))
            .set(entity)
            .get_result::<Fish>(&mut connection)?;

        Ok(updated_fish)
    }

    fn delete(entity: &Fish) -> Result<bool, Box<dyn Error>> {
        let mut connection = get_db_connection()?;

        let deleted_count = diesel::delete(fish_fishes::table)
            .filter(fish_fishes::id.eq(entity.id))
            .execute(&mut connection)?;

        Ok(deleted_count > 0)
    }
}
