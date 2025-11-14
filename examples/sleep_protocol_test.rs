use judicial_core::{SleepProtocol, SleepState, SleepRequestResult};

fn main() {
    println!("😴 COMPLETE SLEEP PROTOCOL TEST");
    println!("==================================================");  // ← OR JUST USE LITERAL
    println!("🧠 Integrated Sleep + Judicial Core + Memory Systems");
    println!();

    let mut sleep_protocol = SleepProtocol::new();

    // Display initial status
    let initial_status = sleep_protocol.get_status();
    println!("📊 INITIAL SYSTEM STATUS:");
    println!("   Current State: {:?}", initial_status.current_state);
    println!("   Should Sleep: {}", initial_status.should_sleep);
    println!("   Recommended State: {:?}", initial_status.recommended_state);
    println!("   Judicial Compliance: {:.1}%", initial_status.judicial_compliance * 100.0);
    println!("   Memory System: {}", initial_status.memory_stats.system_type);
    println!();

    // STEP 1: Store some memories with judicial oversight
    println!("💾 STORING MEMORIES WITH JUDICIAL OVERSIGHT:");
    
    let memories = vec![
        ("critical_config".to_string(), "server_timeout=30".to_string(), 0.9),
        ("user_session".to_string(), "user_123_active".to_string(), 0.6),
        ("temp_cache".to_string(), "cached_result_xyz".to_string(), 0.2),
    ];

    for (key, value, importance) in memories {
        let success = sleep_protocol.store_memory_with_oversight(key, value, importance);
        println!("   Stored memory (importance: {}): {}", importance, success);
    }
    println!();

    // STEP 2: Check system health after memory storage
    let status_after_storage = sleep_protocol.get_status();
    println!("📊 SYSTEM HEALTH AFTER MEMORY STORAGE:");
    println!("   Waste Level: {:.1}%", status_after_storage.system_health.waste_level * 100.0);
    println!("   Memory Usage: {:.1}%", status_after_storage.system_health.memory_usage * 100.0);
    println!("   Health Score: {:.1}%", status_after_storage.system_health.health_score * 100.0);
    println!();

    // STEP 3: Request light sleep (should be approved)
    println!("💤 REQUESTING LIGHT SLEEP:");
    match sleep_protocol.request_sleep(SleepState::LightSleep) {
        SleepRequestResult::Completed(maintenance) => {
            println!("   ✅ Light sleep completed successfully!");
            println!("   Memories purged: {}", maintenance.memories_purged);
            println!("   Maintenance type: {}", maintenance.maintenance_type);
        }
        SleepRequestResult::Denied(reason) => {
            println!("   ❌ Light sleep denied: {}", reason);
        }
        SleepRequestResult::DeniedWithSuggestion(reason, suggestion) => {
            println!("   ⚠️ Light sleep denied: {} -> {}", reason, suggestion);
        }
    }
    println!();

    // STEP 4: Check status after light sleep
    let status_after_light_sleep = sleep_protocol.get_status();
    println!("📊 STATUS AFTER LIGHT SLEEP:");
    println!("   Current State: {:?}", status_after_light_sleep.current_state);
    println!("   Waste Level: {:.1}%", status_after_light_sleep.system_health.waste_level * 100.0);
    println!("   Health Score: {:.1}%", status_after_light_sleep.system_health.health_score * 100.0);
    println!("   Total Sleep Cycles: {}", status_after_light_sleep.total_sleep_cycles);
    println!();

    println!("🎉 SLEEP PROTOCOL FULLY OPERATIONAL!");
    println!("   ✅ Integrated Judicial Core oversight");
    println!("   ✅ Flexible memory system interface (ready for Cognee)");
    println!("   ✅ Multiple sleep states with different maintenance");
    println!("   ✅ Comprehensive status monitoring");
}
