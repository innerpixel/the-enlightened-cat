use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod mistral;
mod routes;
mod state;
mod templates;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();
    
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    // Initialize application state
    let state = state::AppState::new().await?;
    
    // Build our application with routes
    let app = Router::new()
        // API routes
        .route("/api/chat", post(routes::chat::handle_chat))
        .route("/api/daily-wisdom", get(routes::wisdom::get_daily_wisdom))
        
        // Static routes for templates
        .route("/", get(routes::pages::index))
        .route("/about", get(routes::pages::about))
        .route("/wisdom", get(routes::pages::wisdom_page))
        
        // Serve static files
        .nest_service("/static", ServeDir::new("static"))
        
        // Add middleware
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state);
    
    // Run the server
    let port = config::Config::global().server_port;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}
