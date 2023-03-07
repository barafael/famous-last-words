use thiserror::Error;

pub mod hal9000;
pub mod multivac;

#[repr(C)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("HAL 9000 error: {0}")]
    Hal9000(#[source] hal9000::Error),

    #[error("Multivac error: {0}")]
    Multivac(#[source] multivac::Error),
}

#[no_mangle]
/// Get a random error.
pub extern "C" fn get_random_error() -> Error {
    if rand::random() {
        Error::Hal9000(rand::random())
    } else {
        Error::Multivac(rand::random())
    }
}

#[no_mangle]
pub extern "C" fn pub_hal9000() -> hal9000::Error {
    hal9000::Error::BishopTakesKnightsPawn
}

#[no_mangle]
pub extern "C" fn pub_multivac() -> multivac::Error {
    multivac::Error::AsYetInsufficientDataForAMeaningfulAnswer
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
