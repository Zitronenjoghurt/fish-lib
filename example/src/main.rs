use chrono_tz::Tz;
use dotenv::dotenv;
use fish_lib::config::Config;
use fish_lib::game::repositories::user_repository::UserRepository;
use fish_lib::game::services::fish_service::FishService;
use fish_lib::game::services::user_service::UserService;
use fish_lib::traits::repository::Repository;
use fish_lib::{connect_db, set_config};
use std::env;
use std::path::Path;

#[cfg(test)]
mod tests;

fn init_config() {
    let fish_json_file = Path::new("./../example_data/fish_stats.json");

    let config = Config::builder()
        .fish_json_file(fish_json_file)
        .unwrap()
        .build();

    set_config(config);
}

fn init_db() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    connect_db(&database_url).unwrap();
}

fn main() {
    init_config();
    init_db();

    let user = UserService::create_and_save_user(1337).unwrap();
    let fish = FishService::generate_and_save_fish(&user, 1).unwrap();

    println!("{:?}", user);
    println!("{:?}", fish);

    let (fish, entry) = FishService::process_catch(&user, 1).unwrap();
    println!("Catching a fish:");
    println!("{:?}", fish);
    println!("{:?}", entry);

    let mut user = UserRepository::find_by_external_id(1337).unwrap().unwrap();
    user.set_timezone(Tz::Europe__Berlin);
    UserRepository::save(user).unwrap();
}
