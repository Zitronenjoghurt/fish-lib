use crate::database::DatabaseEntity;

#[cfg(feature = "db-diesel")]
use crate::schema::users::dsl::users;

#[derive(Debug, Clone)]
pub struct User<ID> {
    pub id: ID,
}

impl<ID> DatabaseEntity for User<ID> {
    type ID = ID;

    #[cfg(feature = "db-diesel")]
    type Table = users;

    fn get_id(&self) -> &Self::ID {
        &self.id
    }
}