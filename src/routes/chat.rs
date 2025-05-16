use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub message: String,
}

pub async fn handle_chat(
    State(state): State<AppState>,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, (StatusCode, String)> {
    info!("Received chat request: {}", request.message);
    
    match state.mistral_client.get_enlightened_cat_response(&request.message).await {
        Ok(response) => {
            info!("Generated response from Enlightened Cat");
            Ok(Json(ChatResponse { message: response }))
        }
        Err(err) => {
            error!("Error generating response: {:?}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to generate response: {}", err),
            ))
        }
    }
}
