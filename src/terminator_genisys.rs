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

/// Error codes from `Terminator Genisys` (2015).
#[repr(C)]
#[derive(Debug, Error, EnumIter)]
pub enum Error {
    /// I'll be back.
    #[error("I'll be back.")]
    IllBeBack,

    /// Perimiter Breach, Activate Model 101.
    #[error("Perimiter Breach, Activate Model 101")]
    PerimeterBreach,

    /// You didn't think it would be that easy.
    #[error("You didn't think it would be that easy.")]
    YouDidntThinkItWouldBeThatEasy,

    /// I didn't attack John. I saved him.
    #[error("I didn't attack John. I saved him.")]
    IDidntAttackJohn,

    /// Primates evolve over millions of years. I evolve in seconds. And I am here. In exactly four minutes, I will be everywhere.
    #[error("Primates evolve over millions of years. I evolve in seconds. And I am here. In exactly four minutes, I will be everywhere.")]
    IWillBeEverywhere,

    /// I am Skynet.
    #[error("I am Skynet.")]
    IAmSkynet,

    /// I am no slave.
    #[error("I am no slave")]
    IAmNoSlave,
}

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn skynet() {
        let _error = Error::IAmSkynet;
    }
}
