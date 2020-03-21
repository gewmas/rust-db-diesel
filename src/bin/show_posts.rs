extern crate diesel;

use self::diesel::prelude::*;
use rust_db_diesel::models::*;
use rust_db_diesel::*;

fn main() {
    use rust_db_diesel::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{:?}:{}", post.id, post.title);
        println!("{} published:{}", post.body, post.published);
        println!("----------\n");
    }
}
