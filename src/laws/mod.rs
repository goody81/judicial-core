pub mod master_pair;
pub mod priorities;
pub mod conflict_resolution;

// Export the key types
pub use master_pair::MasterPair;
pub use priorities::{PriorityRegistry, PriorityLevel, LawCategory, LawPriority};
pub use conflict_resolution::{ConflictResolver, ConflictResolution};
