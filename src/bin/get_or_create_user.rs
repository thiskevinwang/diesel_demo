extern crate diesel;
extern crate rust_server;

// This brings `.filter()` and other methods into scope
use self::diesel::prelude::*;

// This enables `self::models::*;`
// This brings `establish_connection` into scope;
use self::rust_server::*;

// This brings `User` into scope
use self::models::*;

// bring this module into scope
mod random_chars;
// import a pub fn
use random_chars::generate_password_from_length;

/// # get_or_create_user
/// This helper function attempts to find a User by id.
///
/// If it doesn't exist, it will create a new User, with
/// next incremented `id` according to the DB.
pub fn get_or_create_user(primary_key: i32) -> User {
    use rust_server::schema::Users::dsl::*;
    let connection = establish_connection();

    // Try to get a user;
    let user = Users.find(primary_key).get_result::<User>(&connection);

    match user {
        // return the existing user
        Ok(user) => {
            println!("Found an existing user: {}\n", user.id);
            return user;
        }
        //
        Err(e) => {
            println!("Oops: {}", e);
            println!("Creating a new user...\n");
        }
    }

    let new_user = NewUser {
        username: format!(
            "{}_{}",
            generate_password_from_length(4),
            generate_password_from_length(4)
        ),
        email: format!("{}@test.com", generate_password_from_length(8)),
        password: generate_password_from_length(8),
        first_name: String::from("Kevin"),
        last_name: String::from("Wang"),
    };

    use schema::Users;
    let user = diesel::insert_into(Users::table)
        .values(&new_user)
        .get_result::<User>(&connection)
        .expect("An error occured when saving a new user");
    return user;
}

fn main() {
    let user = get_or_create_user(1);
    println!("{:?}", user);
}
