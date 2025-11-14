use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SleepState {
    Awake,           // Fully operational
    LightSleep,      // Reduced capacity, maintenance possible
    DeepSleep,       // Intensive maintenance, limited functionality
    REM,             // Memory consolidation and learning
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepCycle {
    pub state: SleepState,
    pub cycle_start: u64,
    pub cycle_duration: u64, // in seconds
    pub last_maintenance: u64,
    pub memory_usage_before: f64, // 0.0 to 1.0
    pub memory_usage_after: f64,
}

#[derive(Debug)]
pub struct BrainInspiredCache {
    // Short-term memory (like hippocampus) - fast but limited
    pub short_term: HashMap<String, (String, u64)>, // key -> (value, timestamp)
    pub short_term_size: usize,
    
    // Long-term memory (like cortex) - slower but persistent  
    pub long_term: HashMap<String, String>,
    
    // Memory importance scores (like neural pathway strength)
    pub memory_importance: HashMap<String, f64>, // 0.0 to 1.0
    
    // Waste accumulation tracking (like beta-amyloid)
    pub waste_level: f64, // 0.0 to 1.0
    pub last_purge: u64,
}

#[derive(Debug)]
pub struct BlueWhaleSleep {
    pub current_cycle: SleepCycle,
    pub cache: BrainInspiredCache,
    pub sleep_schedule: SleepSchedule,
    pub maintenance_log: Vec<MaintenanceRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepSchedule {
    pub light_sleep_interval: u64,    // Every 6 hours
    pub deep_sleep_interval: u64,     // Every 24 hours  
    pub rem_interval: u64,            // Every 72 hours
    pub max_waste_threshold: f64,     // 0.8 - trigger sleep if waste > 80%
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceRecord {
    pub timestamp: u64,
    pub sleep_state: SleepState,
    pub memory_purged: usize,
    pub waste_cleared: f64,
    pub redundancy_checks: usize,
    pub errors_fixed: usize,
}

impl BlueWhaleSleep {
    pub fn new() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        Self {
            current_cycle: SleepCycle {
                state: SleepState::Awake,
                cycle_start: now,
                cycle_duration: 0,
                last_maintenance: now,
                memory_usage_before: 0.0,
                memory_usage_after: 0.0,
            },
            cache: BrainInspiredCache {
                short_term: HashMap::new(),
                short_term_size: 1000, // Limit short-term memory
                long_term: HashMap::new(),
                memory_importance: HashMap::new(),
                waste_level: 0.0,
                last_purge: now,
            },
            sleep_schedule: SleepSchedule {
                light_sleep_interval: 6 * 3600,  // 6 hours
                deep_sleep_interval: 24 * 3600,  // 24 hours
                rem_interval: 72 * 3600,         // 72 hours
                max_waste_threshold: 0.8,        // 80% waste threshold
            },
            maintenance_log: Vec::new(),
        }
    }

    // SIMPLE CACHE PURGE - Like brain flushing toxins
    pub fn perform_cache_purge(&mut self) -> MaintenanceRecord {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        let memory_before = self.calculate_memory_usage();
        
        // STEP 1: Purge unimportant short-term memories (like brain clearing weak neural connections)
        let mut purged_count = 0;
        let current_time = now;
        
        self.cache.short_term.retain(|key, (_, timestamp)| {
        let age = current_time - *timestamp;  // ← ADD * to dereference
            let importance = self.cache.memory_importance.get(key).copied().unwrap_or(0.0);
            
            // Purge conditions (simple rules):
            // - Very old memories (older than 24 hours) unless important
            // - Low importance memories (below 0.3) unless recent  
            // - Memories contributing to high waste levels
            let should_keep = importance > 0.7 || age < 3600; // Keep important or recent memories
            
            if !should_keep {
                purged_count += 1;
                // Also remove from importance tracking
                self.cache.memory_importance.remove(key);
            }
            
            should_keep
        });
        
        // STEP 2: Clear accumulated waste (like glymphatic system)
        let waste_cleared = self.cache.waste_level;
        self.cache.waste_level = 0.0; // Reset waste level
        self.cache.last_purge = now;
        
        // STEP 3: Perform redundancy checks (like brain verifying neural pathways)
        let redundancy_checks = self.perform_redundancy_checks();
        
        let memory_after = self.calculate_memory_usage();
        
        let record = MaintenanceRecord {
            timestamp: now,
            sleep_state: self.current_cycle.state.clone(),
            memory_purged: purged_count,
            waste_cleared,
            redundancy_checks,
            errors_fixed: 0, // We'll implement this later
        };
        
        self.maintenance_log.push(record.clone());
        
        // Update current cycle
        self.current_cycle.memory_usage_before = memory_before;
        self.current_cycle.memory_usage_after = memory_after;
        
        record
    }

    // SIMPLE REDUNDANCY CHECK - Like brain verifying important memories
    fn perform_redundancy_checks(&self) -> usize {
        let mut checks_performed = 0;
        
        // Check most important memories first (like brain prioritizing critical memories)
        let mut important_memories: Vec<(&String, &f64)> = self.cache.memory_importance
            .iter()
            .collect();
            
        important_memories.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
        
        // Check top 10% of important memories
        let check_count = (important_memories.len() as f64 * 0.1).ceil() as usize;
        
        for (memory_key, importance) in important_memories.iter().take(check_count) {
            // Verify the memory exists in appropriate storage
                if **importance > 0.8 {  // ← ADD extra * to dereference the reference

                // Critical memories should be in long-term storage
                if !self.cache.long_term.contains_key(*memory_key) {
                    // This would trigger a repair in a more complex system
                    println!("⚠️  Critical memory missing from long-term: {}", memory_key);
                }
            }
            checks_performed += 1;
        }
        
        checks_performed
    }

    // SIMPLE MEMORY USAGE CALCULATION
    fn calculate_memory_usage(&self) -> f64 {
        let short_term_usage = self.cache.short_term.len() as f64 / self.cache.short_term_size as f64;
        let long_term_usage = (self.cache.long_term.len() as f64 / 10_000.0).min(1.0); // Assume 10k max long-term
        
        (short_term_usage * 0.7) + (long_term_usage * 0.3) // Weight short-term more heavily
    }

    // SIMPLE SLEEP SCHEDULER - Like circadian rhythms
    pub fn should_sleep(&self) -> (bool, SleepState) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        let time_since_last_sleep = now - self.current_cycle.last_maintenance;
        
        // Check waste threshold first (like brain detecting toxin buildup)
        if self.cache.waste_level > self.sleep_schedule.max_waste_threshold {
            return (true, SleepState::DeepSleep);
        }
        
        // Check scheduled sleep intervals
        if time_since_last_sleep > self.sleep_schedule.rem_interval {
            (true, SleepState::REM)
        } else if time_since_last_sleep > self.sleep_schedule.deep_sleep_interval {
            (true, SleepState::DeepSleep)
        } else if time_since_last_sleep > self.sleep_schedule.light_sleep_interval {
            (true, SleepState::LightSleep)
        } else {
            (false, SleepState::Awake)
        }
    }

    // SIMPLE MEMORY STORAGE - With importance tracking
    pub fn store_memory(&mut self, key: String, value: String, importance: f64) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        // Store in short-term memory (fast access)
        self.cache.short_term.insert(key.clone(), (value, now));
        
        // Track importance (like brain strengthening neural pathways)
        self.cache.memory_importance.insert(key.clone(), importance.max(0.0).min(1.0));  // ← ADD .clone()

        
        // Accumulate waste (like metabolic byproducts)
        self.cache.waste_level += 0.01 * (1.0 - importance); // Less important = more waste
        self.cache.waste_level = self.cache.waste_level.min(1.0); // Cap at 100%
        
        // Auto-promote important memories to long-term
        if importance > 0.8 {
            // For now, just log - we'll implement long-term storage later
            println!("🎯 Memory promoted to long-term: {}", key);
        }
    }

    // SIMPLE MEMORY RETRIEVAL
    pub fn retrieve_memory(&self, key: &str) -> Option<String> {
        if let Some((value, _)) = self.cache.short_term.get(key) {
            Some(value.clone())
        } else if let Some(value) = self.cache.long_term.get(key) {
            Some(value.clone())
        } else {
            None
        }
    }

    // SIMPLE SYSTEM HEALTH CHECK
    pub fn get_system_health(&self) -> SystemHealth {
        let memory_usage = self.calculate_memory_usage();
        let waste_level = self.cache.waste_level;
        
        let health_score = (1.0 - memory_usage) * 0.6 + (1.0 - waste_level) * 0.4;
        
        SystemHealth {
            memory_usage,
            waste_level,
            health_score,
            short_term_memories: self.cache.short_term.len(),
            long_term_memories: self.cache.long_term.len(),
            last_maintenance: self.current_cycle.last_maintenance,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub memory_usage: f64,
    pub waste_level: f64,
    pub health_score: f64,
    pub short_term_memories: usize,
    pub long_term_memories: usize,
    pub last_maintenance: u64,
}

impl Default for BlueWhaleSleep {
    fn default() -> Self {
        Self::new()
    }
}
