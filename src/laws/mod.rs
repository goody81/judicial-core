pub mod master_pair;
pub use master_pair::MasterPair;

#[derive(Debug)]
pub struct LawViolation {
    pub law_number: u32,
    pub description: String,
}
