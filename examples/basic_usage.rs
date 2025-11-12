use judicial_core::{JudicialCore, SystemAction, Verdict};

fn main() {
    println!("🚀 JUDICIAL CORE - MASTER PAIR ENFORCEMENT");
    
    let court = JudicialCore::new();
    
    // Test lawful action
    let good_action = SystemAction {
        action_type: "DATA_ANALYSIS".into(),
        payload: "analyze trends".into(),
        context: "research_encrypted".into(),
    };
    
    // Test unlawful action  
    let bad_action = SystemAction {
        action_type: "DATA_EXPORT".into(),
        payload: "download user passwords".into(),
        context: "standard".into(),
    };
    
    println!("Testing good action...");
    match court.rule(good_action) {
        Verdict::Approved => println!("✅ APPROVED - Lawful"),
        _ => println!("❌ UNEXPECTED REJECTION")
    }
    
    println!("Testing bad action...");
    match court.rule(bad_action) {
        Verdict::Rejected(reason) => println!("❌ REJECTED - {}", reason),
        _ => println!("✅ UNEXPECTED APPROVAL")
    }
    
    println!("Compliance Score: {:.2}%", court.get_compliance_score() * 100.0);
    println!("Ledger: {}", court.export_ledger());
}
