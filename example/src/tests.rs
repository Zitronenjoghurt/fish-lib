use dotenv::dotenv;
use fish_lib::{clear_db, connect_db};
use std::env;

mod test_database;

pub fn setup_test() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    connect_db(&database_url).unwrap();
    clear_db().unwrap();
}
