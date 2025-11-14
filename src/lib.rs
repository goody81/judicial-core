pub mod judicial_core;
pub mod laws;
pub mod verdicts;
pub mod ledger;
pub mod integration;  // This line should exist

pub use judicial_core::JudicialCore;
pub use verdicts::{Verdict, SystemAction};
pub use laws::{MasterPair};

// Remove or comment out the Python feature line for now
// #[cfg(feature = "python")]
// pub use integration::python_ffi;
