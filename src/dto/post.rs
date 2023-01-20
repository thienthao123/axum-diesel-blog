use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePostDto {
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Deserialize)]
pub struct UpdatePostDto<'a> {
    pub user_id: i32,
    pub title: Option<&'a str>,
    pub body: Option<&'a str>,
    pub published: Option<bool>,
}
