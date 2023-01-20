use crate::{model::*, schema::tags};
use anyhow::{anyhow, Error, Result};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};

use diesel::prelude::*;

pub struct Tags {}

impl Tags {
    pub async fn create(conn: &mut PgConnection, new_tag: NewTag<'_>) -> Result<Tag> {
        let tag: Tag = diesel::insert_into(tags::table)
            .values(&new_tag)
            .get_result(conn)
            .expect("Saving failse");
        Ok(tag)
    }

    pub async fn find(conn: &mut PgConnection) -> Result<Vec<Tag>> {
        let tags = tags::table.load::<Tag>(conn)?;
        Ok(tags)
    }

    pub async fn find_one(conn: &mut PgConnection, tag_id: i32) -> Result<Tag, Error> {
        let result = tags::table.filter(tags::id.eq(tag_id)).get_result(conn);
        let tag = match result {
            Ok(post) => post,
            Err(_) => return Err(anyhow!(format!("id: {} non-exist", tag_id))),
        };
        Ok(tag)
    }
}
