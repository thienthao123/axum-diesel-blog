use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::model::User;
use crate::schema::posts;

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

#[derive(Serialize)]
pub struct PostTags {
    #[serde(flatten)]
    pub post: Post,
    pub tags: Vec<Tag>,
}
