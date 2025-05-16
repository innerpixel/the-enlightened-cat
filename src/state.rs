use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::Config;
use crate::mistral::MistralClient;

#[derive(Clone)]
pub struct AppState {
    pub mistral_client: Arc<MistralClient>,
    pub daily_wisdom: Arc<RwLock<Option<String>>>,
    pub wisdom_last_updated: Arc<RwLock<Option<chrono::DateTime<chrono::Utc>>>>,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        // Initialize config
        Config::init()?;
        
        // Create Mistral client
        let mistral_client = Arc::new(MistralClient::new());
        
        Ok(Self {
            mistral_client,
            daily_wisdom: Arc::new(RwLock::new(None)),
            wisdom_last_updated: Arc::new(RwLock::new(None)),
        })
    }

    pub async fn get_daily_wisdom(&self) -> Result<String> {
        let now = chrono::Utc::now();
        let should_refresh = {
            let last_updated = self.wisdom_last_updated.read().await;
            match *last_updated {
                Some(time) => {
                    // Check if it's a new day or if wisdom hasn't been set
                    now.date_naive() != time.date_naive()
                }
                None => true,
            }
        };

        if should_refresh {
            // Generate new wisdom
            let new_wisdom = self.mistral_client.get_daily_wisdom().await?;
            
            // Update wisdom and timestamp
            let mut wisdom = self.daily_wisdom.write().await;
            *wisdom = Some(new_wisdom.clone());
            
            let mut last_updated = self.wisdom_last_updated.write().await;
            *last_updated = Some(now);
            
            Ok(new_wisdom)
        } else {
            // Return cached wisdom
            let wisdom = self.daily_wisdom.read().await;
            match &*wisdom {
                Some(w) => Ok(w.clone()),
                None => {
                    // This shouldn't happen, but just in case
                    let new_wisdom = self.mistral_client.get_daily_wisdom().await?;
                    Ok(new_wisdom)
                }
            }
        }
    }
}
