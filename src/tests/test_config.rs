use crate::config::Config;
use std::path::Path;

#[test]
fn test_building() {
    let fish_json_file = Path::new("./example_data/fish_stats.json");
    let settings_json_file = Path::new("./example_data/settings.json");

    let config = Config::builder()
        .fish_json_file(fish_json_file)
        .unwrap()
        .settings_json_file(settings_json_file)
        .unwrap()
        .build();

    assert_eq!(config.fish.get(&1).unwrap().name, "Salmon");
    assert_eq!(config.settings.time_speed_multiplier as u64, 1);
}
