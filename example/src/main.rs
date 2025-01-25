use dotenv::dotenv;
use fish_lib::config::Config;
use fish_lib::game::repositories::fish_repository::FishRepository;
use fish_lib::game::repositories::user_repository::UserRepository;
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

    let user = UserRepository::create_from(1337).unwrap();
    let fish = FishRepository::create_from(&user, 1).unwrap();

    println!("{:?}", user);
    println!("{:?}", fish);
}
