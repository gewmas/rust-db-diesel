pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use models::{Post, NewPost};

use diesel::sqlite::SqliteConnection;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &SqliteConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    let inserted_rows = diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn);
        // .get_results(conn);
        // .expect("Error saving new post")
    
    println!("inserted rows {:?}", inserted_rows);

    Post { id: Option::Some(1), body: "Hello".to_string(), title: "Hi".to_string(), published: true }
}
