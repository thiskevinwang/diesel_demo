extern crate diesel;
extern crate rust_server;

use self::rust_server::*;
use std::io::{stdin, Read};

fn custom_unwrap(potentially_null_user_id: Option<i32>) -> i32 {
    match potentially_null_user_id {
        Some(1) => 1,
        Some(whatever) => whatever,
        None => 0,
    }
}

/// run with `cargo run --bin write_post`
fn main() {
    let connection = establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)];

    println!(
        "\nOk! Let's write {} (Press {} when finished) \n",
        title, EOF
    );
    let mut body = String::new();
    let user_id = 1;
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(&connection, title, &body, &user_id);
    println!(
        "\nSaved draft {} with id {}. user_id:{}",
        title,
        post.id,
        custom_unwrap(post.user_id),
    );
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
