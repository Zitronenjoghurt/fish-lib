use crate::config::ConfigInterface;
use crate::data::location_data::LocationData;
use std::collections::HashMap;
use std::sync::Arc;

pub trait LocationServiceInterface: Send + Sync {
    fn get_location_names(&self) -> Arc<HashMap<i32, String>>;
    fn get_location_data(&self, location_id: i32) -> Option<Arc<LocationData>>;
}

pub struct LocationService {
    config: Arc<dyn ConfigInterface>,
}

impl LocationService {
    pub fn new(config: Arc<dyn ConfigInterface>) -> LocationService {
        LocationService { config }
    }
}

impl LocationServiceInterface for LocationService {
    fn get_location_names(&self) -> Arc<HashMap<i32, String>> {
        self.config.location_names().clone()
    }

    fn get_location_data(&self, location_id: i32) -> Option<Arc<LocationData>> {
        self.config.get_location_data(location_id)
    }
}
