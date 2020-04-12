extern crate diesel;
extern crate rust_server;

use self::diesel::prelude::*;
use self::rust_server::*;
use self::models::Post;
use std::env::args;

fn main() {
    // import table columns here, like 'body' or 'published'
    use rust_server::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true)
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));
    println!("Published post {}", post.title);
}
