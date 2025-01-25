use crate::{init_config, init_db};
use fish_lib::clear_db;

mod test_database;

pub fn setup_test() {
    init_config();
    init_db();
    clear_db().unwrap();
}
