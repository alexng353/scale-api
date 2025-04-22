use crate::AppState;

pub(super) use super::*;

pub mod log_weight;

pub const WEIGHT_TAG: &str = "weight";

pub(super) fn router(state: AppState) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(log_weight::log_weight))
        .with_state(state)
}
