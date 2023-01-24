use anyhow::Result;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{encode, Header};

use crate::auth::jwt;

use crate::{model::users::UserLogin, repository::AuthRepository, AppError, AppState};

pub struct AuthController;

impl AuthController {
    pub async fn authorize(
        State(app_state): State<AppState>,
        Json(payload): Json<jwt::AuthPayload>,
    ) -> Result<Response, AppError> {
        let mut conn = app_state.pool.get().await?;
        let auth = UserLogin {
            username: payload.username,
            password: payload.password,
        };
        let user = AuthRepository::login(&mut conn, auth).map_err(|_| jwt::AuthError::InvalidToken);
        match user {
            Ok(u) => {
                let claims = jwt::Claims {
                    user_id: u.id,
                    username: u.username,
                    exp: 2000000000,
                };
                let token = encode(&Header::default(), &claims, &jwt::KEYS.encoding)
                    .map_err(|_| jwt::AuthError::TokenCreation)
                    .unwrap();
                Ok(Json(jwt::AuthBody::new(token)).into_response())
            }
            Err(_) => Ok(jwt::AuthError::InvalidToken.into_response()),
        }
    }

    pub async fn test(claims: jwt::Claims) -> Result<Response, AppError> {
        Ok(Json(claims).into_response())
    }
}
