use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};

use axum::http::header::AUTHORIZATION;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{model::users::UserLogin, repository::AuthRepository, AppError, AppState};

pub struct AuthController;

impl AuthController {
    pub async fn authorize(
        State(app_state): State<AppState>,
        Json(payload): Json<AuthPayload>,
    ) -> Result<Response, AppError> {
        let mut conn = app_state.pool.get().await?;
        let auth = UserLogin {
            username: payload.username,
            password: payload.password,
        };
        let user = AuthRepository::login(&mut conn, auth).unwrap();
        let claims = Claims {
            username: user.username,
            exp: 2000000000,
        };
        let token = encode(&Header::default(), &claims, &KEYS.encoding)
            .map_err(|_| AuthError::TokenCreation)
            .unwrap();
        Ok(Json(AuthBody::new(token)).into_response())
    }

    pub async fn test(claims: Claims) -> Result<Response, AppError> {
        Ok(Json(claims).into_response())
    }
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = "123123";
    Keys::new(secret.as_bytes())
});
struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    username: String,
    exp: usize,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let bearer = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(AuthError::MissingCredentials)?
            .to_str()
            .map_err(|_| AuthError::MissingCredentials)?;
        let t = match bearer.split_once(' ') {
            Some((_, contens)) => contens,
            _ => return Err(AuthError::MissingCredentials),
        };
        let token_data = decode::<Claims>(t, &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;
        Ok(token_data.claims)
    }
}

#[derive(Deserialize, Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    fn new(token: String) -> Self {
        Self {
            access_token: token,
            token_type: "Bearer".to_owned(),
        }
    }
}

#[derive(Deserialize)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}
#[derive(Deserialize, Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, s) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let json = json!({ "message": s });
        (status_code, Json(json)).into_response()
    }
}
