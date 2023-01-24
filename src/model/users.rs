use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use diesel::prelude::*;

use crate::schema::users;

#[derive(Identifiable, Queryable, PartialEq, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hashed_password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub hashed_password: String,
}

#[derive(Serialize)]
pub struct UserPosts<T> {
    #[serde(flatten)]
    pub user: User,
    pub posts: Vec<T>,
}

#[derive(Serialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}
