use thiserror::Error;

/// Error code from "The Last Question", 1956 short story by Isaac Asimov.
#[derive(Debug, Error)]
pub enum Error {
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
