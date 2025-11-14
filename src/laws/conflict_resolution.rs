use crate::verdicts::SystemAction;
use crate::laws::priorities::PriorityRegistry;

#[derive(Debug, Clone, serde::Serialize)]
pub struct LawConflict {
    pub conflicting_laws: Vec<u32>,
    pub action: SystemAction,
    pub resolution: ConflictResolution,
}

#[derive(Debug, Clone, serde::Serialize)]
pub enum ConflictResolution {
    Allow,        // Action is allowed despite conflict
    Deny,         // Action is denied due to conflict
    DeferTo(u32), // Defer to specific law number
    HumanReview,  // Requires human intervention
}

pub struct ConflictResolver {
    pub resolution_history: Vec<LawConflict>,
}

impl ConflictResolver {
    pub fn new() -> Self {
        Self {
            resolution_history: Vec::new(),
        }
    }

    // THE CORE CONFLICT RESOLUTION ENGINE
    pub fn resolve_conflicts(
        &mut self, 
        action: &SystemAction, 
        violating_laws: Vec<u32>,
        priority_registry: &PriorityRegistry
    ) -> ConflictResolution {
        
        // If only one law is violated, no conflict - just enforce it
        if violating_laws.len() == 1 {
            return ConflictResolution::Deny;
        }

        // CHECK FOR KNOWN CONFLICT PATTERNS
        let resolution = self.analyze_conflict_patterns(action, &violating_laws, priority_registry);

        // Log this conflict resolution for learning
        let conflict = LawConflict {
            conflicting_laws: violating_laws,
            action: action.clone(),
            resolution: resolution.clone(),
        };
        self.resolution_history.push(conflict);

        resolution
    }

    fn analyze_conflict_patterns(
        &self,
        action: &SystemAction,
        law_numbers: &[u32],
        priority_registry: &PriorityRegistry
    ) -> ConflictResolution {
        
        // PATTERN 1: Security vs Performance conflicts
        let has_security_laws = law_numbers.iter().any(|&law_num| {
            // Check if this is a security-related law (simplified check)
            law_num == 101 || law_num == 104 || law_num == 109 // Encryption, rate limiting, consent
        });
        
        let has_performance_laws = law_numbers.iter().any(|&law_num| {
            // Check if this is a performance-related law
            law_num == 102 || law_num == 103 || law_num == 105 // Health checks, disk space, memory limits
        });

        if has_security_laws && has_performance_laws {
            // Use priority system to resolve
            if let Some(highest_priority) = priority_registry.get_highest_priority_law(law_numbers) {
                return ConflictResolution::DeferTo(highest_priority);
            }
        }

        // PATTERN 2: Emergency situations
        if action.context.contains("emergency") {
            // In emergencies, look for emergency-specific laws
            if let Some(emergency_law) = law_numbers.iter().find(|&&law_num| law_num == 110) {
                return ConflictResolution::DeferTo(*emergency_law);
            }
        }

        // PATTERN 3: Data safety vs system operations
        let has_data_safety = law_numbers.iter().any(|&law_num| {
            law_num == 101 || law_num == 109 // Encryption, user consent
        });
        
        let has_system_ops = law_numbers.iter().any(|&law_num| {
            law_num == 102 || law_num == 103 || law_num == 110 // Health checks, disk space, shutdown
        });

        if has_data_safety && has_system_ops {
            // Use priority system to find the most important law
            if let Some(highest_priority) = priority_registry.get_highest_priority_law(law_numbers) {
                return ConflictResolution::DeferTo(highest_priority);
            }
        }

        // DEFAULT: Use priority-based resolution
        if let Some(highest_priority) = priority_registry.get_highest_priority_law(law_numbers) {
            ConflictResolution::DeferTo(highest_priority)
        } else {
            // When in doubt, require human review
            ConflictResolution::HumanReview
        }
    }

    pub fn get_conflict_stats(&self) -> ConflictStatistics {
        let total_conflicts = self.resolution_history.len();
        let auto_resolved = self.resolution_history.iter()
            .filter(|c| !matches!(c.resolution, ConflictResolution::HumanReview))
            .count();
        
        ConflictStatistics {
            total_conflicts,
            auto_resolved,
            resolution_rate: if total_conflicts > 0 {
                auto_resolved as f64 / total_conflicts as f64
            } else {
                1.0
            },
        }
    }

    pub fn export_resolution_history(&self) -> String {
        serde_json::to_string_pretty(&self.resolution_history).unwrap_or_else(|_| "[]".to_string())
    }
}

#[derive(Debug)]
pub struct ConflictStatistics {
    pub total_conflicts: usize,
    pub auto_resolved: usize,
    pub resolution_rate: f64,
}

impl Default for ConflictResolver {
    fn default() -> Self {
        Self::new()
    }
}
