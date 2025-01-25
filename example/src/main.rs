use dotenv::dotenv;
use fish_lib::connect_db;
use fish_lib::game::repositories::fish_repository::add_fish;
use fish_lib::game::repositories::user_repository::add_user;
use std::env;

#[cfg(test)]
mod tests;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    connect_db(&database_url).unwrap();

    let new_user = add_user(1337).unwrap();
    add_fish(&new_user).unwrap();
}
