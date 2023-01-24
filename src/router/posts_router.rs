use axum::{
    routing::{get, post},
    Router,
};

use crate::{controllers::posts_controller, AppState};

pub fn posts_router() -> Router<AppState> {
    let router: Router<AppState> = Router::new()
        .route("/", get(posts_controller::find))
        .route("/", post(posts_controller::create))
        .route("/:id", get(posts_controller::find_one))
        .route("/:id", post(posts_controller::update));
    router
}
