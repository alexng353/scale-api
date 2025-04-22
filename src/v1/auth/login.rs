use crate::*;
use argon2::{
    password_hash::{Encoding, PasswordHash, PasswordVerifier},
    Argon2,
};
use axum::http::StatusCode;
use jwt::SignWithKey;
use sqlx::query;
use util::auth::JWTClaims;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginBody {
    username: String,
    password: String,
}

/// Login
#[utoipa::path(
    post,
    path = "/login",
    responses(
        (status = OK, body = String),
        (status = BAD_REQUEST, body = String),
        (status = UNAUTHORIZED, body = String),
        (status = NOT_FOUND, body = String),
        (status = INTERNAL_SERVER_ERROR, body = String)
    ),
    tag = super::AUTH_TAG
)]
pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginBody>,
) -> (StatusCode, String) {
    info!("User {} logging in", body.username);
    let user = query!(
        "SELECT id, real_name, username, email, password_hash, is_admin, created_at
        FROM users
        WHERE username = $1 OR email = $1",
        body.username,
    )
    .fetch_one(&*state.db)
    .await;

    let user = match user {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            return (StatusCode::NOT_FOUND, "User not found".into())
        }
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };

    let argon2 = Argon2::default();
    let hash = PasswordHash::parse(&user.password_hash, Encoding::B64)
        .expect("Password hashing failed");

    if !argon2
        .verify_password(body.password.as_bytes(), &hash)
        .is_ok()
    {
        return (StatusCode::UNAUTHORIZED, "Incorrect password".into());
    }

    let claims =
        JWTClaims::new(user.id, user.username, user.real_name, user.email);
    let token_str = match claims.sign_with_key(&state.jwt_key) {
        Ok(token_str) => token_str,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };

    (StatusCode::OK, token_str)
}
