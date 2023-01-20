use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub password: String,
}
