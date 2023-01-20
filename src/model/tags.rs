use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::model::Post;
use crate::schema::tags;

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Deserialize, Serialize)]
#[diesel(belongs_to(Post))]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: i32,
    pub post_id: i32,
    pub name: String,
    pub slug: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag<'a> {
    pub post_id: i32,
    pub name: &'a str,
    pub slug: &'a str,
}
