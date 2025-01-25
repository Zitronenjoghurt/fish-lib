use diesel::prelude::*;

pub trait Model: Sized {
    type Table: Table;
    type PrimaryKeyType: Copy;
    type InsertType;

    fn table() -> Self::Table;

    fn id(&self) -> Self::PrimaryKeyType;
}
