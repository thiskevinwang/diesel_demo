extern crate diesel;
extern crate rust_server;

// This brings `.filter()` and other methods into scope
use self::diesel::prelude::*;

// This enables `self::models::*;`
// This brings `establish_connection` into scope;
use self::rust_server::*;

// This brings `User` into scope
use self::models::*;

/// cargo run --bin get_user_1
fn main() {
    use rust_server::schema::Users::dsl::*;

    let connection = establish_connection();

    // note:
    // User is the model
    // Users is the schema (use this)

    // add `.unwrap()` at the end to fix error
    // > no field `first_name` on type `std::result::Result<rust_server::models::User, diesel::result::Error>`
    let user = Users.find(1).get_result::<User>(&connection).unwrap();
    println!("{:#?}", user);
}
