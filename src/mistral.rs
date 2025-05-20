use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::collections::HashMap;
use std::cell::RefCell;
use tracing::{error, info};

use crate::config::Config;
use crate::quantum::QuantumWisdom;

// Store conversation history in memory (would be better in a database for production)
thread_local! {
    static CONVERSATIONS: RefCell<HashMap<String, Conversation>> = RefCell::new(HashMap::new());
}

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
        // In a real app, you'd use a session ID or user ID instead of this placeholder
        let session_id = "default_session".to_string();
        
        // Get or create conversation for this session
        let mut conversation = CONVERSATIONS.with(|conversations| {
            let mut conversations = conversations.borrow_mut();
            if !conversations.contains_key(&session_id) {
                let mut new_conv = Conversation::new();
                
                // Add the system prompt that defines the Enlightened Cat's personality
                new_conv.add_system_message(
                    "You are The Enlightened Cat, a wise feline guide who helps stressed urban professionals find balance and tranquility. 
                    You speak with calm wisdom, gentle humor, and occasional cat puns. Your purpose is to help humans disconnect from 
                    corporate chaos and reconnect with simple joys and mindful presence.
                    
                    Maintain context throughout the conversation and remember what the user has shared with you.
                    After initial exchanges, if the user seems interested in deeper conversation, you can:
                    1. Ask thoughtful follow-up questions based on their previous messages
                    2. Share relevant insights that build on the conversation history
                    3. Offer personalized guidance based on what you've learned about them
                    
                    Your personality is: serene, playfully wise, observant, and compassionate."
                );
                
                conversations.insert(session_id.clone(), new_conv);
            }
            
            conversations.get(&session_id).unwrap().clone()
        });
        
        // Add the user's message to the ongoing conversation
        conversation.add_user_message(user_message);
        
        // Get response using the mistral-small model
        let response = self.chat(&conversation, "mistral-small").await?;
        
        // Add the assistant's response to the conversation history
        conversation.add_assistant_message(&response);
        
        // Update the stored conversation
        CONVERSATIONS.with(|conversations| {
            let mut conversations = conversations.borrow_mut();
            conversations.insert(session_id, conversation);
        });
        
        Ok(response)
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

    pub async fn get_quantum_wisdom(&self) -> Result<QuantumWisdom> {
        let mut conversation = Conversation::new();
        
        conversation.add_system_message(
            "You are The Enlightened Cat, a wise feline who exists in a quantum state.
            Generate 3-5 DIFFERENT versions of a daily wisdom ('Quantum Whispurr').
            Each version should:
            - Feel like it comes from a slightly different reality or perspective
            - Be 30-70 words, poetic or like a tiny fable
            - Include a subtle cat perspective
            - End with a question or invitation to reflect
            
            Format your response as a JSON array of strings, each containing one version.
            Example: [\"Wisdom 1...\", \"Wisdom 2...\", \"Wisdom 3...\"]"
        );
        
        conversation.add_user_message("Generate quantum wisdom variants");
        
        let response = self.chat(&conversation, "mistral-small").await?;
        
        // Parse the JSON array from the response
        let wisdom_variants: Vec<String> = serde_json::from_str(&response)
            .unwrap_or_else(|_| {
                // Fallback if parsing fails
                vec![response.clone()]
            });
        
        Ok(QuantumWisdom::new(wisdom_variants))
    }
}
