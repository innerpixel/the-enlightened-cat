//! # Chat Route Handler
//! 
//! This module handles the API endpoint for chat interactions with the Enlightened Cat.
//! It processes user messages, sends them to the Mistral AI API via our client,
//! and returns the AI-generated responses.

// Import necessary dependencies:
// - axum: Web framework (similar to Express in Node.js)
// - serde: Serialization/deserialization library (for JSON handling)
// - tracing: Logging framework
use axum:{
    extract::{Json, State},  // Extractors to get JSON data and app state from requests
    http::StatusCode,        // HTTP status codes (200 OK, 500 Error, etc.)
};
use serde::{Deserialize, Serialize};  // Traits for JSON conversion
use tracing::{error, info};           // Logging utilities

// Import our application state
use crate::state::AppState;

/// Structure representing an incoming chat request from the user
/// 
/// `#[derive(Debug, Deserialize)]` automatically implements:
/// - Debug: For printing the struct during debugging
/// - Deserialize: For converting JSON to this struct
#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,  // The user's message to the Enlightened Cat
}

/// Structure representing the response sent back to the user
/// 
/// `#[derive(Debug, Serialize)]` automatically implements:
/// - Debug: For printing the struct during debugging
/// - Serialize: For converting this struct to JSON
#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub message: String,  // The Enlightened Cat's response
}

/// Handler function for POST /api/chat endpoint
/// 
/// This function:
/// 1. Extracts the application state and JSON request body
/// 2. Sends the user's message to the Mistral AI API
/// 3. Returns the AI response as JSON
/// 4. Handles any errors that might occur
/// 
/// The `async` keyword allows this function to perform I/O operations
/// without blocking the server thread.
pub async fn handle_chat(
    // Extract the AppState from the request
    State(state): State<AppState>,
    // Extract and parse the JSON request body into a ChatRequest struct
    Json(request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, (StatusCode, String)> {
    // Log the incoming message
    info!("Received chat request: {}", request.message);
    
    // Send the message to the Mistral client and handle the result
    match state.mistral_client.get_enlightened_cat_response(&request.message).await {
        // If successful, wrap the response in a ChatResponse and return as JSON
        Ok(response) => {
            info!("Generated response from Enlightened Cat");
            Ok(Json(ChatResponse { message: response }))
        }
        // If there's an error, log it and return a 500 error response
        Err(err) => {
            error!("Error generating response: {:?}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to generate response: {}", err),
            ))
        }
    }
}
