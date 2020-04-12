extern crate diesel;
extern crate rust_server;

// This brings `.filter()` and other methods into scope
use self::diesel::prelude::*;

// This enables `self::models::*;`
// This brings `establish_connection` into scope;
use self::rust_server::*;

// This brings `User` into scope
use self::models::*;

pub fn get_user_by_id(primary_key: i32) -> User {
    use rust_server::schema::Users::dsl::*;

    let connection = establish_connection();
    let user = Users
        .find(primary_key)
        .get_result::<User>(&connection)
        .unwrap();

    return user;
}

/// # get_user_1
/// This function gets the User with `id: 1`.
///
/// Panics if that user doesn't exist
///
/// ```bash
/// cargo run --bin get_user_1
/// ```
fn main() {
    // use rust_server::schema::Users::dsl::*;

    // let connection = establish_connection();

    // note:
    // User is the model
    // Users is the schema (use this)

    // add `.unwrap()` at the end to fix error
    // > no field `first_name` on type `std::result::Result<rust_server::models::User, diesel::result::Error>`
    // let user = Users.find(1).get_result::<User>(&connection).unwrap();

    let user = get_user_by_id(1);
    println!("{:?}", user);
}
