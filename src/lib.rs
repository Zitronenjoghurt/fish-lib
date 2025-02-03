use crate::config::Config;
use crate::database::Database;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use lazy_static::lazy_static;
use std::path::Path;
use std::sync::{Arc, RwLock};

pub mod config;
pub mod data;
pub mod database;
pub mod enums;
pub mod game;
pub mod models;
pub mod schema;
#[cfg(test)]
pub mod tests;
pub mod traits;
pub mod utils;

lazy_static! {
    static ref DB: RwLock<Database> = RwLock::new(Database::new());
}

lazy_static! {
    static ref CONFIG: RwLock<Arc<Config>> = RwLock::new(Arc::new(Config::default()));
}

pub fn connect_db(postgres_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    DB.write()
        .expect("Failed to get write lock on DB")
        .connect(postgres_url)?;
    Ok(())
}

pub fn get_db_connection(
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Box<dyn std::error::Error>> {
    DB.read()
        .expect("Failed to get read lock on DB")
        .get_connection()
}

pub fn clear_db() -> Result<(), Box<dyn std::error::Error>> {
    DB.write().expect("Failed to get write lock on DB").clear()
}

pub fn get_config() -> Arc<Config> {
    CONFIG.read().unwrap().clone()
}

pub fn set_config(new_config: Config) {
    *CONFIG.write().unwrap() = Arc::new(new_config);
}

pub fn reset_config() {
    set_config(Config::default());
}

pub fn setup_test() {
    connect_db("postgresql://admin:root@db:5432/test_db").unwrap();
    clear_db().unwrap();

    let config = Config::builder()
        .species_json_file(Path::new("./example_data/species_data.json"))
        .unwrap()
        .build();
    set_config(config);
}
