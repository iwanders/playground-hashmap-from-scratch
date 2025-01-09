mod bucket_separate_chain;

pub use bucket_separate_chain::BucketSeperateChainHashMap;

pub mod bucket_seperate_chain_simple;

pub type MainError = Box<dyn std::error::Error + Sync + Send>;
pub fn main() -> Result<(), MainError> {
    Ok(())
}
