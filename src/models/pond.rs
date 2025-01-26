use crate::traits::model::Model;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};

#[derive(Debug, Clone, PartialEq, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::fish_ponds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Pond {
    pub id: i64,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
