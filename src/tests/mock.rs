use crate::config::{Config, ConfigBuilderInterface, ConfigInterface};
use crate::database::{Database, DatabaseInterface};
use crate::game::service_provider::{ServiceProvider, ServiceProviderInterface};
use std::env;
use std::path::Path;
use std::sync::{Arc, RwLock};

pub fn mock_default_db() -> Arc<RwLock<dyn DatabaseInterface>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = Database::create();
    db.write().unwrap().connect(&database_url).unwrap();
    db.read().unwrap().clear().unwrap();
    db
}

pub fn mock_default_config() -> Arc<dyn ConfigInterface> {
    Config::builder()
        .species_json_file(Path::new("./example_data/species_data.json"))
        .unwrap()
        .locations_json_file(Path::new("./example_data/locations.json"))
        .unwrap()
        .build()
        .unwrap()
}

pub fn mock_service_provider(
    config: Arc<dyn ConfigInterface>,
) -> Arc<dyn ServiceProviderInterface> {
    let db = mock_default_db();
    ServiceProvider::create(config, db)
}

pub fn mock_default_service_provider() -> Arc<dyn ServiceProviderInterface> {
    mock_service_provider(mock_default_config())
}
