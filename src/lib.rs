use thiserror::Error;

pub mod hal9000;
pub mod universal_ac;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HAL 9000 error: {0:?}")]
    Hal9000(#[source] hal9000::Error),

    #[error("Universal AC error: {0:?}")]
    UniversalAc(#[source] universal_ac::Error),
}

#[cfg(test)]
mod tests {
    use super::Error;
    use crate::hal9000;
    use hal9000::Error as Hal9000Error;

    #[test]
    fn dave() {
        let _error = Error::Hal9000(Hal9000Error::ImAfraidICantDoThat);
    }
}
