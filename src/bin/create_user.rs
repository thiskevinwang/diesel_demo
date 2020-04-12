extern crate diesel;
extern crate rand;
extern crate rust_server;

use self::diesel::*;
use self::models::*;
use self::rust_server::*;
use bcrypt::{hash, DEFAULT_COST};

mod random_chars;
use random_chars::generate_password_from_length;

/// cargo run --bin create_user
fn main() {
    // use rust_server::schema::Users::dsl::*;
    let connection = establish_connection();

    let chars = generate_password_from_length(8);
    println!("{}", chars);

    let hashed = hash(generate_password_from_length(8), DEFAULT_COST).unwrap();

    // use self::models::NewUser;
    let new_user = NewUser {
        username: format!(
            "{}_{}",
            generate_password_from_length(4),
            generate_password_from_length(4)
        ),
        email: format!("{}@test.com", generate_password_from_length(8)),
        password: hashed,
        first_name: String::from("Kevin"),
        last_name: String::from("Wang"),
    };

    use schema::Users;
    let user = diesel::insert_into(Users::table)
        .values(&new_user)
        .get_result::<User>(&connection)
        .expect("An error occured when saving a new user");
    println!("{:#?}", user)
}
