extern crate diesel;

use diesel::prelude::*;
use rust_db_diesel::*;
use rust_db_diesel::models::Post;
use std::env::args;

fn main() {
    use rust_db_diesel::schema::posts::dsl::{posts, published};

    let id = args().nth(1).expect("publish_post requires a post id")
        .parse::<i32>().expect("Invalid ID");
    let connection = establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .execute(&connection);
        // .get_result::<Post>(&connection)
    //     .expect(&format!("Unable to find post {}", id));
    println!("Published post {:?}", post);
}