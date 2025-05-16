use axum::{
    extract::State,
    response::Html,
};
use askama::Template;
use tracing::info;

use crate::state::AppState;
use crate::templates::*;

pub async fn index(State(state): State<AppState>) -> Html<String> {
    info!("Rendering index page");
    
    let wisdom = state.get_daily_wisdom().await.unwrap_or_else(|_| {
        "Even in moments of technical difficulty, the enlightened cat remains calm and patient.".to_string()
    });
    
    let template = IndexTemplate { daily_wisdom: wisdom };
    Html(template.render().unwrap_or_else(|_| {
        "<h1>The Enlightened Cat</h1><p>Wisdom loading...</p>".to_string()
    }))
}

pub async fn about() -> Html<String> {
    info!("Rendering about page");
    
    let template = AboutTemplate {};
    Html(template.render().unwrap_or_else(|_| {
        "<h1>About The Enlightened Cat</h1><p>Content loading...</p>".to_string()
    }))
}

pub async fn wisdom_page(State(state): State<AppState>) -> Html<String> {
    info!("Rendering wisdom page");
    
    let wisdom = state.get_daily_wisdom().await.unwrap_or_else(|_| {
        "Even in moments of technical difficulty, the enlightened cat remains calm and patient.".to_string()
    });
    
    let template = WisdomTemplate { daily_wisdom: wisdom };
    Html(template.render().unwrap_or_else(|_| {
        "<h1>Daily Wisdom</h1><p>Wisdom loading...</p>".to_string()
    }))
}
