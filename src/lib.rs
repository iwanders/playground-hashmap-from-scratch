mod bucket_separate_chain;
mod std_unordered_map;

pub use bucket_separate_chain::BucketSeperateChainHashMap;
pub use std_unordered_map::CppStdUnorderedMapU64U64;

pub type MainError = Box<dyn std::error::Error + Sync + Send>;
pub fn main() -> Result<(), MainError> {
    Ok(())
}
