use uuid::Uuid;

use crate::{extractors::users::UserId, *};

#[derive(sqlx::Type, Serialize, Deserialize, ToSchema)]
#[sqlx(type_name = "weight_unit")]
pub enum WeightUnit {
    #[sqlx(rename = "LBs")]
    LBs,
    #[sqlx(rename = "KGs")]
    KGs,
}

#[derive(Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct WeightRowReturning {
    id: Uuid,
    unit: WeightUnit, // for casting reasons (sqlx funny behaviour)
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
#[axum::debug_handler]
pub async fn log_weight(
    UserId(user_id): UserId,
    State(state): State<AppState>,
    Json(body): Json<LogWeightBody>,
) -> Result<String, AppError> {
    let (weight_lbs, weight_kgs) = body.as_both_units();
    info!("logging weight for user {} @ {} LBs", user_id, weight_lbs);

    let result = sqlx::query_as!(
        WeightRowReturning,
        r#"INSERT INTO weights (
            user_id,
            weightLBs,
            weightKGs,
            unit
        )
        VALUES ($1, $2, $3, $4)
        RETURNING id, unit as "unit!: WeightUnit""#r,
        user_id,
        weight_lbs,
        weight_kgs,
        body.unit as WeightUnit
    )
    .fetch_one(&*state.db)
    .await;

    let id = match result {
        Ok(row) => row.id,
        Err(err) => {
            error!("failed to log weight: {}", err);
            panic!("failed to log weight: {}", err);
        }
    };

    Ok(id.to_string())
}
