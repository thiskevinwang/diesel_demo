// ❓ how does `super::schema` point to
// src/schema.rs?
use super::chrono::NaiveDateTime;
use super::schema::posts;

/**
 * A note of Field Order
 *
 * Using #[derive(Queryable)] assumes that the
 * order of fields on the **Post** struct matches
 * the columns in the **posts** table, so make
 * sure to define them in the order seen in the
 * **schema.rs** file.
 *
 * https://docs.diesel.rs/diesel/deserialize/trait.Queryable.html
 */
#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    /**
     * | Rust        | SQL              |
     * | :---------- | :--------------- |
     * | i32         | INTEGER          |
     * | Option<i32> | INTEGER NOT NULL |
     */
    pub user_id: Option<i32>,
}

/**
 * https://docs.diesel.rs/diesel_derives/derive.Insertable.html
 */
#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub user_id: &'a i32,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub updated_at: NaiveDateTime,
    pub deleted_at: NaiveDateTime,
    pub name: String,
    pub username: String,
}
