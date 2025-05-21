//! # Pages Route Handler
//! 
//! This module handles the rendering of HTML pages for The Enlightened Cat website.
//! It uses the Askama templating engine (similar to Jinja2 in Python or Handlebars in JavaScript)
//! to render HTML templates with dynamic content.

// Import necessary dependencies:
// - axum: Web framework for handling HTTP requests
// - askama: Templating engine for rendering HTML
// - tracing: Logging framework
use axum::{
    extract::State,     // For accessing application state in handlers
    response::Html,     // For returning HTML responses
};
use askama::Template;  // Trait that provides the render() method for templates
use tracing::info;     // For logging information

// Import our application state and template definitions
use crate::state::AppState;
use crate::templates::{AboutTemplate, IndexTemplate, WisdomTemplate, QuantumFieldTemplate};  // Import all template structs (IndexTemplate, AboutTemplate, etc.)

/// Handler function for the home page (GET /)
/// 
/// This function:
/// 1. Extracts the application state from the request
/// 2. Gets the daily wisdom from the state
/// 3. Renders the index template with the wisdom
/// 4. Returns the rendered HTML
/// 
/// The `async` keyword allows this function to perform I/O operations
/// without blocking the server thread.
pub async fn index(State(state): State<AppState>) -> Html<String> {
    // Log that we're rendering the index page
    info!("Rendering index page");
    
    // Get the daily wisdom, with a fallback message if there's an error
    let wisdom = state.get_daily_wisdom().await.unwrap_or_else(|_| {
        "Even in moments of technical difficulty, the enlightened cat remains calm and patient.".to_string()
    });
    
    // Create a template instance with the wisdom
    let template = IndexTemplate { daily_wisdom: wisdom };
    
    // Render the template to HTML and wrap it in an Html response
    // If rendering fails, provide a simple fallback HTML
    Html(template.render().unwrap_or_else(|_| {
        "<h1>The Enlightened Cat</h1><p>Wisdom loading...</p>".to_string()
    }))
}

/// Handler function for the about page (GET /about)
/// 
/// This function:
/// 1. Creates an AboutTemplate instance
/// 2. Renders it to HTML
/// 3. Returns the rendered HTML
/// 
/// Note that this handler doesn't need access to the application state
/// since it doesn't use any dynamic data from the state.
pub async fn about() -> Html<String> {
    // Log that we're rendering the about page
    info!("Rendering about page");
    
    // Create a template instance (with no dynamic data in this case)
    let template = AboutTemplate {};
    
    // Render the template to HTML and wrap it in an Html response
    // If rendering fails, provide a simple fallback HTML
    Html(template.render().unwrap_or_else(|_| {
        "<h1>About The Enlightened Cat</h1><p>Content loading...</p>".to_string()
    }))
}

/// Handler function for the wisdom page (GET /wisdom)
/// 
/// This function:
/// 1. Extracts the application state from the request
/// 2. Gets the daily wisdom from the state
/// 3. Renders the wisdom template with the wisdom
/// 4. Returns the rendered HTML
/// 
/// This page is dedicated to displaying the daily wisdom with sharing options.
pub async fn wisdom_page(State(state): State<AppState>) -> Html<String> {
    // Log that we're rendering the wisdom page
    info!("Rendering wisdom page");
    
    // Get the daily wisdom, with a fallback message if there's an error
    let wisdom = state.get_daily_wisdom().await.unwrap_or_else(|_| {
        "Even in moments of technical difficulty, the enlightened cat remains calm and patient.".to_string()
    });
    
    // Create a template instance with the wisdom
    let template = WisdomTemplate { daily_wisdom: wisdom };
    
    // Render the template to HTML and wrap it in an Html response
    // If rendering fails, provide a simple fallback HTML
    Html(template.render().unwrap_or_else(|_| {
        "<h1>Daily Wisdom</h1><p>Wisdom loading...</p>".to_string()
    }))
}

// Quantum Whispurrs page removed - replaced by Quantum Field

/// Handler function for the quantum field page (GET /quantum-field)
/// 
/// This function renders the 6-Fold Wisdom Field page that presents wisdom
/// in a structured field of six nodes representing different dimensions of awareness.
pub async fn quantum_field_page() -> Html<String> {
    // Log that we're rendering the quantum field page
    info!("Rendering quantum field page");
    
    // Create a template instance
    let template = QuantumFieldTemplate {};
    
    // Render the template to HTML and wrap it in an Html response
    // If rendering fails, provide a simple fallback HTML
    Html(template.render().unwrap_or_else(|_| {
        "<html><body><h1>The Enlightened Cat</h1><p>The quantum field collapsed unexpectedly. Please try again later.</p></body></html>".to_string()
    }))
}
