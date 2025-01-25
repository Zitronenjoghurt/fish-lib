use crate::traits::model::Model;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Debug, Clone, PartialEq, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::fish_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub external_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
