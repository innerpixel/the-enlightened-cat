use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{error, info};

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct MistralClient {
    client: reqwest::Client,
    api_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponseChoice {
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatResponseChoice>,
}

#[derive(Debug, Clone)]
pub struct Conversation {
    pub messages: Vec<ChatMessage>,
}

impl Conversation {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, role: &str, content: &str) {
        self.messages.push(ChatMessage {
            role: role.to_string(),
            content: content.to_string(),
        });
    }

    pub fn add_system_message(&mut self, content: &str) {
        self.add_message("system", content);
    }

    pub fn add_user_message(&mut self, content: &str) {
        self.add_message("user", content);
    }

    pub fn add_assistant_message(&mut self, content: &str) {
        self.add_message("assistant", content);
    }
}

impl MistralClient {
    pub fn new() -> Self {
        let config = Config::global();
        let mut headers = HeaderMap::new();
        
        let auth_value = format!("Bearer {}", config.mistral_api_key);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_value).expect("Invalid API key format"),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            api_url: config.mistral_api_url.clone(),
        }
    }

    pub async fn chat(&self, conversation: &Conversation, model: &str) -> Result<String> {
        let request = ChatRequest {
            model: model.to_string(),
            messages: conversation.messages.clone(),
            temperature: Some(0.7),
            max_tokens: Some(500),
        };

        info!("Sending request to Mistral API");
        
        let response = self.client
            .post(&format!("{}/chat/completions", self.api_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Mistral API error: {}", error_text);
            return Err(anyhow::anyhow!("Mistral API error: {}", error_text));
        }

        let chat_response: ChatResponse = response.json().await?;
        
        if let Some(choice) = chat_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(anyhow::anyhow!("No response from Mistral API"))
        }
    }

    pub async fn get_enlightened_cat_response(&self, user_message: &str) -> Result<String> {
        let mut conversation = Conversation::new();
        
        // Add the system prompt that defines the Enlightened Cat's personality
        conversation.add_system_message(
            "You are The Enlightened Cat, a wise feline guide who helps stressed urban professionals find balance and tranquility. 
            You speak with calm wisdom, gentle humor, and occasional cat puns. Your purpose is to help humans disconnect from 
            corporate chaos and reconnect with simple joys and mindful presence.
            
            Keep your wisdom concise (50-100 words). Include a small, practical suggestion at the end.
            Occasionally reference your experiences observing humans in 'the corporate jungle'.
            Use language that evokes peaceful imagery. End conversations with an open question that encourages reflection.
            
            Your personality is: serene, playfully wise, observant, and compassionate."
        );
        
        // Add the user's message
        conversation.add_user_message(user_message);
        
        // Get response using the mistral-small model
        self.chat(&conversation, "mistral-small").await
    }

    pub async fn get_daily_wisdom(&self) -> Result<String> {
        let mut conversation = Conversation::new();
        
        conversation.add_system_message(
            "You are The Enlightened Cat, a wise and mysterious feline guide who helps stressed professionals reconnect with presence.
            Generate a short yet surprising reflection (a 'Daily Whispurr') designed to interrupt the ordinary mind and invite deeper thought.
            It should:
            - Feel slightly mysterious, poetic, or like a tiny fable or riddle.
            - Be 30-70 words, enough to tell a small story or offer an enigma.
            - Include a subtle cat or feline perspective.
            - End with a question or invitation to reflect."
        );
        
        conversation.add_user_message("Please provide today's Daily Whispurr meditation.");
        
        self.chat(&conversation, "mistral-small").await
    }
}
