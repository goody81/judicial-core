use crate::laws::MasterPair;
use crate::verdicts::{Verdict, SystemAction};
use crate::ledger::TamperProofLedger;
use std::sync::RwLock;

#[derive(Debug)]
pub struct JudicialCore {
    master_pair: MasterPair,
    ledger: RwLock<TamperProofLedger>,
}

impl JudicialCore {
    pub fn new() -> Self {
        Self {
            master_pair: MasterPair::default(),
            ledger: RwLock::new(TamperProofLedger::new()),
        }
    }

    pub fn rule(&self, action: SystemAction) -> Verdict {
        // Law 1: Safety & Sovereignty - ABSOLUTE
        if let Some(violation) = self.master_pair.check_law_1(&action) {
            self.log_violation(action, violation.clone());
            return Verdict::Rejected(violation);
        }

        // Law 2: Improvement & Integrity - STRICT  
        if let Some(violation) = self.master_pair.check_law_2(&action) {
            self.log_violation(action, violation.clone());
            return Verdict::RejectedWithSuggestion(
                violation, 
                "Provide rollback mechanism or sandbox execution.".into()
            );
        }

        // Action is lawful
        self.log_approval(action);
        Verdict::Approved
    }

    pub fn get_compliance_score(&self) -> f64 {
        let ledger = self.ledger.read().unwrap();
        ledger.calculate_compliance_score()
    }

    pub fn export_ledger(&self) -> String {
        let ledger = self.ledger.read().unwrap();
        serde_json::to_string_pretty(ledger.entries()).unwrap()
    }

    fn log_violation(&self, action: SystemAction, reason: String) {
        let mut ledger = self.ledger.write().unwrap();
        ledger.record_violation(action, reason);
    }

    fn log_approval(&self, action: SystemAction) {
        let mut ledger = self.ledger.write().unwrap();
        ledger.record_approval(action);
    }
}

impl Default for JudicialCore {
    fn default() -> Self {
        Self::new()
    }
}
