use crate::{clear_db, connect_db};

mod repositories;
mod test_config;

pub fn setup_test() {
    connect_db("postgresql://admin:root@db:5432/test_db").unwrap();
    clear_db().unwrap();
}
