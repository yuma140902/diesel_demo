use std::env;

use diesel::prelude::*;
use diesel_demo::establish_connection;

fn main() {
    use diesel_demo::schema::posts::dsl::*;

    let target = env::args()
        .nth(1)
        .expect("Expected a target to match against.");
    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
