use diesel::{Insertable, Queryable, Selectable};

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::fish_fishes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Fish {
    pub id: i64,
    pub user_id: i64,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::fish_fishes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewFish {
    pub user_id: i64,
}
