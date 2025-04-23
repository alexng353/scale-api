use crate::AppState;

pub(super) use super::*;

pub mod get_weights;
pub mod log_weight;

pub const WEIGHT_TAG: &str = "weight";

pub(super) fn router(state: AppState) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(log_weight::log_weight))
        .routes(routes!(get_weights::get_weights))
        .with_state(state)
}
