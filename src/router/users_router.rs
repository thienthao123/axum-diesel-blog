use axum::{
    routing::{get, post},
    Router,
};

use crate::{controllers::users_controller, AppState};

pub fn users_router() -> Router<AppState> {
    let router = Router::new()
        .route("/", get(users_controller::find))
        .route("/", post(users_controller::create))
        .route("/:id", get(users_controller::find_one));
    router
}
