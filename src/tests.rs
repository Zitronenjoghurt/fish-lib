use crate::{clear_db, connect_db};

mod test_config;
mod test_database;

pub fn setup_test() {
    connect_db("postgresql://admin:root@db:5432/test_db").unwrap();
    clear_db().unwrap();
}
