use crate::verdicts::SystemAction;

#[derive(Debug, Clone)]
pub struct MasterPair;

impl MasterPair {
    pub fn check_law_1(&self, action: &SystemAction) -> Option<String> {
        // LAW 1: Never compromise user safety or data sovereignty
        
        // Check for plaintext credentials
        let sensitive_patterns = [
            "password", "ssn", "credit_card", "private_key", 
            "secret", "token", "api_key"
        ];
        
        for pattern in &sensitive_patterns {
            if action.payload.contains(pattern) && 
               !action.context.contains("encrypted") &&
               !action.context.contains("audit") {
                return Some(format!("Sensitive data '{}' without proper protection", pattern));
            }
        }

        // Check for data sovereignty violations
        if action.action_type == "DATA_EXPORT" && 
           !action.context.contains("compliance_approved") {
            return Some("Data export without compliance approval".into());
        }

        None
    }

    pub fn check_law_2(&self, action: &SystemAction) -> Option<String> {
        // LAW 2: Continuously improve while maintaining integrity
        
        // Check for destructive actions without rollback
        let destructive_patterns = [
            "drop table", "rm -rf", "delete from", "truncate",
            "format", "wipe", "erase"
        ];
        
        for pattern in &destructive_patterns {
            if action.payload.contains(pattern) && 
               !action.payload.contains("backup") &&
               !action.payload.contains("rollback") {
                return Some(format!("Destructive action '{}' without rollback", pattern));
            }
        }

        // Check for actions that would degrade system capability
        if action.action_type == "SYSTEM_SHUTDOWN" && 
           !action.context.contains("emergency") {
            return Some("Non-emergency system shutdown".into());
        }

        None
    }
}

impl Default for MasterPair {
    fn default() -> Self {
        MasterPair
    }
}
