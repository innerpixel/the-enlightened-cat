use anyhow::Result;
use once_cell::sync::OnceCell;
use std::env;

static CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct Config {
    pub mistral_api_key: String,
    pub mistral_api_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn global() -> &'static Config {
        CONFIG.get().expect("Config not initialized")
    }

    pub fn init() -> Result<&'static Config> {
        let config = Config {
            mistral_api_key: env::var("MISTRAL_API_KEY")
                .expect("MISTRAL_API_KEY must be set"),
            mistral_api_url: env::var("MISTRAL_API_URL")
                .unwrap_or_else(|_| "https://api.mistral.ai/v1".to_string()),
            server_port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a number"),
        };

        CONFIG.set(config).expect("Failed to initialize config");
        Ok(CONFIG.get().unwrap())
    }
}
