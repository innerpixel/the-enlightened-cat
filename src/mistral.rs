use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::collections::HashMap;
use std::cell::RefCell;
use tracing::{error, info};

use crate::config::Config;
use crate::quantum_field::QuantumField;

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

    // Quantum Wisdom method removed - replaced by Quantum Field
    
    pub async fn get_quantum_field(&self) -> Result<QuantumField> {
        let mut conversation = Conversation::new();
        
        conversation.add_system_message(
            "You are The Enlightened Cat, a wise feline who understands quantum physics and spiritual wisdom.
            Create six poetic fragments of wisdom representing symbolic domains:
            1. Essence - Core truth or soul resonance
            2. Inner Path - Internal reflection, personal myth
            3. Outer Path - Action or movement in the world
            4. Portal - Invitation, threshold, or call
            5. Friction - Challenge, tension, or transformation
            6. Crystallization - Integration, revelation, or clarity
            
            Each fragment should be:
            - Short (10-20 words)
            - Evocative and open-ended—like a seed
            - Poetic and mysterious
            - Suitable for visualization
            - Containing subtle feline wisdom
            
            Format your response as a JSON array of 6 strings, each containing one wisdom fragment.
            Example: [\"Fragment 1...\", \"Fragment 2...\", \"Fragment 3...\", \"Fragment 4...\", \"Fragment 5...\", \"Fragment 6...\"]"
        );
        
        conversation.add_user_message("Generate six wisdom fragments for the quantum field");
        
        let response = self.chat(&conversation, "mistral-small").await?;
        
        // Parse the JSON array from the response
        let wisdom_seeds: Vec<String> = serde_json::from_str(&response)
            .unwrap_or_else(|_| {
                // Fallback if parsing fails - generate 6 placeholder seeds
                vec![
                    "A single note played in the silent forest".to_string(),
                    "The mirror ripples but does not break".to_string(),
                    "Footsteps echo through the sky-bound stair".to_string(),
                    "The door hums though no hand touches it".to_string(),
                    "Ashes glowing under the weight of stillness".to_string(),
                    "The gem turns inside the breathless hour".to_string()
                ]
            });
        
        // Ensure we have exactly 6 seeds
        let seeds = if wisdom_seeds.len() < 6 {
            // Pad with defaults if we have fewer than 6
            let mut seeds = wisdom_seeds;
            let defaults = vec![
                "A single note played in the silent forest".to_string(),
                "The mirror ripples but does not break".to_string(),
                "Footsteps echo through the sky-bound stair".to_string(),
                "The door hums though no hand touches it".to_string(),
                "Ashes glowing under the weight of stillness".to_string(),
                "The gem turns inside the breathless hour".to_string()
            ];
            
            for i in seeds.len()..6 {
                seeds.push(defaults[i % defaults.len()].clone());
            }
            seeds
        } else if wisdom_seeds.len() > 6 {
            // Truncate if we have more than 6
            wisdom_seeds[0..6].to_vec()
        } else {
            // Use as-is if we have exactly 6
            wisdom_seeds
        };
        
        Ok(QuantumField::new(seeds))
    }
}
