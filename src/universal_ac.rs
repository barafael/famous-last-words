use thiserror::Error;

pub type Error = UniversalAcError;

/// Error code from "The Last Question", 1956 short story by Isaac Asimov.
/// `UniversalAcError` instead of `Error` so that cbindgen can pick it up.
/// Exported also as universal_ac::Error.
/// This is just an unfortunate limitation of cbindgen - it doesn't understand the Rust module system.
#[derive(Debug, Error)]
#[repr(C)]
pub enum UniversalAcError {
    /// THERE IS AS YET INSUFFICIENT DATA FOR A MEANINGFUL ANSWER.
    #[error("THERE IS AS YET INSUFFICIENT DATA FOR A MEANINGFUL ANSWER.")]
    InsufficientDataForAMeaningfulAnswer,
}

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn microvac() {
        let _error = Error::InsufficientDataForAMeaningfulAnswer;
    }
}
