use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::blue_whale_sleep::{BlueWhaleSleep, SleepState, SystemHealth};
use crate::judicial_core::JudicialCore;
use crate::verdicts::SystemAction;

// TRAIT FOR MEMORY SYSTEMS - This lets us work with Cognee OR ANY other memory system
pub trait MemorySystem {
    fn store_memory(&mut self, key: String, value: String, importance: f64) -> bool;
    fn retrieve_memory(&self, key: &str) -> Option<String>;
    fn perform_maintenance(&mut self) -> MaintenanceResult;
    fn get_stats(&self) -> MemoryStats;
}

// DEFAULT IMPLEMENTATION (for when Cognee isn't ready yet)
pub struct DefaultMemorySystem {
    memories: HashMap<String, String>,
    maintenance_count: u64,
}

impl DefaultMemorySystem {
    pub fn new() -> Self {
        Self {
            memories: HashMap::new(),
            maintenance_count: 0,
        }
    }
}

impl MemorySystem for DefaultMemorySystem {
    fn store_memory(&mut self, key: String, value: String, _importance: f64) -> bool {
        self.memories.insert(key, value);
        true
    }

    fn retrieve_memory(&self, key: &str) -> Option<String> {
        self.memories.get(key).cloned()
    }

    fn perform_maintenance(&mut self) -> MaintenanceResult {
        self.maintenance_count += 1;
        
        // Simple maintenance: remove very old-looking keys (simulating purge)
        let before_count = self.memories.len();
        self.memories.retain(|key, _| !key.contains("temp_"));
        let after_count = self.memories.len();
        let purged_count = before_count - after_count;
        
        MaintenanceResult {
            memories_purged: purged_count,
            memories_consolidated: 0,
            errors_fixed: 0,
            maintenance_type: "default_cleanup".to_string(),
        }
    }

    fn get_stats(&self) -> MemoryStats {
        MemoryStats {
            total_memories: self.memories.len(),
            maintenance_count: self.maintenance_count,
            system_type: "default".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceResult {
    pub memories_purged: usize,
    pub memories_consolidated: usize,
    pub errors_fixed: usize,
    pub maintenance_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_memories: usize,
    pub maintenance_count: u64,
    pub system_type: String,
}

// MAIN SLEEP PROTOCOL MANAGER
pub struct SleepProtocol {
    pub sleep_system: BlueWhaleSleep,
    pub memory_system: Box<dyn MemorySystem>,
    pub judicial_core: JudicialCore,
    pub sleep_log: Vec<SleepCycleRecord>,
    pub is_sleeping: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepCycleRecord {
    pub cycle_id: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub sleep_state: SleepState,
    pub maintenance_result: MaintenanceResult,
    pub system_health_before: SystemHealth,
    pub system_health_after: SystemHealth,
    pub judicial_approval: bool,
}

impl SleepProtocol {
    pub fn new() -> Self {
        Self {
            sleep_system: BlueWhaleSleep::new(),
            memory_system: Box::new(DefaultMemorySystem::new()),
            judicial_core: JudicialCore::new(),
            sleep_log: Vec::new(),
            is_sleeping: false,
        }
    }

    // MAIN ENTRY POINT: Request sleep with judicial approval
    pub fn request_sleep(&mut self, requested_state: SleepState) -> SleepRequestResult {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // STEP 1: Check with Judicial Core if sleep is allowed
        let sleep_action = SystemAction {
            action_type: "SLEEP_REQUEST".into(),
            payload: format!("requested_state:{:?}", requested_state),
            context: "sleep_protocol".into(),
        };

        let judicial_verdict = self.judicial_core.rule(sleep_action);

        match judicial_verdict {
            crate::verdicts::Verdict::Approved => {
                // Sleep approved - proceed with sleep cycle
                self.execute_sleep_cycle(requested_state, now)
            }
            crate::verdicts::Verdict::Rejected(reason) => {
                // Sleep denied by Judicial Core
                SleepRequestResult::Denied(reason)
            }
            crate::verdicts::Verdict::RejectedWithSuggestion(reason, suggestion) => {
                // Sleep denied with suggestion (maybe try different sleep state)
                SleepRequestResult::DeniedWithSuggestion(reason, suggestion)
            }
        }
    }

    // EXECUTE THE SLEEP CYCLE
    fn execute_sleep_cycle(&mut self, sleep_state: SleepState, start_time: u64) -> SleepRequestResult {
        self.is_sleeping = true;
        
        let health_before = self.sleep_system.get_system_health();
        
        println!("😴 ENTERING SLEEP STATE: {:?}", sleep_state);
        println!("   System health before: {:.1}%", health_before.health_score * 100.0);

        // STEP 2: Perform appropriate maintenance based on sleep state
        let maintenance_result = match sleep_state {
            SleepState::LightSleep => self.perform_light_sleep_maintenance(),
            SleepState::DeepSleep => self.perform_deep_sleep_maintenance(),
            SleepState::REM => self.perform_rem_sleep_maintenance(),
            _ => MaintenanceResult {
                memories_purged: 0,
                memories_consolidated: 0,
                errors_fixed: 0,
                maintenance_type: "awake_no_maintenance".to_string(),
            }
        };

        // STEP 3: Update Blue Whale Sleep system
        self.sleep_system.current_cycle.state = sleep_state.clone();
        self.sleep_system.current_cycle.cycle_start = start_time;
        self.sleep_system.current_cycle.last_maintenance = start_time;

        let health_after = self.sleep_system.get_system_health();
        
        // STEP 4: Record this sleep cycle
        let cycle_record = SleepCycleRecord {
            cycle_id: self.sleep_log.len() as u64 + 1,
            start_time,
            end_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            sleep_state: sleep_state.clone(),
            maintenance_result: maintenance_result.clone(),
            system_health_before: health_before,
            system_health_after: health_after,
            judicial_approval: true,
        };

        self.sleep_log.push(cycle_record);
        self.is_sleeping = false;

        SleepRequestResult::Completed(maintenance_result)
    }

    // LIGHT SLEEP MAINTENANCE - Quick cleanup
    fn perform_light_sleep_maintenance(&mut self) -> MaintenanceResult {
        println!("   Performing light sleep maintenance...");
        
        // Quick cache purge
        let bw_maintenance = self.sleep_system.perform_cache_purge();
        
        // Basic memory system maintenance
        let memory_maintenance = self.memory_system.perform_maintenance();
        
        MaintenanceResult {
            memories_purged: bw_maintenance.memory_purged + memory_maintenance.memories_purged,
            memories_consolidated: memory_maintenance.memories_consolidated,
            errors_fixed: memory_maintenance.errors_fixed,
            maintenance_type: "light_sleep".to_string(),
        }
    }

    // DEEP SLEEP MAINTENANCE - Intensive cleanup
    fn perform_deep_sleep_maintenance(&mut self) -> MaintenanceResult {
        println!("   Performing deep sleep maintenance...");
        
        // Intensive cache purge
        let bw_maintenance = self.sleep_system.perform_cache_purge();
        
        // Multiple rounds of memory maintenance
        let mut total_purged = 0;
        let mut total_consolidated = 0;
        let mut total_errors = 0;
        
        for _ in 0..3 { // 3 rounds of intensive maintenance
            let memory_maintenance = self.memory_system.perform_maintenance();
            total_purged += memory_maintenance.memories_purged;
            total_consolidated += memory_maintenance.memories_consolidated;
            total_errors += memory_maintenance.errors_fixed;
        }
        
        MaintenanceResult {
            memories_purged: bw_maintenance.memory_purged + total_purged,
            memories_consolidated: total_consolidated,
            errors_fixed: total_errors,
            maintenance_type: "deep_sleep".to_string(),
        }
    }

    // REM SLEEP MAINTENANCE - Memory consolidation
    fn perform_rem_sleep_maintenance(&mut self) -> MaintenanceResult {
        println!("   Performing REM sleep maintenance...");
        
        // Focus on memory consolidation rather than purging
        let bw_maintenance = self.sleep_system.perform_cache_purge();
        
        // Special REM: Multiple consolidation rounds
        let mut consolidation_rounds = 0;
        for _ in 0..5 {
            let memory_maintenance = self.memory_system.perform_maintenance();
            consolidation_rounds += memory_maintenance.memories_consolidated;
        }
        
        MaintenanceResult {
            memories_purged: bw_maintenance.memory_purged,
            memories_consolidated: consolidation_rounds,
            errors_fixed: bw_maintenance.errors_fixed,
            maintenance_type: "rem_sleep".to_string(),
        }
    }

    // EMERGENCY WAKE FUNCTION
    pub fn emergency_wake(&mut self) -> bool {
        if self.is_sleeping {
            println!("🚨 EMERGENCY WAKE INITIATED!");
            self.is_sleeping = false;
            self.sleep_system.current_cycle.state = SleepState::Awake;
            true
        } else {
            false
        }
    }

    // MEMORY STORAGE WITH JUDICIAL OVERSIGHT
    pub fn store_memory_with_oversight(&mut self, key: String, value: String, importance: f64) -> bool {
        // Check with Judicial Core first
        let memory_action = SystemAction {
            action_type: "MEMORY_STORAGE".into(),
            payload: format!("key:{}, importance:{}", key, importance),
            context: "sleep_protocol".into(),
        };

        match self.judicial_core.rule(memory_action) {
            crate::verdicts::Verdict::Approved => {
                // Store in both systems
                self.sleep_system.store_memory(key.clone(), value.clone(), importance);
                self.memory_system.store_memory(key, value, importance)
            }
            _ => {
                println!("❌ Judicial Core blocked memory storage");
                false
            }
        }
    }

    // GET SLEEP PROTOCOL STATUS
    pub fn get_status(&self) -> SleepProtocolStatus {
        let system_health = self.sleep_system.get_system_health();
        let memory_stats = self.memory_system.get_stats();
        let (should_sleep, recommended_state) = self.sleep_system.should_sleep();

        SleepProtocolStatus {
            is_sleeping: self.is_sleeping,
            current_state: self.sleep_system.current_cycle.state.clone(),
            system_health,
            memory_stats,
            should_sleep,
            recommended_state,
            total_sleep_cycles: self.sleep_log.len(),
            judicial_compliance: self.judicial_core.get_compliance_score(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SleepRequestResult {
    Completed(MaintenanceResult),
    Denied(String),
    DeniedWithSuggestion(String, String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepProtocolStatus {
    pub is_sleeping: bool,
    pub current_state: SleepState,
    pub system_health: SystemHealth,
    pub memory_stats: MemoryStats,
    pub should_sleep: bool,
    pub recommended_state: SleepState,
    pub total_sleep_cycles: usize,
    pub judicial_compliance: f64,
}

impl Default for SleepProtocol {
    fn default() -> Self {
        Self::new()
    }
}
