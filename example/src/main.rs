use chrono::{Duration, Utc};
use chrono_tz::Tz;
use dotenv::dotenv;
use fish_lib::config::{Config, ConfigBuilderInterface, ConfigInterface};
use fish_lib::game::interface::GameInterface;
use fish_lib::game::service_provider::ServiceProviderInterface;
use fish_lib::game::Game;
use plotters::prelude::full_palette::GREY;
use plotters::prelude::*;
use std::env;
use std::path::Path;
use std::sync::Arc;

#[cfg(test)]
mod tests;

fn init_config() -> Arc<dyn ConfigInterface> {
    let species_json_file = Path::new("./../example_data/species_data.json");
    let locations_json_file = Path::new("./../example_data/locations.json");

    Config::builder()
        .species_json_file(species_json_file)
        .unwrap()
        .locations_json_file(locations_json_file)
        .unwrap()
        .build()
        .unwrap()
}

fn init_game() -> Game {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Game::new(&database_url, Some(init_config())).unwrap()
}

fn main() {
    let game = init_game();

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

    //let mut time = DateTime::from_timestamp(0, 0).unwrap();
    //let weather_service = WeatherService::get_instance();
    //for _ in 0..=1000 {
    //    let weather = weather_service.get_weather(1, time).unwrap();
    //    println!("{:?}", time);
    //    println!(
    //        "{:.2}°C | 💧 {:.2}% | ☀️ {:.2}% | ☁️ {:.2}% | Rain? {} ({:.2}%) {:?} ({:.2}%) | ({:.2} - {:.2}°C)",
    //        weather.temperature_c,
    //        weather.humidity * 100.0,
    //        weather.light_level * 100.0,
    //        weather.cloudiness * 100.0,
    //        weather.is_raining,
    //        weather.rain_strength * 100.0,
    //        weather.season,
    //        weather.season_progress * 100.0,
    //        weather.min_possible_temp_c,
    //        weather.max_possible_temp_c
    //    );
    //    time += Duration::hours(1);
    //}

    weather_plot(
        &game,
        Tz::Europe__Berlin,
        0,
        false,
        24000,
        true,
        -20.0,
        40.0,
    )
    .unwrap();

    //let mut time = Utc::now().with_timezone(&Tz::Europe__Berlin);
    //time -= Duration::hours(1);
    //println!("{:?}", WeatherService::get_instance().get_weather(1, time));
}

fn weather_plot(
    game: &Game,
    timezone: Tz,
    start_hour_offset: i64,
    by_days: bool,
    count: u32,
    only_temperature: bool,
    y_min: f32,
    y_max: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("weather_plot.png", (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut time = Utc::now().with_timezone(&timezone);
    time += Duration::hours(start_hour_offset);

    let mut temperatures = Vec::new();
    let mut humidity = Vec::new();
    let mut light = Vec::new();
    let mut cloudiness = Vec::new();
    let mut days = Vec::new();

    let location_data = game.location_find(1).unwrap();

    for day in 0..=count {
        let weather = game
            .weather_service()
            .get_weather(location_data.clone(), time)
            .unwrap();

        temperatures.push((day as f32, weather.temperature_c));
        humidity.push((day as f32, (weather.humidity * 100.0)));
        light.push((day as f32, (weather.light_level * 100.0)));
        cloudiness.push((day as f32, (weather.cloudiness * 100.0)));
        days.push(day as f32);

        if by_days {
            time += Duration::days(1);
        } else {
            time += Duration::hours(1);
        }
    }

    let mut chart = ChartBuilder::on(&root)
        .caption("Weather Metrics", ("sans-serif", 40))
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..count as f32, y_min..y_max)?;

    chart
        .configure_mesh()
        .x_desc(if by_days { "Days" } else { "Hours" })
        .y_desc("Values")
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            temperatures.iter().map(|&(x, y)| (x, y)),
            &RED,
        ))?
        .label("Temperature (°C)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    if !only_temperature {
        chart
            .draw_series(LineSeries::new(
                humidity.iter().map(|&(x, y)| (x, y)),
                &BLUE,
            ))?
            .label("Humidity (%)")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

        chart
            .draw_series(LineSeries::new(light.iter().map(|&(x, y)| (x, y)), &YELLOW))?
            .label("Light Level (%)")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], YELLOW));

        chart
            .draw_series(LineSeries::new(
                cloudiness.iter().map(|&(x, y)| (x, y)),
                &GREY,
            ))?
            .label("Cloudiness (%)")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREY));
    }

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    Ok(())
}
