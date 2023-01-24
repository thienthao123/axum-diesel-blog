use axum::Router;

use crate::AppState;

use super::{
    auth::auth_router, posts_router::posts_router, tags_router::tags_router,
    users_router::users_router,
};

pub fn root_router() -> axum::Router<AppState> {
    let router = Router::new()
        .nest("/posts", posts_router())
        .nest("/tags", tags_router())
        .nest("/users", users_router())
        .nest("/auth", auth_router());

    router
}
