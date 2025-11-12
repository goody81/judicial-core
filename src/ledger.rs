use crate::verdicts::SystemAction;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct LedgerEntry {
    pub timestamp: DateTime<Utc>,
    pub action: SystemAction,
    pub verdict: String,
    pub hash: String,
    pub previous_hash: Option<String>,
}

#[derive(Debug)]
pub struct TamperProofLedger {
    entries: Vec<LedgerEntry>,
}

impl TamperProofLedger {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn record_violation(&mut self, action: SystemAction, reason: String) {
        self.record_entry(action, format!("REJECTED: {}", reason));
    }

    pub fn record_approval(&mut self, action: SystemAction) {
        self.record_entry(action, "APPROVED".into());
    }

    fn record_entry(&mut self, action: SystemAction, verdict: String) {
        let timestamp = Utc::now();
        let previous_hash = self.entries.last().map(|e| e.hash.clone());
        
        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}{:?}{:?}", timestamp, action, verdict).as_bytes());
        if let Some(prev_hash) = &previous_hash {
            hasher.update(prev_hash.as_bytes());
        }
        let hash = format!("{:x}", hasher.finalize());

        let entry = LedgerEntry {
            timestamp,
            action,
            verdict,
            hash,
            previous_hash,
        };

        self.entries.push(entry);
    }

    pub fn calculate_compliance_score(&self) -> f64 {
        if self.entries.is_empty() {
            return 1.0;
        }

        let approved_count = self.entries.iter()
            .filter(|e| e.verdict.starts_with("APPROVED"))
            .count();

        approved_count as f64 / self.entries.len() as f64
    }

    pub fn entries(&self) -> &Vec<LedgerEntry> {
        &self.entries
    }
}
