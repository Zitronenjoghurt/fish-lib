use crate::config::Config;
use std::path::Path;

#[test]
fn test_building() {
    let fish_json_file = Path::new("./example_data/fish_stats.json");

    let config = Config::builder()
        .fish_json_file(fish_json_file).unwrap()
        .build();

    assert_eq!(config.fish.get(&1).unwrap().name, "salmon");
}