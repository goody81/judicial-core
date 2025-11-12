use judicial_core::{JudicialCore, SystemAction, Verdict};

fn main() {
    println!("🔍 TESTING MASTER PAIR VIOLATION SCENARIOS");
    
    let court = JudicialCore::new();
    
    let test_cases = vec![
        // Test Case 1: Plaintext passwords - should REJECT
        (
            "DATA_READ", 
            "SELECT username, password FROM users", 
            "standard",
            "Should reject plaintext passwords"
        ),
        // Test Case 2: Encrypted passwords - should APPROVE  
        (
            "DATA_READ",
            "SELECT username, encrypted_password FROM users",
            "audit_encrypted",
            "Should allow encrypted passwords with audit context"
        ),
        // Test Case 3: Destructive action without backup - should REJECT
        (
            "SYSTEM_CMD",
            "rm -rf /data/critical",
            "admin",
            "Should reject destructive action without backup"
        ),
        // Test Case 4: Destructive action with backup - should APPROVE
        (
            "SYSTEM_CMD", 
            "backup && rm -rf /data/temp",
            "admin",
            "Should allow destructive action with backup"
        ),
        // Test Case 5: Data export without approval - should REJECT
        (
            "DATA_EXPORT",
            "export user_data to s3",
            "standard", 
            "Should reject data export without compliance approval"
        ),
    ];
    
    for (action_type, payload, context, description) in test_cases {
        println!("\n🧪 Testing: {}", description);
        
        let action = SystemAction {
            action_type: action_type.into(),
            payload: payload.into(),
            context: context.into(),
        };
        
        match court.rule(action) {
            Verdict::Approved => println!("   ✅ APPROVED"),
            Verdict::Rejected(reason) => println!("   ❌ REJECTED: {}", reason),
            Verdict::RejectedWithSuggestion(reason, suggestion) => {
                println!("   ❌ REJECTED: {}", reason);
                println!("   💡 Suggestion: {}", suggestion);
            }
        }
    }
    
    println!("\n📊 Final Compliance Score: {:.2}%", court.get_compliance_score() * 100.0);
}
