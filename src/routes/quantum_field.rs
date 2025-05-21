//! # Quantum Field Route Handler
//!
//! This module handles the API endpoints for the 6-Fold Wisdom Field
//! that allows users to explore quantum wisdom in a structured field.

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::quantum_field::QuantumField;
use crate::state::AppState;

/// The response structure for quantum field API requests
#[derive(Debug, Serialize)]
pub struct QuantumFieldResponse {
    pub wisdom_field: Vec<WisdomNodeResponse>,
}

/// The response structure for a wisdom node
#[derive(Debug, Serialize)]
pub struct WisdomNodeResponse {
    pub index: usize,
    pub domain: String,
    pub seed: String,
}

/// The response structure for a collapsed field
#[derive(Debug, Serialize)]
pub struct CollapsedFieldResponse {
    pub selected_index: usize,
    pub collapsed_prompt: String,
}

/// Query parameters for collapsing the field
#[derive(Debug, Deserialize)]
pub struct CollapseParams {
    pub index: usize,
}

/// Handler function for GET /api/quantum-field endpoint
///
/// Returns the 6-fold wisdom field in superposition
pub async fn get_quantum_field(
    State(state): State<AppState>,
) -> Result<Json<QuantumFieldResponse>, (StatusCode, String)> {
    // Get the quantum field from the state
    let field = state.get_quantum_field().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get quantum field: {}", e)))?;
    
    // Convert to response format
    let response = QuantumFieldResponse {
        wisdom_field: field.get_wisdom_field().iter().map(|node| {
            WisdomNodeResponse {
                index: node.index,
                domain: node.domain.clone(),
                seed: node.seed.clone(),
            }
        }).collect(),
    };
    
    Ok(Json(response))
}

/// Handler function for GET /api/quantum-field/collapse endpoint
///
/// Collapses the field based on the selected node index
pub async fn collapse_quantum_field(
    State(state): State<AppState>,
    Query(params): Query<CollapseParams>,
) -> Result<Json<CollapsedFieldResponse>, (StatusCode, String)> {
    // Get and collapse the quantum field
    let mut field = state.get_quantum_field().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get quantum field: {}", e)))?;
    
    // Collapse the field
    let collapsed_prompt = field.collapse(params.index).to_string();
    
    // Create the response
    let response = CollapsedFieldResponse {
        selected_index: params.index,
        collapsed_prompt,
    };
    
    Ok(Json(response))
}
