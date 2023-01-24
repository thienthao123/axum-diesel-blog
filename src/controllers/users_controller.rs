use crate::dto::user::CreateUserDto;
use crate::model::NewUser;
use crate::repository::UserRepository;
use crate::{AppError, AppState};
use anyhow::Result;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use axum::{extract::State, response::Response};

pub async fn find(State(app_state): State<AppState>) -> Result<Response, AppError> {
    let mut conn = app_state.pool.get().await?;
    let users = UserRepository::find(&mut conn).await?;
    Ok(Json(users).into_response())
}

pub async fn find_one(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Response, AppError> {
    let mut conn = app_state.pool.get().await?;
    let user = UserRepository::find_one(&mut conn, id).await?;
    Ok(Json(user).into_response())
}

pub async fn create(
    State(app_sate): State<AppState>,
    Json(payload): Json<CreateUserDto>,
) -> Result<Response, AppError> {
    let mut conn = app_sate.pool.get().await?;
    let new_user = NewUser {
        username: payload.username,
        hashed_password: payload.password,
    };
    let user = UserRepository::create(&mut conn, new_user).await?;
    Ok(Json(user).into_response())
}
