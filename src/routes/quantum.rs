//! # Quantum Wisdom Route Handler
//! 
//! This module handles the API endpoints for retrieving and observing quantum wisdom quotes
//! from the Enlightened Cat. It provides JSON APIs that return quantum wisdom in superposition
//! and allow users to observe it, causing it to collapse into a single state.

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::Serialize;
use tracing::{error, info};

use crate::state::AppState;
use crate::quantum::QuantumWisdom;

/// The response structure for quantum wisdom API requests
#[derive(Debug, Serialize)]
pub struct QuantumWisdomResponse {
    pub quantum_wisdom: QuantumWisdom,
    pub timestamp: String,
}

/// Handler function for GET /api/quantum-wisdom endpoint
/// 
/// Returns quantum wisdom in superposition (all potential states)
pub async fn get_quantum_wisdom(
    State(state): State<AppState>,
) -> Result<Json<QuantumWisdomResponse>, (StatusCode, String)> {
    info!("Fetching quantum wisdom in superposition");
    
    match state.get_quantum_wisdom().await {
        Ok(wisdom) => {
            let now = chrono::Utc::now();
            Ok(Json(QuantumWisdomResponse {
                quantum_wisdom: wisdom,
                timestamp: now.to_rfc3339(),
            }))
        }
        Err(err) => {
            error!("Error fetching quantum wisdom: {:?}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch quantum wisdom: {}", err),
            ))
        }
    }
}

use axum::extract::Query;
use serde::Deserialize;

/// Query parameters for observing quantum wisdom
#[derive(Debug, Deserialize)]
pub struct ObserveParams {
    /// Optional index of the specific state to observe
    pub state_index: Option<usize>,
}

/// Handler function for GET /api/quantum-wisdom/observe endpoint
/// 
/// Observes the quantum wisdom, causing it to collapse into a single state
/// Accepts an optional state_index query parameter to observe a specific state
pub async fn observe_quantum_wisdom(
    Query(params): Query<ObserveParams>,
    State(state): State<AppState>,
) -> Result<Json<QuantumWisdomResponse>, (StatusCode, String)> {
    info!("Observing quantum wisdom, causing collapse");
    
    match state.get_quantum_wisdom().await {
        Ok(mut wisdom) => {
            if !wisdom.observed {
                // Observe the wisdom, causing it to collapse
                if let Some(index) = params.state_index {
                    info!("Observing specific state at index {}", index);
                    wisdom.observe_specific(index);
                } else {
                    info!("No specific state selected, observing randomly");
                    wisdom.observe();
                }
                
                // Update the stored wisdom with the collapsed state
                let mut stored_wisdom = state.quantum_wisdom.write().await;
                *stored_wisdom = Some(wisdom.clone());
            }
            
            let now = chrono::Utc::now();
            Ok(Json(QuantumWisdomResponse {
                quantum_wisdom: wisdom,
                timestamp: now.to_rfc3339(),
            }))
        }
        Err(err) => {
            error!("Error observing quantum wisdom: {:?}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to observe quantum wisdom: {}", err),
            ))
        }
    }
}
