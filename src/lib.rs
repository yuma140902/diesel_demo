use std::env;

use diesel::prelude::*;
use diesel::{sqlite::SqliteConnection, Connection};
use dotenvy::dotenv;

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Error connecting to {} : {:?}", database_url, e))
}
