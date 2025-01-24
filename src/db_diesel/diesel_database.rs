use crate::database::{Database, DatabaseEntity};
use std::error::Error;

pub struct DieselDatabase {}

impl Database for DieselDatabase {
    type ID = u32;

    fn find<E: DatabaseEntity<ID=Self::ID>>(&self, id: Self::ID) -> Result<E, Box<dyn Error>> {
        todo!()
    }

    fn save<E: DatabaseEntity<ID=Self::ID>>(&mut self, entity: E) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}