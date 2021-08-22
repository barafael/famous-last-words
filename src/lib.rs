use thiserror::Error;

pub mod hal9000;

#[derive(Debug, Error)]
#[repr(usize)]
pub enum Error {
    #[error("HAL 9000 error: {0:?}")]
    Hal9000(#[from] hal9000::Error),
}

#[no_mangle]
pub extern "C" fn print_famous_last_word(word: Error) {
    println!("{:?}", word);
}

#[cfg(test)]
mod tests {
    use super::Error;
    use super::hal9000;

    #[test]
    fn dave() {
        let _error = Error::Hal9000(hal9000::Error::ImAfraidICantDoThat);
    }
}
