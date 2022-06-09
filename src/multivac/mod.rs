use enum_index::IndexEnum;
use enum_index_derive::IndexEnum;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use thiserror::Error;
use variant_count::VariantCount;

/// Implement a standard distribution for our error code enum.
/// With this trait, `rand::random()` can be used to get a random enum variant.
/// All members of enum variants must be default-constructible for that to work.
impl Distribution<Error> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Error {
        let index = rng.gen_range(0..Error::VARIANT_COUNT);

        // unwrap here is OK by construction, given that the VARIANT_COUNT is derived correctly.
        Error::index_enum(index).unwrap()
    }
}

/// Error code from "The Last Question", 1956 short story by Isaac Asimov.
#[non_exhaustive]
#[derive(Debug, Error, VariantCount, IndexEnum)]
pub enum Error {
    /// INSUFFICIENT DATA FOR MEANINGFUL ANSWER.
    #[error("INSUFFICIENT DATA FOR MEANINGFUL ANSWER.")]
    InsufficientDataForMeaningfulAnswer,
    
    /// THERE IS AS YET INSUFFICIENT DATA FOR A MEANINGFUL ANSWER.
    #[error("THERE IS AS YET INSUFFICIENT DATA FOR A MEANINGFUL ANSWER.")]
    AsYetInsufficientDataForAMeaningfulAnswer,

    /// LET THERE BE LIGHT!
    #[error("LET THERE BE LIGHT!")]
    LetThereBeLight,
}

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn multivac() {
        let _error = Error::InsufficientDataForMeaningfulAnswer;
    }
}
