#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

// if you hover over `models`, it allows you
// to cmd+click into `src/models.rs`
// - How?
pub mod models;
/**
 * ... in schema.rs
 *
 * The table! macro creates a bunch of code
 * based on the datbase schema to represent
 * all of the tables and columns.
 *
 * Any time we run or revert a migration,
 * this file will get automatically updated
 */
pub mod schema;

pub fn establish_connection() -> PgConnection {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	PgConnection::establish(&database_url).expect(&format!("Error connection to {}", database_url))
}

use self::models::{NewPost, Post};
pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
	use schema::posts;

	let new_post = NewPost {
		title: title,
		body: body,
	};

	diesel::insert_into(posts::table)
		.values(&new_post)
		.get_result(conn)
		.expect("Error save new post")
}
