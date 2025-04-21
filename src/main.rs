use std::{net::Ipv4Addr, sync::Arc};
use hmac::{Hmac, Mac};
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

// tags
use v1::{
    auth::AUTH_TAG,
    // exercises::EXERCISES_TAG,
    // muscles::MUSCLES_TAG
};

mod v1;
mod structs;
mod state;
mod db;
mod error;
mod util;
mod extractors;

pub(crate) use anyhow::Context;
pub(crate) use axum::extract::{Json, State};
pub(crate) use error::{AnyhowError, AppError, Errors};
pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use state::AppState;
pub(crate) use utoipa::ToSchema;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = AUTH_TAG, description = "Authentication API endpoints"),
        // (name = EXERCISES_TAG, description = "Exercise API endpoints"),
        // (name = MUSCLES_TAG, description = "Muscle API endpoints"),
        // (name = CUSTOMER_TAG, description = "Customer API endpoints"),
        // (name = ORDER_TAG, description = "Order API endpoints")
    ),
)]
struct ApiDoc;

/// Get health of the API.
#[utoipa::path(
    get,
    path = "/.well-known/health-check",
    responses(
        (status = OK, description = "Success", body = str, content_type = "text/plain")
    )
)]
async fn health_check() -> &'static str {
    "ok"
}

#[utoipa::path(
    get, 
    path = "/",
    responses(
        (status = OK, description = "Success", body = str, content_type = "text/plain")
    )
)]
async fn index() -> &'static str {
    "ok 200"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let port: u16 = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse()?;
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());

    let db = db::db().await?;
    let state = state::AppState {
        db: Arc::new(db),
        jwt_key: Hmac::new_from_slice(jwt_secret.as_bytes()).context("Failed to create HMAC")? 
    };

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(health_check))
        .routes(routes!(index))
        .with_state(state.clone())
        .nest("/api/v1", v1::router(state.clone()))
        .split_for_parts();

    let router = router.merge(
        SwaggerUi::new("/docs").url("/docs/openapi.json",api.clone())
    );

    println!("Listening on http://localhost:{port}");
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, port)).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
