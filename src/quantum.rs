use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

/// Represents a wisdom that exists in multiple quantum states
/// until it's observed by a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumWisdom {
    /// The possible states this wisdom can collapse into
    pub potential_states: Vec<String>,
    
    /// The collapsed state (only set after observation)
    pub collapsed_state: Option<String>,
    
    /// Whether this wisdom has been observed
    pub observed: bool,
    
    /// Metadata about the quantum state
    pub quantum_metadata: QuantumMetadata,
}

/// Metadata about the quantum properties of the wisdom
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumMetadata {
    /// Coherence level (0.0-1.0) affects stability of states
    pub coherence: f32,
    
    /// Entanglement ID for wisdoms that affect each other
    pub entanglement_id: Option<String>,
    
    /// Quantum phenomena exhibited by this wisdom
    pub phenomena: Vec<QuantumPhenomenon>,
}

/// Different quantum phenomena that can affect wisdom
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuantumPhenomenon {
    Superposition,
    Entanglement,
    QuantumTunneling,
    WaveParticleDuality,
    SchrodingerEffect,
}

impl QuantumWisdom {
    /// Create a new quantum wisdom with multiple potential states
    pub fn new(potential_states: Vec<String>) -> Self {
        Self {
            potential_states,
            collapsed_state: None,
            observed: false,
            quantum_metadata: QuantumMetadata {
                coherence: rand::random::<f32>(),
                entanglement_id: None,
                phenomena: vec![QuantumPhenomenon::Superposition],
            },
        }
    }
    
    /// Observe the wisdom, causing it to collapse into a single state randomly
    pub fn observe(&mut self) -> &str {
        if !self.observed {
            // Collapse the quantum state based on coherence and other factors
            let chosen = self.potential_states
                .choose(&mut rand::thread_rng())
                .cloned()
                .unwrap_or_else(|| "The quantum state collapsed unexpectedly.".to_string());
            
            self.collapsed_state = Some(chosen);
            self.observed = true;
        }
        
        self.collapsed_state.as_ref().unwrap()
    }
    
    /// Observe a specific state by index
    pub fn observe_specific(&mut self, index: usize) -> &str {
        if !self.observed && index < self.potential_states.len() {
            // Collapse to the specific state
            let chosen = self.potential_states[index].clone();
            self.collapsed_state = Some(chosen);
            self.observed = true;
        } else if !self.observed {
            // If index is invalid, fall back to random
            return self.observe();
        }
        
        self.collapsed_state.as_ref().unwrap()
    }
    
    /// Check if this wisdom is in a superposition state
    pub fn is_in_superposition(&self) -> bool {
        !self.observed
    }
    
    /// Get all potential states for display
    pub fn get_potential_states(&self) -> &Vec<String> {
        &self.potential_states
    }
}
