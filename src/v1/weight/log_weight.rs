use crate::{extractors::users::UserId, *};

#[derive(Serialize, Deserialize, ToSchema)]
pub enum WeightUnit {
    LBs,
    KGs,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LogWeightBody {
    weight: f64,
    unit: WeightUnit,
}

impl LogWeightBody {
    pub fn as_both_units(&self) -> (f64, f64) {
        match self.unit {
            WeightUnit::LBs => (self.weight, self.weight / 2.20462),
            WeightUnit::KGs => (self.weight * 2.20462, self.weight),
        }
    }
}

#[utoipa::path(
    post,
    path = "/log_weight",
    responses(
        (status = OK, body = String)
    ),
    tag = super::WEIGHT_TAG
)]
pub async fn log_weight(
    UserId(user_id): UserId,
    State(state): State<AppState>,
    Json(body): Json<LogWeightBody>,
) -> Result<String, AppError> {
    let (weight_lbs, weight_kgs) = body.as_both_units();
    info!("logging weight for user {} @ {} LBs", user_id, weight_lbs);

    let result = sqlx::query!(
        "INSERT INTO weights (user_id, weightLBs, weightKGs)
        VALUES ($1, $2, $3)
        RETURNING id",
        user_id,
        weight_lbs,
        weight_kgs
    )
    .fetch_one(&*state.db)
    .await?;

    Ok(result.id.to_string())
}
