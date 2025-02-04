use crate::traits::model::Model;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Default, Serialize, Deserialize, Clone, PartialEq, Queryable, Selectable, AsChangeset,
)]
#[diesel(table_name = crate::schema::fish_ponds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Pond {
    /// Primary key of this pond in the database
    pub id: i64,
    /// The primary key of the user this pond belongs to
    pub user_id: i64,
    /// When the dataset was created
    pub created_at: DateTime<Utc>,
    /// When the dataset was last updated
    pub updated_at: DateTime<Utc>,
    /// How many specimen fit in this pond
    pub capacity: i32,
}

impl Model for Pond {
    type Table = crate::schema::fish_ponds::table;
    type PrimaryKeyType = i64;
    type InsertType = NewPond;

    fn table() -> Self::Table {
        crate::schema::fish_ponds::table
    }

    fn id(&self) -> Self::PrimaryKeyType {
        self.id
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::fish_ponds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPond {
    pub user_id: i64,
    pub capacity: i32,
}
