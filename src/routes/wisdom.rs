use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::Serialize;
use tracing::{error, info};

use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct WisdomResponse {
    pub wisdom: String,
    pub timestamp: String,
}

pub async fn get_daily_wisdom(
    State(state): State<AppState>,
) -> Result<Json<WisdomResponse>, (StatusCode, String)> {
    info!("Fetching daily wisdom");
    
    match state.get_daily_wisdom().await {
        Ok(wisdom) => {
            let now = chrono::Utc::now();
            Ok(Json(WisdomResponse {
                wisdom,
                timestamp: now.to_rfc3339(),
            }))
        }
        Err(err) => {
            error!("Error fetching daily wisdom: {:?}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch daily wisdom: {}", err),
            ))
        }
    }
}
