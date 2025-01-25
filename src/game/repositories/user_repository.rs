use crate::get_db_connection;
use crate::models::user::{NewUser, User};
use crate::schema::users;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn add_user(external_id: i64) -> Result<User, Box<dyn std::error::Error>> {
    let new_user = NewUser { external_id };

    let mut connection = get_db_connection()?;
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut connection)?;

    let user = find_user_by_external_id(external_id)?;
    Ok(user)
}

pub fn find_user_by_external_id(external_id: i64) -> Result<User, Box<dyn std::error::Error>> {
    let mut connection = get_db_connection()?;
    let user = users::table
        .filter(users::external_id.eq(external_id))
        .first::<User>(&mut connection)?;
    Ok(user)
}
