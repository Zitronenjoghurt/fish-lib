use crate::data::location_data::LocationData;
use crate::get_config;
use std::collections::HashMap;
use std::sync::Arc;

pub struct LocationService;

impl LocationService {
    pub fn get_location_names() -> Arc<HashMap<i32, String>> {
        get_config().location_names.clone()
    }

    pub fn get_location_data(location_id: i32) -> Option<Arc<LocationData>> {
        get_config().locations.get(&location_id).cloned()
    }
}
