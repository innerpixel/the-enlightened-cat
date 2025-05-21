//! # The Enlightened Cat - Main Application Entry Point
//! 
//! This is the entry point for The Enlightened Cat web application.
//! It sets up the web server, configures routes, initializes middleware,
//! and starts listening for HTTP requests.

// Import necessary dependencies:
// - anyhow: For flexible error handling with the Result type
// - axum: The web framework (similar to Express in Node.js)
// - tower_http: Middleware components for HTTP services
// - tracing_subscriber: For logging and diagnostics
use anyhow::Result;
use axum::{
    routing::{get, post},  // HTTP method handlers
    Router,                // Main router for defining routes
};
use std::net::SocketAddr;  // For defining the server's listening address
use tower_http::{
    cors::{Any, CorsLayer},  // Cross-Origin Resource Sharing middleware
    services::ServeDir,      // For serving static files (CSS, JS, images)
    trace::TraceLayer,       // For request/response tracing
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Import our application modules
mod config;    // Configuration management (environment variables)
mod mistral;   // Mistral AI API client
mod quantum_field; // Quantum field functionality
mod routes;    // HTTP route handlers
mod state;     // Application state management
mod templates; // HTML templates using Askama

/// Main application entry point
/// 
/// The #[tokio::main] attribute macro sets up the Tokio runtime for async/await.
/// This is similar to how you'd set up an event loop in Node.js.
#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    // Similar to dotenv in Node.js
    dotenv::dotenv().ok();
    
    // Initialize logging system
    // This sets up structured logging based on the RUST_LOG env var
    // Similar to Winston or Bunyan in Node.js
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    // Initialize application state (similar to creating a store in Redux/Zustand)
    // This creates our shared application state with the Mistral client
    let state = state::AppState::new().await?;
    
    // Build our application with routes
    // This is similar to defining routes in Express
    let app = Router::new()
        // API routes - JSON endpoints
        .route("/api/chat", post(routes::chat::handle_chat))             // POST /api/chat - Chat with the cat
        .route("/api/daily-wisdom", get(routes::wisdom::get_daily_wisdom)) // GET /api/daily-wisdom - Get wisdom as JSON
        .route("/api/quantum-field", get(routes::quantum_field::get_quantum_field)) // GET /api/quantum-field - Get quantum field
        .route("/api/quantum-field/collapse", get(routes::quantum_field::collapse_quantum_field)) // GET /api/quantum-field/collapse - Collapse quantum field
        
        // Page routes - HTML endpoints
        .route("/", get(routes::pages::index))           // GET / - Home page
        .route("/about", get(routes::pages::about))       // GET /about - About page
        .route("/wisdom", get(routes::pages::wisdom_page)) // GET /wisdom - Daily wisdom page
        .route("/quantum-field", get(routes::pages::quantum_field_page)) // GET /quantum-field - Quantum field page
        
        // Serve static files (CSS, JS, images)
        // Similar to express.static in Node.js
        .nest_service("/static", ServeDir::new("static"))
        
        // Add middleware
        .layer(TraceLayer::new_for_http())  // Add request/response logging
        .layer(
            CorsLayer::new()                // Configure CORS policy
                .allow_origin(Any)          // Allow any origin (can be restricted in production)
                .allow_methods(Any)         // Allow any HTTP method
                .allow_headers(Any),        // Allow any headers
        )
        .with_state(state);  // Attach our application state to the router
    
    // Get the server port from configuration
    // This comes from the PORT environment variable
    let port = config::Config::global().server_port;
    
    // Create a socket address to listen on all interfaces
    // 0.0.0.0 means "listen on all available network interfaces"
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Listening on {}", addr);
    
    // Start the HTTP server
    // This is similar to app.listen() in Express
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}
