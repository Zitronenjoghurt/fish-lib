use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Debug, Clone, PartialEq, Queryable)]
#[diesel(table_name = crate::schema::fish_user_locations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserLocation {
    pub user_id: i64,
    pub location_id: i32,
    pub unlocked_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::fish_user_locations)]
pub struct NewUserLocation {
    pub user_id: i64,
    pub location_id: i32,
}
