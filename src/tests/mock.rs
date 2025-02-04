use crate::config::{Config, ConfigBuilderInterface, ConfigInterface};
use crate::database::{Database, DatabaseInterface};
use crate::game::service_provider::{ServiceProvider, ServiceProviderInterface};
use std::path::Path;
use std::sync::{Arc, RwLock};

pub fn mock_default_db() -> Arc<RwLock<dyn DatabaseInterface>> {
    let db = Database::new();
    db.write()
        .unwrap()
        .connect("postgresql://admin:root@db:5432/test_db")
        .unwrap();
    db.read().unwrap().clear().unwrap();
    db
}

pub fn mock_default_config() -> Arc<dyn ConfigInterface> {
    Config::builder()
        .species_json_file(Path::new("./example_data/species_data.json"))
        .unwrap()
        .build()
}

pub fn mock_service_provider(
    config: Arc<dyn ConfigInterface>,
) -> Arc<dyn ServiceProviderInterface> {
    let db = mock_default_db();
    ServiceProvider::new(config, db)
}

pub fn mock_default_service_provider() -> Arc<dyn ServiceProviderInterface> {
    mock_service_provider(mock_default_config())
}
