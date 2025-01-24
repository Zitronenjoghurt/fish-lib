use crate::database::DatabaseEntity;

#[cfg(feature = "db-diesel")]
use crate::schema::fish::dsl::fish;

#[derive(Debug, Clone)]
pub struct Fish<ID> {
    pub id: ID,
}

impl<ID> DatabaseEntity for Fish<ID> {
    type ID = ID;

    #[cfg(feature = "db-diesel")]
    type Table = fish;

    fn get_id(&self) -> &Self::ID {
        &self.id
    }
}