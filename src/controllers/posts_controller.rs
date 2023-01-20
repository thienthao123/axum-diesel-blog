use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{dto::post::CreatePostDto, model::NewPost, repository, AppError, AppState};

pub async fn find(State(app_sate): State<AppState>) -> anyhow::Result<Response, AppError> {
    let mut conn = app_sate.pool.get().await.unwrap();
    let posts = repository::Posts::find(&mut conn)?;
    Ok(Json(posts).into_response())
}
pub async fn create(
    State(app_state): State<AppState>,
    Json(payload): Json<CreatePostDto>,
) -> impl IntoResponse {
    let mut conn = app_state.pool.get().await.unwrap();
    let new_post = NewPost {
        user_id: payload.user_id,
        title: payload.title,
        body: payload.body,
        published: payload.published,
    };
    let post = repository::Posts::create(&mut conn, new_post).unwrap();
    (StatusCode::CREATED, Json(post))
}

pub async fn get_post(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Response, AppError> {
    let mut conn = app_state.pool.get().await.unwrap();
    let result = repository::Posts::find_one(&mut conn, id)?;
    Ok(Json(result).into_response())
}
