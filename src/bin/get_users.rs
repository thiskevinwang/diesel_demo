extern crate diesel;
extern crate rust_server;

// This brings `.filter()` and other methods into scope
use self::diesel::prelude::*;

// This enables `self::models::*;`
// This brings `establish_connection` into scope;
use self::rust_server::*;

// This brings `User` into scope
use self::models::*;

/// # get_users
/// This function fetches the first 10 Users
/// and prints them to the console.
///
/// ```bash
/// cargo run --bin get_users
/// ```
fn main() {
    use rust_server::schema::Users::dsl::*;
    let connection = establish_connection();

    let results = Users
        .limit(10)
        .load::<User>(&connection)
        .expect("Error loading users");

    for user in results {
        println!("{:?}", user)
    }
}
