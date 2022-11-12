use std::env;

use diesel::prelude::*;
use diesel::{sqlite::SqliteConnection, Connection};
use dotenvy::dotenv;
use models::Post;

use crate::models::NewPost;

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Error connecting to {} : {:?}", database_url, e))
}

pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) -> usize {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new posts")
}
