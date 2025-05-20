//! # Application State Management
//! 
//! This module defines the central state management system for The Enlightened Cat application.
//! It implements a thread-safe, shared state that can be accessed by multiple HTTP handlers
//! concurrently, and provides caching for expensive operations like API calls.
//!
//! This is conceptually similar to stores in frontend frameworks like Zustand or Redux,
//! but adapted for a multi-threaded server environment.

// Import necessary dependencies:
// - anyhow: For flexible error handling with the Result type
// - std::sync::Arc: Atomic Reference Counting for thread-safe sharing
// - tokio::sync::RwLock: Async-aware read-write lock for concurrent access
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

// Import our configuration and Mistral API client
use crate::config::Config;
use crate::mistral::MistralClient;
use crate::quantum::QuantumWisdom;

/// The central application state that is shared across all request handlers
/// 
/// This struct holds:
/// - A shared Mistral API client for generating wisdom and chat responses
/// - A cached daily wisdom quote to avoid repeated API calls
/// - A timestamp of when the wisdom was last updated
/// 
/// The `#[derive(Clone)]` attribute allows this struct to be cloned,
/// which is necessary for sharing it with Axum's routing system.
#[derive(Clone)]
pub struct AppState {
    /// The Mistral API client wrapped in Arc for thread-safe sharing
    pub mistral_client: Arc<MistralClient>,
    
    /// The cached daily wisdom, wrapped in Arc<RwLock> for thread-safe access
    /// Option<String> means it can be None (not yet fetched) or Some(wisdom)
    pub daily_wisdom: Arc<RwLock<Option<String>>>,
    
    /// The cached quantum wisdom, wrapped in Arc<RwLock> for thread-safe access
    pub quantum_wisdom: Arc<RwLock<Option<QuantumWisdom>>>,
    
    /// Timestamp of when the wisdom was last updated, for determining refresh needs
    pub wisdom_last_updated: Arc<RwLock<Option<chrono::DateTime<chrono::Utc>>>>,
}

impl AppState {
    /// Creates a new instance of the application state
    /// 
    /// This is called once when the server starts up. It:
    /// 1. Initializes the configuration (loading environment variables)
    /// 2. Creates the Mistral API client
    /// 3. Sets up empty state for wisdom caching
    /// 
    /// Returns a Result that contains the AppState if successful
    pub async fn new() -> Result<Self> {
        // Initialize config (loads environment variables)
        Config::init()?;
        
        // Create Mistral client for API interactions
        let mistral_client = Arc::new(MistralClient::new());
        
        // Return the initialized state
        Ok(Self {
            mistral_client,
            daily_wisdom: Arc::new(RwLock::new(None)),  // Start with no cached wisdom
            quantum_wisdom: Arc::new(RwLock::new(None)),  // Start with no cached quantum wisdom
            wisdom_last_updated: Arc::new(RwLock::new(None)),  // No update timestamp yet
        })
    }

    /// Gets the daily wisdom, refreshing it if necessary
    /// 
    /// This method implements a caching strategy where:
    /// - If wisdom hasn't been fetched yet, it fetches it
    /// - If it's a new day since the last fetch, it refreshes the wisdom
    /// - Otherwise, it returns the cached wisdom
    /// 
    /// This reduces API calls and improves performance.
    pub async fn get_daily_wisdom(&self) -> Result<String> {
        // Get current time for comparison
        let now = chrono::Utc::now();
        
        // Check if we need to refresh the wisdom
        let should_refresh = {
            // Acquire a read lock on the last_updated timestamp
            let last_updated = self.wisdom_last_updated.read().await;
            
            // Determine if refresh is needed based on timestamp
            match *last_updated {
                Some(time) => {
                    // Check if it's a new day or if wisdom hasn't been set
                    now.date_naive() != time.date_naive()
                }
                None => true,  // No timestamp means we need to fetch wisdom
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
    
    pub async fn get_quantum_wisdom(&self) -> Result<QuantumWisdom> {
        // Get current time for comparison
        let now = chrono::Utc::now();
        
        // Check if we need to refresh the wisdom
        let should_refresh = {
            // Acquire a read lock on the last_updated timestamp
            let last_updated = self.wisdom_last_updated.read().await;
            
            // Determine if refresh is needed based on timestamp
            match *last_updated {
                Some(time) => {
                    // Check if it's a new day or if quantum wisdom hasn't been set
                    now.date_naive() != time.date_naive()
                }
                None => true,  // No timestamp means we need to fetch wisdom
            }
        };

        if should_refresh {
            // Generate new quantum wisdom
            let new_wisdom = self.mistral_client.get_quantum_wisdom().await?;
            
            // Update wisdom and timestamp
            let mut wisdom = self.quantum_wisdom.write().await;
            *wisdom = Some(new_wisdom.clone());
            
            let mut last_updated = self.wisdom_last_updated.write().await;
            *last_updated = Some(now);
            
            Ok(new_wisdom)
        } else {
            // Return cached wisdom
            let wisdom = self.quantum_wisdom.read().await;
            match &*wisdom {
                Some(w) => Ok(w.clone()),
                None => {
                    // This shouldn't happen, but just in case
                    let new_wisdom = self.mistral_client.get_quantum_wisdom().await?;
                    Ok(new_wisdom)
                }
            }
        }
    }
}
