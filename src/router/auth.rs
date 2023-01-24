use axum::{
    routing::{get, post},
    Router,
};

use crate::{controllers::auth_controller, AppState};

pub fn auth_router() -> Router<AppState> {
    let router: Router<AppState> = Router::new()
        .route("/login", post(auth_controller::AuthController::authorize))
        .route("/logined", get(auth_controller::AuthController::test));
    router
}
