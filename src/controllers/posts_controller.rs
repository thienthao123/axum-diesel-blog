use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    auth::jwt::Claims,
    dto::post::{CreatePostDto, UpdatePostDto},
    model::{posts::UpdatePost, NewPost},
    AppError, AppState,
};

use crate::repository::PostRepository;

pub async fn find(State(app_sate): State<AppState>) -> anyhow::Result<Response, AppError> {
    let mut conn = app_sate.pool.get().await.unwrap();
    let posts = PostRepository::find(&mut conn)?;
    Ok(Json(posts).into_response())
}
pub async fn create(
    claims: Claims,
    State(app_state): State<AppState>,
    Json(payload): Json<CreatePostDto>,
) -> impl IntoResponse {
    let mut conn = app_state.pool.get().await.unwrap();
    let new_post = NewPost {
        user_id: claims.user_id,
        title: payload.title,
        body: payload.body,
        published: payload.published,
    };
    let post = PostRepository::create(&mut conn, new_post).unwrap();
    (StatusCode::CREATED, Json(post))
}

pub async fn find_one(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Response, AppError> {
    let mut conn = app_state.pool.get().await.unwrap();
    let result = PostRepository::find_one(&mut conn, id)?;
    Ok(Json(result).into_response())
}

pub async fn update(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePostDto>,
) -> Result<Response, AppError> {
    let mut conn = app_state.pool.get().await?;
    let update_post = UpdatePost {
        title: payload.title,
        body: payload.body,
        published: payload.published,
    };
    let post = PostRepository::update(&mut conn, id, update_post)?;
    Ok(Json(post).into_response())
}
