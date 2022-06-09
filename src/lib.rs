use thiserror::Error;

pub mod hal9000;
pub mod multivac;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HAL 9000 error: {0}")]
    Hal9000(#[source] hal9000::Error),

    #[error("Multivac error: {0}")]
    Multivac(#[source] multivac::Error),
}

/// Get a random error.
pub fn get_random_error() -> Error {
    if rand::random() {
        Error::Hal9000(rand::random())
    } else {
        Error::Multivac(rand::random())
    }
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
