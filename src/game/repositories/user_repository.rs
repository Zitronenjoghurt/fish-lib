use crate::get_db_connection;
use crate::models::user::{NewUser, User};
use crate::schema::fish_users;
use crate::traits::repository::Repository;
use chrono::Utc;
use diesel::prelude::*;
use std::error::Error;

pub struct UserRepository;

impl UserRepository {
    pub fn find_by_external_id(external_id: i64) -> Result<Option<User>, Box<dyn Error>> {
        let mut connection = get_db_connection()?;
        let user = fish_users::table
            .filter(fish_users::external_id.eq(external_id))
            .first::<User>(&mut connection)
            .optional()?;
        Ok(user)
    }
}

impl Repository<User> for UserRepository {
    fn create(new_entity: NewUser) -> Result<User, Box<dyn Error>> {
        let mut connection = get_db_connection()?;

        let new_result = diesel::insert_into(fish_users::table)
            .values(new_entity)
            .get_result::<User>(&mut connection)?;

        Ok(new_result)
    }

    fn find(id: i64) -> Result<Option<User>, Box<dyn Error>> {
        let mut connection = get_db_connection()?;
        let user = fish_users::table
            .find(id)
            .first::<User>(&mut connection)
            .optional()?;
        Ok(user)
    }

    fn save(mut entity: User) -> Result<User, Box<dyn Error>> {
        let mut connection = get_db_connection()?;
        entity.updated_at = Utc::now();

        let updated_user = diesel::update(fish_users::table)
            .filter(fish_users::id.eq(entity.id))
            .set(entity)
            .get_result::<User>(&mut connection)?;

        Ok(updated_user)
    }

    fn delete(entity: &User) -> Result<bool, Box<dyn Error>> {
        let mut connection = get_db_connection()?;

        let deleted_count = diesel::delete(fish_users::table)
            .filter(fish_users::id.eq(entity.id))
            .execute(&mut connection)?;

        Ok(deleted_count > 0)
    }
}
