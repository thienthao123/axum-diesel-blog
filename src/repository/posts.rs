use crate::model::posts::PostTags;
use crate::model::{NewPost, Post, Tag};
use crate::schema::posts;

use anyhow::{anyhow, Result};
use diesel::{PgConnection, RunQueryDsl};

use diesel::prelude::*;

pub struct Posts {}

impl Posts {
    pub fn find(conn: &mut PgConnection) -> Result<Vec<PostTags>> {
        let posts = posts::table.load::<Post>(conn)?;
        let post_tags = Self::get_tags_for_post(conn, posts);
        post_tags
    }

    pub fn create(conn: &mut PgConnection, new_post: NewPost) -> Result<Post> {
        let post: Post = diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result(conn)
            .expect("Saving post is error");
        Ok(post)
    }

    pub fn find_one(conn: &mut PgConnection, post_id: i32) -> Result<PostTags> {
        let result = posts::table.filter(posts::id.eq(post_id)).get_result(conn);
        let post: Post = match result {
            Ok(post) => post,
            Err(e) => return Err(anyhow!(e)),
        };
        let tags: Vec<Tag> = Tag::belonging_to(&post).load::<Tag>(conn)?;
        let post_tags = PostTags { post, tags };
        Ok(post_tags)
    }

    pub fn get_tags_for_post(conn: &mut PgConnection, posts: Vec<Post>) -> Result<Vec<PostTags>> {
        let tags = Tag::belonging_to(&posts)
            .load::<Tag>(conn)?
            .grouped_by(&posts);
        let data: Vec<(Post, Vec<Tag>)> = posts.into_iter().zip(tags).collect();
        let result = data
            .into_iter()
            .map(|r| PostTags {
                post: r.0,
                tags: r.1,
            })
            .collect();
        Ok(result)
    }
}
