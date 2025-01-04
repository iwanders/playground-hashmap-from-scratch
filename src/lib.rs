mod bucket_separate_chain;

pub use bucket_separate_chain::BucketSeperateChainHashMap;

pub type MainError = Box<dyn std::error::Error + Sync + Send>;
pub fn main() -> Result<(), MainError> {
    Ok(())
}
