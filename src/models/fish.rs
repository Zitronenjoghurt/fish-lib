use crate::traits::model::Model;
use diesel::{Insertable, Queryable, Selectable};

#[derive(Debug, Clone, PartialEq, Queryable, Selectable)]
#[diesel(table_name = crate::schema::fish_fishes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Fish {
    pub id: i64,
    pub user_id: i64,
    pub data_id: i32,
}

impl Model for Fish {
    type Table = crate::schema::fish_fishes::table;
    type PrimaryKeyType = i64;
    type InsertType = NewFish;

    fn table() -> Self::Table {
        crate::schema::fish_fishes::table
    }

    fn id(&self) -> Self::PrimaryKeyType {
        self.id
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::fish_fishes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewFish {
    pub user_id: i64,
    pub data_id: i32,
}
