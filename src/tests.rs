use crate::config::Config;
use crate::{clear_db, connect_db, set_config};
use std::path::Path;

mod models;
mod repositories;
mod services;
mod test_config;

pub fn setup_test() {
    connect_db("postgresql://admin:root@db:5432/test_db").unwrap();
    clear_db().unwrap();

    let config = Config::builder()
        .fish_json_file(Path::new("./example_data/fish_stats.json"))
        .unwrap()
        .build();
    set_config(config);
}
