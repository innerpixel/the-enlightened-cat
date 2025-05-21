use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

/// Represents a wisdom node in the 6-fold field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WisdomNode {
    /// The index of this node (1-6)
    pub index: usize,
    
    /// The domain this node represents
    pub domain: String,
    
    /// The seed wisdom fragment
    pub seed: String,
}

/// Represents the entire 6-fold wisdom field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumField {
    /// The 6 wisdom nodes forming the field
    pub wisdom_field: Vec<WisdomNode>,
    
    /// The selected node index (if any)
    pub selected_index: Option<usize>,
    
    /// The collapsed prompt generated from the selected node
    pub collapsed_prompt: Option<String>,
}

/// Domain definitions for the 6-fold field
pub const DOMAINS: [&str; 6] = [
    "Essence",         // Core truth or soul resonance
    "Inner Path",      // Internal reflection, personal myth
    "Outer Path",      // Action or movement in the world
    "Portal",          // Invitation, threshold, or call
    "Friction",        // Challenge, tension, or transformation
    "Crystallization", // Integration, revelation, or clarity
];

impl QuantumField {
    /// Create a new quantum field with 6 wisdom nodes
    pub fn new(seeds: Vec<String>) -> Self {
        let mut wisdom_field = Vec::new();
        
        // Create the 6 wisdom nodes
        for (i, seed) in seeds.iter().enumerate() {
            wisdom_field.push(WisdomNode {
                index: i + 1,
                domain: DOMAINS[i].to_string(),
                seed: seed.clone(),
            });
        }
        
        Self {
            wisdom_field,
            selected_index: None,
            collapsed_prompt: None,
        }
    }
    
    /// Collapse the field by selecting a specific node
    pub fn collapse(&mut self, index: usize) -> &str {
        if index < self.wisdom_field.len() {
            self.selected_index = Some(index);
            
            // Generate a collapsed prompt based on the selected node
            let selected_node = &self.wisdom_field[index];
            let prompt = self.generate_collapsed_prompt(selected_node);
            
            self.collapsed_prompt = Some(prompt);
        } else {
            // If index is invalid, generate a default prompt
            self.collapsed_prompt = Some("The quantum field collapsed in an unexpected way.".to_string());
        }
        
        self.collapsed_prompt.as_ref().unwrap()
    }
    
    /// Generate a collapsed prompt based on the selected node
    fn generate_collapsed_prompt(&self, node: &WisdomNode) -> String {
        // In a real implementation, this would be more sophisticated
        // For now, we'll create a simple visual prompt based on the seed
        match node.domain.as_str() {
            "Essence" => format!(
                "A luminous core of energy revealing {}, with soft golden light emanating outward, creating a sense of profound truth and connection.", 
                node.seed
            ),
            "Inner Path" => format!(
                "A winding path through a misty forest where {}, illuminated by moonlight filtering through ancient trees, suggesting introspection and personal discovery.", 
                node.seed
            ),
            "Outer Path" => format!(
                "A figure standing at a crossroads where {}, with multiple paths stretching toward distant horizons, symbolizing choices and action in the world.", 
                node.seed
            ),
            "Portal" => format!(
                "A mysterious doorway glowing with ethereal light, where {}. The threshold vibrates with possibility, inviting passage to another realm.", 
                node.seed
            ),
            "Friction" => format!(
                "Two opposing forces meeting in a dance of energy where {}, creating sparks and transformation at their intersection, beautiful in their tension.", 
                node.seed
            ),
            "Crystallization" => format!(
                "A perfect crystal forming in slow motion, capturing the moment when {}. Each facet reflects a different aspect of the same unified truth.", 
                node.seed
            ),
            _ => format!("A visual representation of: {}", node.seed),
        }
    }
    
    /// Get all wisdom nodes in the field
    pub fn get_wisdom_field(&self) -> &Vec<WisdomNode> {
        &self.wisdom_field
    }
}
