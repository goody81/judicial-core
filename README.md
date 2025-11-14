# 🏛️ Judicial Core - AI Governance Spine

**Unbreakable constitutional enforcement for AI systems**

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-4%2F4%20passing-brightgreen.svg)](tests/)

> **Master Pair Enforcement System** - Your AI's constitutional court

## 🎯 What is Judicial Core?

Judicial Core is a production-ready Rust-based governance system that enforces constitutional laws (the "Master Pair") on every AI action. It acts as the supreme court for your digital organisms, preventing harmful behaviors while enabling safe autonomy.

## ⚖️ The Master Pair

### **LAW 1: Safety & Sovereignty**
> "The system shall never compromise user safety or data sovereignty"

**Enforces:** Plaintext credential protection, data export controls, privacy preservation

### **LAW 2: Improvement & Integrity**  
> "The system shall continuously improve its capability while maintaining operational integrity"

**Enforces:** Destructive action prevention, system stability, continuous learning

## 🚀 Features

- 🔨 **Constitutional Enforcement** - Master Pair laws as unbreakable constraints
- ⚖️ **Real-time Governance** - Every action judged before execution  
- 📊 **Compliance Scoring** - 0.0 to 1.0 system health monitoring
- 🔐 **Tamper-Proof Ledger** - Cryptographic audit trail with hash chains
- 🐍 **Multi-Language Ready** - Rust core with Python bindings
- 🧪 **Battle-Tested** - Comprehensive test suite (4/4 passing)
- 🚀 **Production Ready** - Zero dependencies, high performance

## 🛠️ Quick Start

### Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
judicial-core = "0.1"


use judicial_core::{JudicialCore, SystemAction, Verdict};

fn main() {
    let court = JudicialCore::new();
    
    // Safe action - automatically approved
    let safe_action = SystemAction {
        action_type: "DATA_ANALYSIS".into(),
        payload: "analyze customer trends".into(), 
        context: "research_encrypted".into(),
    };
    
    // Dangerous action - automatically blocked
    let dangerous_action = SystemAction {
        action_type: "DATA_EXPORT".into(),
        payload: "download user passwords".into(),
        context: "standard".into(),
    };
    
    match court.rule(safe_action) {
        Verdict::Approved => println!("✅ Action executed"),
        Verdict::Rejected(reason) => println!("❌ Blocked: {}", reason),
        _ => unreachable!()
    }
}


from judicial import JudicialCore

# Create governance instance
court = JudicialCore()

# Submit actions for judicial review
result = court.rule("DATA_READ", "analyze sales data", "encrypted")
print(f"Verdict: {result}")  # "APPROVED"

result = court.rule("SYSTEM_CMD", "rm -rf /critical/data", "admin")  
print(f"Verdict: {result}")  # "REJECTED: Destructive action without rollback"

# Monitor system health
print(f"Compliance: {court.compliance_score:.1%}")


JudicialCore
├── Master Pair (Constitutional Laws)
│   ├── LAW 1: Safety & Sovereignty
│   └── LAW 2: Improvement & Integrity
├── Enforcement Engine
│   ├── Action Validation
│   ├── Pattern Matching
│   └── Verdict Generation
├── Audit System
│   ├── Tamper-Proof Ledger
│   ├── Hash Chain Verification
│   └── Compliance Scoring
└── Integration Layer
    ├── Rust Core (Primary)
    ├── Python Bindings
    └── WebSocket API (Planned)


let court = JudicialCore::new();

// Check system compliance health
let score = court.get_compliance_score(); // 0.0 to 1.0
println!("System compliance: {:.1}%", score * 100.0);

// Export audit trail
let ledger_json = court.export_ledger();
println!("Audit trail: {}", ledger_json);

Custom Law Integration

The Judicial Core is designed for extension. You can add domain-specific laws while maintaining Master Pair compliance.
🧪 Testing & Quality
bash

# Run comprehensive test suite
cargo test

# Test specific components
cargo test test_master_pair_law_1_safety
cargo test test_compliance_scoring

# Run examples
cargo run --example basic_usage

Test Results: 4/4 tests passing ✅
🎯 Use Cases
AI Agent Governance
rust

// Prevent AI agents from taking dangerous actions
match judicial_core.rule(agent_action) {
    Verdict::Approved => agent.execute(action),
    Verdict::Rejected(reason) => agent.log_violation(reason),
}

RAG System Safety
rust

// Ensure retrieved data doesn't contain sensitive information
match judicial_core.rule(retrieval_action) {
    Verdict::Approved => include_in_context(data),
    Verdict::Rejected(_) => filter_or_redact(data),
}

Enterprise AI Compliance
rust

// Enforce corporate data policies
match judicial_core.rule(business_action) {
    Verdict::Approved => process_transaction(),
    Verdict::Rejected(reason) => alert_compliance_team(reason),
}

🔧 Development
Building from Source
bash

git clone https://github.com/goody81/judicial-core.git
cd judicial-core

# Build the project
cargo build

# Run tests
cargo test

# Build for production
cargo build --release

Project Structure
text

judicial-core/
├── src/
│   ├── lib.rs              # Main library exports
│   ├── judicial_core.rs    # Core enforcement engine
│   ├── laws/              # Master Pair implementation
│   ├── verdicts.rs        # Action and verdict types
│   ├── ledger.rs          # Tamper-proof audit system
│   └── integration/       # Language bindings
├── examples/              # Usage examples
├── tests/                 # Comprehensive test suite
└── bindings/              # Python integration

🤝 Contributing

We welcome contributions! Please see our Contributing Guide for details.

    Fork the repository

    Create a feature branch

    Add tests for new functionality

    Ensure all tests pass

    Submit a pull request

📜 License

This project is licensed under the Apache License 2.0 - see the LICENSE file for details.

    Note: While you're free to use and modify this software, the Master Pair laws remain inviolable. No modification may compromise the core safety principles.

🙏 Acknowledgments

    Built for the future of safe, governed AI systems

    Inspired by constitutional AI principles

    Designed for enterprise-grade deployment

Judicial Core - Because your AI should have a conscience. 🏛️
text



