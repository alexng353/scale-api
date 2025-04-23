pub(super) use utoipa_axum::router::OpenApiRouter;
pub(super) use utoipa_axum::routes;

use crate::AppState;

pub mod auth;
pub mod user;
pub mod weight;
// pub mod muscles;

pub fn router(state: AppState) -> OpenApiRouter {
    OpenApiRouter::new()
        .with_state(state.clone())
        .nest("/auth", auth::router(state.clone()))
        .nest("/weight", weight::router(state.clone()))
        .nest("/user", user::router(state.clone()))
    // .nest("/muscles", muscles::router(state.clone()))
}
