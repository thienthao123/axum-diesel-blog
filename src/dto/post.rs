use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePostDto {
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Deserialize)]
pub struct UpdatePostDto {
    pub title: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>,
}
