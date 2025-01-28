use chrono::{DateTime, Duration};
use dotenv::dotenv;
use fish_lib::config::Config;
use fish_lib::game::services::weather_service::WeatherService;
use fish_lib::{connect_db, set_config};
use std::env;
use std::path::Path;

#[cfg(test)]
mod tests;

fn init_config() {
    let fish_json_file = Path::new("./../example_data/fish_stats.json");
    let locations_json_file = Path::new("./../example_data/locations.json");

    let config = Config::builder()
        .fish_json_file(fish_json_file)
        .unwrap()
        .locations_json_file(locations_json_file)
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

    //let user = UserService::create_and_save_user(1337).unwrap();
    //let fish = FishService::generate_and_save_fish(&user, 1).unwrap();
    //
    //println!("{:?}", user);
    //println!("{:?}", fish);
    //
    //let (fish, entry) = FishService::process_catch(&user, 1).unwrap();
    //println!("Catching a fish:");
    //println!("{:?}", fish);
    //println!("{:?}", entry);
    //
    //let mut user = UserRepository::find_by_external_id(1337).unwrap().unwrap();
    //user.set_timezone(Tz::Europe__Berlin);
    //UserRepository::save(user).unwrap();

    //let mut time = Utc::now();
    //let weather_system = WeatherSystem::new(1337, WeatherSystemConfig::default());
    //for _ in 0..=1000 {
    //    let attributes = weather_system.get_weather_attributes(time);
    //    println!("{:?}", time);
    //    println!("Cloudiness: {:?}", attributes.cloudiness_scale());
    //    println!(
    //        "Cloud brightness: {:?}",
    //        attributes.cloud_brightness_scale()
    //    );
    //    println!("Moisture: {:?}", attributes.moisture_scale());
    //    println!("Wind presence: {:?}", attributes.wind_presence_scale());
    //    println!("Wind Strength: {:?}", attributes.wind_strength_scale());
    //    println!("Temperature: {:?}", attributes.temperature_scale());
    //    println!("Light: {:?}", attributes.light_scale());
    //    time += Duration::days(1);
    //}

    let mut time = DateTime::from_timestamp(0, 0).unwrap();
    let weather_service = WeatherService::get_instance();
    for _ in 0..=1000 {
        let weather = weather_service.get_weather(1, time).unwrap();
        println!("{:?}", time);
        println!(
            "{:.2}°C | 💧 {:.2}% | ☀️ {:.2}% | ☁️ {:.2}% | {:?} ({:.2}%) | ({:.2} - {:.2}°C)",
            weather.temperature_c,
            weather.humidity * 100.0,
            weather.light_level * 100.0,
            weather.cloudiness * 100.0,
            weather.season,
            weather.season_progress * 100.0,
            weather.min_possible_temp_c,
            weather.max_possible_temp_c
        );
        time += Duration::days(1);
    }
}
