use crate::get_db_connection;
use crate::models::pond::{NewPond, Pond};
use crate::models::user::User;
use crate::schema::fish_ponds;
use crate::traits::repository::Repository;
use chrono::Utc;
use diesel::prelude::*;
use std::error::Error;

pub struct PondRepository;

impl PondRepository {
    pub fn find_by_user(owner_user: &User) -> Result<Vec<Pond>, Box<dyn Error>> {
        let mut connection = get_db_connection()?;

        let pond = fish_ponds::table
            .filter(fish_ponds::user_id.eq(owner_user.id))
            .load::<Pond>(&mut connection)?;

        Ok(pond)
    }
}

impl Repository<Pond> for PondRepository {
    fn create(new_entity: NewPond) -> Result<Pond, Box<dyn Error>> {
        let mut connection = get_db_connection()?;

        let new_result = diesel::insert_into(fish_ponds::table)
            .values(new_entity)
            .get_result::<Pond>(&mut connection)?;

        Ok(new_result)
    }

    fn find(id: i64) -> Result<Option<Pond>, Box<dyn Error>> {
        let mut connection = get_db_connection()?;
        let pond = fish_ponds::table
            .find(id)
            .first::<Pond>(&mut connection)
            .optional()?;
        Ok(pond)
    }

    fn save(mut entity: Pond) -> Result<Pond, Box<dyn Error>> {
        let mut connection = get_db_connection()?;
        entity.updated_at = Utc::now();

        let updated_pond = diesel::update(fish_ponds::table)
            .filter(fish_ponds::id.eq(entity.id))
            .set(entity)
            .get_result::<Pond>(&mut connection)?;

        Ok(updated_pond)
    }

    fn delete(entity: &Pond) -> Result<bool, Box<dyn Error>> {
        let mut connection = get_db_connection()?;

        let deleted_count = diesel::delete(fish_ponds::table)
            .filter(fish_ponds::id.eq(entity.id))
            .execute(&mut connection)?;

        Ok(deleted_count > 0)
    }
}
