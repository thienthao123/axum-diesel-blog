use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTagDto {
    pub post_id: i32,
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateTagDto {
    pub name: Option<String>,
    pub post_id: Option<i32>,
}
