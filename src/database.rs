#[cfg(feature = "db-diesel")]
pub use diesel;
use std::error::Error;

pub trait DatabaseEntity {
    type ID;
    fn get_id(&self) -> &Self::ID;

    #[cfg(feature = "db-diesel")]
    type Table: diesel::Table;
}

pub trait Database {
    type ID;

    fn find<E: DatabaseEntity<ID=Self::ID>>(&self, id: Self::ID) -> Result<E, Box<dyn Error>>;
    fn save<E: DatabaseEntity<ID=Self::ID>>(&mut self, entity: E) -> Result<(), Box<dyn Error>>;
}