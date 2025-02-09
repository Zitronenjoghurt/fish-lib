use crate::data::location_data::LocationData;
use crate::models::user_location::UserLocation;
use chrono::{DateTime, Utc};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct UserLocationUnlock {
    pub location: Arc<LocationData>,
    pub unlocked_at: DateTime<Utc>,
}

impl UserLocationUnlock {
    pub fn from_user_location(
        user_location: UserLocation,
        get_location: &impl Fn(i32) -> Option<Arc<LocationData>>,
    ) -> Option<Self> {
        get_location(user_location.location_id).map(|location| Self {
            location,
            unlocked_at: user_location.unlocked_at,
        })
    }

    pub fn from_user_locations(
        user_locations: Vec<UserLocation>,
        get_location: impl Fn(i32) -> Option<Arc<LocationData>>,
    ) -> Vec<Self> {
        user_locations
            .into_iter()
            .filter_map(|user_location| Self::from_user_location(user_location, &get_location))
            .collect()
    }
}
