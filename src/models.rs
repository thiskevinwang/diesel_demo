use super::chrono::NaiveDateTime;
use super::diesel_derive_enum::DbEnum;
use super::schema::{Attempts, Comments, Reactions, Users};
use serde::{Deserialize, Serialize};

#[derive(Queryable)]
pub struct Attempt {
    pub id: i32,
    pub created: NaiveDateTime,
    pub updated: Option<NaiveDateTime>,
    pub deleted: Option<NaiveDateTime>,
    pub grade: i32,
    pub send: Option<bool>,
    pub flash: Option<bool>,
    pub date: Option<NaiveDateTime>,
    pub userId: i32,
}

#[derive(Queryable)]
pub struct Comment {
    pub id: i32,
    pub created: NaiveDateTime,
    pub updated: Option<NaiveDateTime>,
    pub deleted: Option<NaiveDateTime>,
    pub type_: String,
    pub body: String,
    pub url: String,
    pub userId: i32,
}

#[derive(Debug, DbEnum)]
pub enum ReactionsVariantEnum {
    Like,
    Love,
    Haha,
    Wow,
    Sad,
    Angry,
    None,
}

#[derive(Queryable)]
pub struct Reaction {
    pub id: i32,
    pub created: NaiveDateTime,
    pub updated: Option<NaiveDateTime>,
    pub deleted: Option<NaiveDateTime>,
    pub type_: String,
    pub variant: ReactionsVariantEnum,
    pub commentId: i32,
    pub userId: i32,
}

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub created: NaiveDateTime,
    pub updated: Option<NaiveDateTime>,
    pub deleted: Option<NaiveDateTime>,
    pub type_: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: Option<String>,
    pub last_password_request: Option<NaiveDateTime>,
    pub verified_date: Option<NaiveDateTime>,
    pub banned: Option<bool>,
}

#[derive(Insertable)]
#[table_name = "Users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}
