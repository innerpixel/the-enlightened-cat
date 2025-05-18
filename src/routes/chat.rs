//! # Chat Route Handler
//! 
//! This module handles the API endpoint for chat interactions with the Enlightened Cat.
//! It processes user messages, sends them to the Mistral AI API via our client,
//! and returns the AI-generated responses.

// Import necessary dependencies:
// - axum: Web framework (similar to Express in Node.js)
// - serde: Serialization/deserialization library (for JSON handling)
// - tracing: Logging framework
use axum::{
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
    pub message: String,           // The user's message to the Enlightened Cat
    pub conversation_depth: Option<u32>,  // How many exchanges have occurred
    pub current_topic: Option<String>,    // The current conversation topic if any
}

/// Structure representing the response sent back to the user
/// 
/// `#[derive(Debug, Serialize)]` automatically implements:
/// - Debug: For printing the struct during debugging
/// - Serialize: For converting this struct to JSON
#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub message: String,                      // The Enlightened Cat's response
    pub suggested_topics: Option<Vec<String>>, // Optional suggested topics for exploration
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
    
    // Check conversation depth to determine response type
    let conversation_depth = request.conversation_depth.unwrap_or(0);
    let current_topic = request.current_topic.clone();
    
    // Log conversation context
    info!("Conversation depth: {}, Current topic: {:?}", conversation_depth, current_topic);
    
    // Send the message to the Mistral client and handle the result
    match state.mistral_client.get_enlightened_cat_response(&request.message).await {
        // If successful, process the response based on conversation context
        Ok(response) => {
            info!("Generated response from Enlightened Cat");
            
            // Generate suggested topics if we're at the right conversation depth
            // and the user seems interested in deeper conversation
            let suggested_topics = if conversation_depth >= 2 && 
                (request.message.to_lowercase().contains("yes") || 
                 request.message.to_lowercase().contains("more") || 
                 request.message.to_lowercase().contains("tell me") ||
                 request.message.to_lowercase().contains("deeper")) {
                
                // Generate topics based on the conversation so far
                // In a real implementation, you might use the AI to suggest these
                Some(generate_topic_suggestions(&request.message))
            } else {
                None
            };
            
            Ok(Json(ChatResponse { 
                message: response,
                suggested_topics
            }))
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

/// Generate topic suggestions based on the user's message
/// 
/// In a production app, you might use AI to generate these dynamically
/// based on the conversation history.
fn generate_topic_suggestions(message: &str) -> Vec<String> {
    // Simple keyword-based topic suggestions
    // In a real app, you'd use more sophisticated NLP or AI for this
    let message_lower = message.to_lowercase();
    
    if message_lower.contains("work") || message_lower.contains("job") || message_lower.contains("career") {
        return vec![
            "Work-life balance".to_string(),
            "Finding meaning in your career".to_string(),
            "Mindfulness at work".to_string(),
        ];
    } else if message_lower.contains("stress") || message_lower.contains("anxiety") || message_lower.contains("overwhelm") {
        return vec![
            "Stress reduction techniques".to_string(),
            "Mindful breathing".to_string(),
            "Creating peaceful spaces".to_string(),
        ];
    } else if message_lower.contains("meditat") || message_lower.contains("mindful") {
        return vec![
            "Daily meditation practices".to_string(),
            "Mindfulness in everyday moments".to_string(),
            "The science of meditation".to_string(),
        ];
    } else {
        // Default topics if no keywords match
        return vec![
            "Finding balance".to_string(),
            "Mindfulness practices".to_string(),
            "Creating peaceful moments".to_string(),
        ];
    }
}
