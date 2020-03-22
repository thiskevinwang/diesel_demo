// this enables `use self::diesel::prelud::*;`
extern crate diesel;
// this enables `use self::diesel_demo::*;`
extern crate diesel_demo;

// this brings `.filter()` & `.eq()` into scope
use self::diesel::prelude::*;
// this enables `use self::models::*;`
// this also brings `establish_connection()` into scope
use self::diesel_demo::*;
// this brings `Post` into scope
use self::models::*;

/// run with `cargo run --bin show_posts`
fn main() {
    /**
     * This line imports a bunch of aliases to that we
     * can say **posts** instead of **posts::table**,
     * and **published** instead of **posts::published**.
     *
     * It's useful when we're only dealing with a single
     * table, but that's not always what we want.
     */
    // hover over diesel_demo
    // - displays 'src/lib.rs'
    use diesel_demo::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
