use hal9000::Hal9000Error;
use thiserror::Error;
use universal_ac::UniversalAcError;

pub mod hal9000;
pub mod universal_ac;

#[derive(Debug, Error)]
#[repr(C)]
pub enum Error {
    #[error("HAL 9000 error: {0:?}")]
    Hal9000(#[from] Hal9000Error),

    #[error("Universal AC error: {0:?}")]
    UniversalAc(#[from] UniversalAcError),
}

#[no_mangle]
pub extern "C" fn print_famous_last_word(word: Error) {
    println!("{:?}", word);
}

#[cfg(test)]
mod tests {
    use crate::hal9000::Hal9000Error;

    use super::Error;

    #[test]
    fn dave() {
        let _error = Error::Hal9000(Hal9000Error::ImAfraidICantDoThat);
    }
}
