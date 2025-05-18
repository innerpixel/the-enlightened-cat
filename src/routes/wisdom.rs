//! # Wisdom Route Handler
//! 
//! This module handles the API endpoint for retrieving daily wisdom quotes from
//! the Enlightened Cat. It provides a JSON API that returns the current wisdom
//! along with a timestamp.

// Import necessary dependencies:
// - axum: The web framework we're using (similar to Express in Node.js)
// - serde: For serializing/deserializing data (similar to JSON.stringify/parse)
// - tracing: For logging (similar to Winston or Bunyan in Node.js)
use axum::{
    extract::State,      // For accessing application state in handlers
    http::StatusCode,    // For HTTP status codes like 200 OK, 500 Error
    Json,                // For returning JSON responses
};
use serde::Serialize;    // For making structs serializable to JSON
use tracing::{error, info}; // For logging information and errors

// Import our application state that contains the Mistral client
use crate::state::AppState;

/// The response structure for wisdom API requests
/// 
/// `#[derive(Debug, Serialize)]` is a macro that automatically implements:
/// - Debug: Allows printing the struct for debugging
/// - Serialize: Allows converting the struct to JSON
#[derive(Debug, Serialize)]
pub struct WisdomResponse {
    pub wisdom: String,      // The wisdom quote text
    pub timestamp: String,   // When the wisdom was generated
}

/// Handler function for GET /api/daily-wisdom endpoint
/// 
/// This function:
/// 1. Extracts the application state from the request
/// 2. Retrieves the daily wisdom from the state
/// 3. Returns it as JSON with a timestamp
/// 4. Handles any errors that might occur
/// 
/// The `async` keyword means this function can be paused/resumed,
/// allowing it to wait for I/O operations without blocking the thread.
pub async fn get_daily_wisdom(
    // Extract the AppState from the request using Axum's State extractor
    State(state): State<AppState>,
) -> Result<Json<WisdomResponse>, (StatusCode, String)> {
    // Log that we're fetching wisdom (will appear in application logs)
    info!("Fetching daily wisdom");
    
    // Try to get wisdom from the state and handle success/failure
    match state.get_daily_wisdom().await {
        // If successful, return the wisdom with current timestamp
        Ok(wisdom) => {
            let now = chrono::Utc::now();  // Get current UTC time
            Ok(Json(WisdomResponse {
                wisdom,
                timestamp: now.to_rfc3339(),  // Format timestamp as RFC3339
            }))
        }
        // If there's an error, log it and return a 500 error
        Err(err) => {
            error!("Error fetching daily wisdom: {:?}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch daily wisdom: {}", err),
            ))
        }
    }
}
