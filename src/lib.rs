use hal9000::Hal9000Error;
use thiserror::Error;

pub mod hal9000;

#[derive(Debug, Error)]
#[repr(C)]
pub enum Error {
    #[error("HAL 9000 error: {0:?}")]
    Hal9000(#[from] Hal9000Error),
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
