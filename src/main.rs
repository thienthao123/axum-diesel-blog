use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router};
use bb8::Pool;
use bb8_diesel::DieselConnectionManager;
use diesel::PgConnection;
use dotenvy::dotenv;
use router::root::root_router;
use serde_json::json;
use std::env;
use std::net::SocketAddr;

mod controllers;
mod dto;
mod model;
mod repository;
mod router;
mod schema;
type ConnectionPool = Pool<DieselConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub pool: ConnectionPool,
}

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let json = json!({ "message": format!("Something went wrong {}", self.0) });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json)).into_response()
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = DieselConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).await.unwrap();
    let state = AppState { pool };

    let app = Router::new().nest("/", root_router()).with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// allow use "?" in fn router
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
