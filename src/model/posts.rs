use crate::model::User;
use crate::schema::posts;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use super::Tag;

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Deserialize, Serialize)]
#[belongs_to(User)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = posts)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>,
}

#[derive(Serialize)]
pub struct PostTags {
    #[serde(flatten)]
    pub post: Post,
    pub tags: Vec<Tag>,
}
