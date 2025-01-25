use crate::get_db_connection;
use crate::models::fish::{Fish, NewFish};
use crate::models::user::User;
use diesel::RunQueryDsl;

pub fn add_fish(owner_user: &User) -> Result<Fish, Box<dyn std::error::Error>> {
    let new_fish = NewFish {
        user_id: owner_user.id,
    };

    let mut connection = get_db_connection()?;
    let fish = diesel::insert_into(crate::schema::fishes::table)
        .values(&new_fish)
        .get_result::<Fish>(&mut connection)?;

    Ok(fish)
}
