use serde::{Deserialize, Serialize};

// First, let's define the LawCategory enum that priorities.rs needs
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum LawCategory {
    DataGovernance,      // Laws about data handling, privacy, sovereignty
    AgentBehavior,       // Laws about how agents should behave
    SystemOperations,    // Laws about system maintenance and operations  
    SecurityProtocols,   // Laws about security and protection
    ResourceManagement,  // Laws about CPU, memory, storage usage
    CommunicationEthics, // Laws about how agents communicate
    LearningEvolution,   // Laws about self-improvement and adaptation
    ErrorHandling,       // Laws about failure modes and recovery
    UserInteraction,     // Laws about human-AI interaction
    EmergencyProtocols,  // Laws for crisis situations
}

impl LawCategory {
    pub fn description(&self) -> &'static str {
        match self {
            Self::DataGovernance => "Governs data handling, privacy, and sovereignty",
            Self::AgentBehavior => "Defines expected agent behaviors and interactions",
            Self::SystemOperations => "Rules for system maintenance and health",
            Self::SecurityProtocols => "Security, protection, and threat response",
            Self::ResourceManagement => "CPU, memory, storage allocation and limits",
            Self::CommunicationEthics => "How agents communicate internally and externally",
            Self::LearningEvolution => "Self-improvement, adaptation, and growth",
            Self::ErrorHandling => "Failure recovery and error management", 
            Self::UserInteraction => "Human-AI interaction protocols",
            Self::EmergencyProtocols => "Crisis response and emergency procedures",
        }
    }
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]  // ← ADD Copy HERE
pub enum PriorityLevel {
    Critical = 100,    // Safety, security, core integrity
    High = 80,         // Data protection, major operations  
    Medium = 60,       // Performance, resource management
    Low = 40,          // Optimization, non-critical features
    Advisory = 20,     // Best practices, recommendations
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LawPriority {
    pub law_number: u32,
    pub priority: PriorityLevel,
    pub category: LawCategory,
    pub weight: f64,  // 0.0 to 1.0 - fine-grained control
}

pub struct PriorityRegistry {
    pub priorities: Vec<LawPriority>,
    pub category_weights: std::collections::HashMap<LawCategory, f64>,
}

impl PriorityRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            priorities: Vec::new(),
            category_weights: std::collections::HashMap::new(),
        };
        
        // SET DEFAULT CATEGORY WEIGHTS
        // These weights multiply the individual law priorities
        registry.category_weights.insert(LawCategory::SecurityProtocols, 1.2);    // Security is 20% more important
        registry.category_weights.insert(LawCategory::DataGovernance, 1.1);       // Data protection is 10% more important
        registry.category_weights.insert(LawCategory::EmergencyProtocols, 1.5);   // Emergencies are 50% more important
        registry.category_weights.insert(LawCategory::SystemOperations, 1.0);     // Operations are baseline
        registry.category_weights.insert(LawCategory::AgentBehavior, 0.9);        // Behavior is slightly less critical
        registry.category_weights.insert(LawCategory::ResourceManagement, 0.8);   // Resources are important but flexible
        registry.category_weights.insert(LawCategory::CommunicationEthics, 0.7);  // Communication is important but not critical
        registry.category_weights.insert(LawCategory::LearningEvolution, 0.6);    // Learning can be delayed if needed
        registry.category_weights.insert(LawCategory::ErrorHandling, 1.0);        // Error handling is baseline important
        registry.category_weights.insert(LawCategory::UserInteraction, 0.8);      // User experience is important but flexible

        // INITIALIZE WITH OUR FIRST 10 LAWS AND THEIR PRIORITIES
        registry.initialize_default_priorities();
        
        registry
    }

    fn initialize_default_priorities(&mut self) {
        // LAW 101: Data encryption - CRITICAL (security + data protection)
        self.priorities.push(LawPriority {
            law_number: 101,
            priority: PriorityLevel::Critical,
            category: LawCategory::DataGovernance,
            weight: 0.95, // Very important within Critical tier
        });

        // LAW 102: Health checks - HIGH (system operations)
        self.priorities.push(LawPriority {
            law_number: 102,
            priority: PriorityLevel::High,
            category: LawCategory::SystemOperations,
            weight: 0.8,
        });

        // LAW 103: Disk space - MEDIUM (resource management)
        self.priorities.push(LawPriority {
            law_number: 103,
            priority: PriorityLevel::Medium,
            category: LawCategory::ResourceManagement,
            weight: 0.7,
        });

        // LAW 104: Rate limiting - HIGH (security + performance)
        self.priorities.push(LawPriority {
            law_number: 104,
            priority: PriorityLevel::High,
            category: LawCategory::SecurityProtocols,
            weight: 0.85,
        });

        // LAW 105: Memory limits - MEDIUM (resource management)
        self.priorities.push(LawPriority {
            law_number: 105,
            priority: PriorityLevel::Medium,
            category: LawCategory::ResourceManagement,
            weight: 0.6,
        });

        // LAW 106: Message acknowledgment - MEDIUM (communication)
        self.priorities.push(LawPriority {
            law_number: 106,
            priority: PriorityLevel::Medium,
            category: LawCategory::CommunicationEthics,
            weight: 0.5,
        });

        // LAW 107: Sandbox testing - HIGH (safety + learning)
        self.priorities.push(LawPriority {
            law_number: 107,
            priority: PriorityLevel::High,
            category: LawCategory::LearningEvolution,
            weight: 0.9,
        });

        // LAW 108: Error logging - MEDIUM (error handling)
        self.priorities.push(LawPriority {
            law_number: 108,
            priority: PriorityLevel::Medium,
            category: LawCategory::ErrorHandling,
            weight: 0.6,
        });

        // LAW 109: User consent - CRITICAL (data protection + ethics)
        self.priorities.push(LawPriority {
            law_number: 109,
            priority: PriorityLevel::Critical,
            category: LawCategory::DataGovernance,
            weight: 0.98, // Almost absolute priority
        });

        // LAW 110: Emergency shutdown - CRITICAL (emergency protocols)
        self.priorities.push(LawPriority {
            law_number: 110,
            priority: PriorityLevel::Critical,
            category: LawCategory::EmergencyProtocols,
            weight: 0.99, // Extremely high priority
        });
    }

    pub fn get_priority_score(&self, law_number: u32) -> f64 {
        if let Some(priority) = self.priorities.iter().find(|p| p.law_number == law_number) {
            let base_score = priority.priority as u32 as f64;
            let category_multiplier = self.category_weights.get(&priority.category).unwrap_or(&1.0);
            let weighted_score = base_score * category_multiplier * priority.weight;
            
            weighted_score
        } else {
            // Default priority for unknown laws
            PriorityLevel::Medium as u32 as f64
        }
    }

    pub fn compare_priorities(&self, law_a: u32, law_b: u32) -> std::cmp::Ordering {
        let score_a = self.get_priority_score(law_a);
        let score_b = self.get_priority_score(law_b);
        
        score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
    }

    pub fn get_highest_priority_law(&self, law_numbers: &[u32]) -> Option<u32> {
        law_numbers.iter()
            .max_by(|a, b| self.compare_priorities(**a, **b))
            .copied()
    }

    pub fn update_priority(&mut self, law_number: u32, new_priority: PriorityLevel, new_weight: f64) -> bool {
        if let Some(priority) = self.priorities.iter_mut().find(|p| p.law_number == law_number) {
            priority.priority = new_priority;
            priority.weight = new_weight.max(0.0).min(1.0);
            true
        } else {
            false
        }
    }

    pub fn add_custom_priority(&mut self, law_priority: LawPriority) {
        // Remove existing priority for this law if it exists
        self.priorities.retain(|p| p.law_number != law_priority.law_number);
        self.priorities.push(law_priority);
    }

    pub fn export_priorities(&self) -> String {
        let mut priorities_json = serde_json::Map::new();
        
        for priority in &self.priorities {
            let law_key = format!("law_{}", priority.law_number);
            let priority_data = serde_json::json!({
                "priority": format!("{:?}", priority.priority),
                "category": format!("{:?}", priority.category),
                "weight": priority.weight,
                "score": self.get_priority_score(priority.law_number)
            });
            
            priorities_json.insert(law_key, priority_data);
        }
        
        serde_json::to_string_pretty(&priorities_json).unwrap()
    }
}

impl Default for PriorityRegistry {
    fn default() -> Self {
        Self::new()
    }
}
