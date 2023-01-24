use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};

use crate::repository::TagRepository;

use crate::{dto::tag::CreateTagDto, model::NewTag, AppError, AppState};

pub async fn find(State(app_state): State<AppState>) -> Result<Response, AppError> {
    let mut conn = app_state.pool.get().await.unwrap();
    let tags = TagRepository::find(&mut conn).await?;
    Ok(Json(tags).into_response())
}

pub async fn find_one(
    State(app_state): State<AppState>,
    Path(tag_id): Path<i32>,
) -> Result<Response, AppError> {
    let mut conn = app_state.pool.get().await?;
    let tag = TagRepository::find_one(&mut conn, tag_id).await?;
    Ok(Json(tag).into_response())
}

pub async fn create(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateTagDto>,
) -> Result<Response, AppError> {
    let mut conn = app_state.pool.get().await.unwrap();
    let new_tag = NewTag {
        name: &payload.name,
        slug: &payload.name.replace(" ", "-"),
        post_id: payload.post_id,
    };
    let tag = TagRepository::create(&mut conn, new_tag).await?;
    Ok(Json(tag).into_response())
}
