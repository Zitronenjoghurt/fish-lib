use crate::config::Config;
use crate::entities::fish_stats::FishStats;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub mod config;
mod entities;

#[cfg(test)]
mod tests;
pub mod models;
mod database;
mod game;

#[cfg(feature = "db-diesel")]
pub mod db_diesel;

#[cfg(feature = "db-diesel")]
pub mod schema;

lazy_static! {
    static ref CONFIG: RwLock<Arc<Config>> = RwLock::new(Arc::new(Config::default()));
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

pub fn config_set_fish(fish: HashMap<u32, FishStats>) -> Arc<Config> {
    set_config((*get_config()).clone().with_fish(fish));
    get_config()
}