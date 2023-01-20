use crate::model::posts::PostTags;
use crate::model::{NewUser, Post, User};
use crate::repository;
use crate::schema::users;
use anyhow::{Ok, Result};
use diesel::prelude::*;
use diesel::{PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
pub struct Users;

#[derive(Serialize)]
pub struct UserPosts<T> {
    #[serde(flatten)]
    user: User,
    posts: Vec<T>,
}

impl Users {
    pub async fn find(conn: &mut PgConnection) -> Result<Vec<UserPosts<Post>>> {
        let results = users::table.load::<User>(conn).unwrap();
        let posts = Post::belonging_to(&results)
            .load::<Post>(conn)?
            .grouped_by(&results);
        let data: Vec<(User, Vec<Post>)> = results.into_iter().zip(posts).collect::<Vec<_>>();
        let result = data
            .into_iter()
            .map(|tuple| UserPosts::<Post> {
                user: tuple.0,
                posts: tuple.1,
            })
            .collect();
        Ok(result)
    }

    pub async fn create(conn: &mut PgConnection, new_user: NewUser) -> Result<User> {
        let user: User = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .expect("Cant create new User");
        Ok(user)
    }

    pub async fn find_one(conn: &mut PgConnection, user_id: i32) -> Result<UserPosts<PostTags>> {
        let user: User = users::table
            .filter(users::id.eq(user_id))
            .get_result(conn)?;
        let posts: Vec<Post> = Post::belonging_to(&user).load::<Post>(conn)?;
        let posts_tags = repository::Posts::get_tags_for_post(conn, posts)?;
        let user_posts = UserPosts::<PostTags> {
            user,
            posts: posts_tags,
        };
        Ok(user_posts)
    }
}
