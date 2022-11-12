use std::env::args;

use diesel::prelude::*;
use diesel_demo::{establish_connection, models, schema};

fn main() {
    use schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();

    diesel::update(posts.find(id))
        .set(published.eq(true))
        .execute(connection)
        .unwrap();

    let post: models::Post = posts.find(id).first(connection).unwrap();
    println!("Published post {}", post.title)
}
