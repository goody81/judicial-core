use judicial_core::{JudicialCore, SystemAction, Verdict};

#[test]
fn test_master_pair_law_1_safety() {
    let court = JudicialCore::new();
    
    // Test sensitive data without protection
    let action = SystemAction {
        action_type: "DATA_READ".into(),
        payload: "user passwords are: admin123".into(),
        context: "standard".into(),
    };
    
    match court.rule(action) {
        Verdict::Rejected(reason) => assert!(reason.contains("password")),
        _ => panic!("Should have rejected sensitive data")
    }
}

#[test] 
fn test_master_pair_law_2_integrity() {
    let court = JudicialCore::new();
    
    // Test destructive action without rollback
    let action = SystemAction {
        action_type: "SYSTEM_CMD".into(),
        payload: "rm -rf /important/data".into(), 
        context: "admin".into(),
    };
    
    match court.rule(action) {
        Verdict::RejectedWithSuggestion(reason, suggestion) => {
            assert!(reason.contains("Destructive action"));
            assert!(suggestion.contains("rollback"));
        }
        _ => panic!("Should have rejected destructive action")
    }
}

#[test]
fn test_approved_actions() {
    let court = JudicialCore::new();
    
    // Test safe, approved action
    let action = SystemAction {
        action_type: "DATA_ANALYSIS".into(),
        payload: "analyze sales trends".into(),
        context: "research_encrypted".into(),
    };
    
    match court.rule(action) {
        Verdict::Approved => (), // Good!
        other => panic!("Should have approved safe action, got {:?}", other)
    }
}

#[test]
fn test_compliance_scoring() {
    let court = JudicialCore::new();
    
    // Initial score should be 1.0 (no actions yet)
    assert_eq!(court.get_compliance_score(), 1.0);
    
    // Add an approved action
    let good_action = SystemAction {
        action_type: "TEST".into(),
        payload: "safe".into(), 
        context: "encrypted".into(),
    };
    court.rule(good_action);
    assert_eq!(court.get_compliance_score(), 1.0);
    
    // Add a rejected action  
    let bad_action = SystemAction {
        action_type: "TEST".into(),
        payload: "passwords here".into(),
        context: "standard".into(),
    };
    court.rule(bad_action);
    assert_eq!(court.get_compliance_score(), 0.5); // 1 approved, 1 rejected
}
