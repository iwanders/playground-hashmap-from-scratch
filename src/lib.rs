mod bucket;

pub use bucket::BucketHashmap;

pub type MainError = Box<dyn std::error::Error + Sync + Send>;
pub fn main() -> Result<(), MainError> {
    Ok(())
}
