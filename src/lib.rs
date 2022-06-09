use thiserror::Error;
use variant_count::VariantCount;

pub mod hal9000;
mod random_error;
pub mod multivac;

#[non_exhaustive]
#[derive(Debug, Error, VariantCount)]
pub enum Error {
    #[error("HAL 9000 error: {0}")]
    Hal9000(#[source] hal9000::Error),

    #[error("Multivac error: {0}")]
    Multivac(#[source] multivac::Error),
}

/// Get a random error.
pub fn get_random_error() -> Error {
    rand::random()
}

#[cfg(test)]
mod tests {
    use super::Error;
    use crate::{get_random_error, hal9000};
    use hal9000::Error as Hal9000Error;

    #[test]
    fn dave() {
        let _error = Error::Hal9000(Hal9000Error::ImAfraidICantDoThat);
    }

    #[test]
    fn random_error() {
        let error = get_random_error();
        println!("{}", error);
    }
}
