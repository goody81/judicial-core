pub mod judicial_core;
pub mod laws;
pub mod verdicts;
pub mod ledger;
pub mod integration;

pub use judicial_core::JudicialCore;
pub use verdicts::{Verdict, SystemAction};
pub use laws::master_pair::MasterPair;  // ← CORRECT

// 👇 ADD THESE NEW LINES - BUT REMOVE THE DUPLICATE 'laws' 👇
pub mod blue_whale_sleep;
pub mod sleep_protocol;

pub use blue_whale_sleep::{BlueWhaleSleep, SleepState, SystemHealth};
pub use sleep_protocol::{SleepProtocol, MemorySystem, DefaultMemorySystem, SleepRequestResult};
