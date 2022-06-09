use rand::distributions::{Distribution, Standard};
use rand::prelude::IteratorRandom;
use rand::Rng;
use strum::EnumIter;
use strum::IntoEnumIterator;
use thiserror::Error;

impl Distribution<Error> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Error {
        Error::iter().choose(rng).unwrap()
    }
}

/// Error codes from "The Last Question", 1956 short story by Isaac Asimov.
#[derive(Debug, Error, EnumIter)]
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
