use crate::traits::model::Model;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(
    Debug, Default, Serialize, Deserialize, Clone, PartialEq, Queryable, Selectable, AsChangeset,
)]
#[diesel(table_name = crate::schema::fish_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    /// Primary key of this user in the database
    pub id: i64,
    /// An ID which identifies this user in the external system
    pub external_id: i64,
    /// How much of the currency this user has
    pub credits: i64,
    /// When the dataset was created
    pub created_at: DateTime<Utc>,
    /// When the dataset was last updated
    pub updated_at: DateTime<Utc>,
    /// The Timezone of this user, defaults to UTC
    pub timezone: String,
}

impl User {
    pub fn get_timezone(&self) -> Tz {
        Tz::from_str(&self.timezone).unwrap()
    }

    pub fn set_timezone(&mut self, timezone: Tz) {
        self.timezone = timezone.to_string();
    }

    pub fn get_local_time(&self) -> DateTime<Tz> {
        Utc::now().with_timezone(&self.get_timezone())
    }
}

impl Model for User {
    type Table = crate::schema::fish_users::table;
    type PrimaryKeyType = i64;
    type InsertType = NewUser;

    fn table() -> Self::Table {
        crate::schema::fish_users::table
    }

    fn id(&self) -> Self::PrimaryKeyType {
        self.id
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::fish_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub external_id: i64,
}
