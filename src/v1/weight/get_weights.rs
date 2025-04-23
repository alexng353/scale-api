use axum::extract::Query;
use chrono::{DateTime, Utc};

use crate::{extractors::users::UserId, *};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct WeightResponse {
    id: uuid::Uuid,
    weight_lbs: f64,
    weight_kgs: f64,
    created_at: DateTime<Utc>,
}

/// Get weights. Automatically calculates offset based on limit and page.
#[utoipa::path(
    get,
    path = "/get_weight",
    responses(
        (status = OK, body = WeightResponse)
    ),
    params(
        ("limit" = Option<i64>, Query, description = "limit"),
        ("page" = Option<i64>, Query, description = "page")
    ),
    tag = super::WEIGHT_TAG
)]
pub async fn get_weights(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Query(limit): Query<Option<i64>>,
    Query(page): Query<Option<i64>>,
) -> Result<Json<Vec<WeightResponse>>, AppError> {
    let limit = limit.unwrap_or(15);
    let page = page.unwrap_or(1);
    let offset = (page - 1) * limit;

    let result = sqlx::query_as!(
        WeightResponse,
        "SELECT weightLBs as weight_lbs, weightKGs as weight_kgs, created_at, id
            FROM WEIGHTS
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            OFFSET $3",
        user_id,
        limit,
        offset
    )
    .fetch_all(&*state.db)
    .await?;

    Ok(Json(result))
}
