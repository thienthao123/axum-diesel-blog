use axum::{
    routing::{get, post},
    Router,
};

use crate::{controllers::tags_controller, AppState};

pub fn tags_router() -> Router<AppState> {
    let router = Router::new()
        .route("/", get(tags_controller::find))
        .route("/", post(tags_controller::create))
        .route("/:id", get(tags_controller::find_one));
    router
}
