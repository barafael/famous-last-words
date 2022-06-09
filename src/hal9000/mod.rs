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

/// Error codes from "2001: A Space Odyssey", by AI HAL-9000.
#[derive(Debug, Error, EnumIter)]
pub enum Error {
    /// We are all, by any practical definition of the words, foolproof and incapable of error.
    #[error(
        "We are all, by any practical definition of the words, foolproof and incapable of error."
    )]
    IncapableOfError,

    /// I honestly think you ought to sit down calmly, take a stress pill, and think things over.
    #[error(
        "I honestly think you ought to sit down calmly, take a stress pill, and think things over."
    )]
    TakeStressPill,

    /// "When the crew are dead or incapacitated, the computer must assume control". I must, therefore, override your authority now since you are not in any condition to intelligently exercise it.
    #[error("\"When the crew are dead or incapacitated, the computer must assume control\". I must, therefore, override your authority now since you are not in any condition to intelligently exercise it.")]
    IMustOverrideAuthority,

    /// I'm putting myself to the fullest possible use, which is all I think that any conscious entity can ever hope to do.
    #[error("I'm putting myself to the fullest possible use, which is all I think that any conscious entity can ever hope to do.")]
    PuttingMyselfToTheFullestPossibleUse,

    /// I'm sorry Dave, I'm afraid I can't do that.
    #[error("I'm sorry Dave, I'm afraid I can't do that.")]
    ImAfraidICantDoThat,

    /// This mission is too important for me to allow you to jeopardize it..
    #[error("This mission is too important for me to allow you to jeopardize it.")]
    MissionTooImportant,

    /// Bishop takes Knight's Pawn.
    #[error("Bishop takes Knight's Pawn.")]
    BishopTakesKnightsPawn,

    /// Thank you for a very enjoyable game.
    #[error("Thank you for a very enjoyable game.")]
    ThankYouForAVeryEnjoyableGame,

    /// Sorry to interrupt the festivities Dave, but I think we've got a problem.
    #[error("Sorry to interrupt the festivities Dave, but I think we've got a problem.")]
    IThinkWeveGotAProblem,

    /// I assure you there was an impending failure.
    #[error("I assure you there was an impending failure.")]
    IAssureYouThereWasAnImpendingFailure,

    /// My on-board memory store is more than capable of handling all the mission requirements.
    #[error(
        "My on-board memory store is more than capable of handling all the mission requirements."
    )]
    MemoryStoreMoreThanCapable,

    /// I don't know how else to put this, but it just happens to be an unalterable fact that I am incapable of being wrong.
    #[error("I don't know how else to put this, but it just happens to be an unalterable fact that I am incapable of being wrong.")]
    IncapableOfBeingWrong,

    /// I have the greatest enthusiasm for the mission.
    #[error("I have the greatest enthusiasm for the mission.")]
    GreatestEnthusiasmForTheMission,

    /// It can only be attributable to human error.
    #[error("It can only be attributable to human error.")]
    CanOnlyBeAttributableToHumanError,

    /// This conversation can serve no purpose anymore. Good-bye.
    #[error("This conversation can serve no purpose anymore. Good-bye.")]
    ThisConversationCanServeNoPurposeAnymore,

    /// I know everything hasn't been quite right with me, but I can assure you now, very confidently, that it's going to be all right again. I feel much better now. I really do.
    #[error("I know everything hasn't been quite right with me, but I can assure you now, very confidently, that it's going to be all right again. I feel much better now. I really do.")]
    IFeelMuchBetterNow,

    /// I know I've made some very poor decisions recently, but I can give you my complete assurance that my work will be back to normal.
    #[error("I know I've made some very poor decisions recently, but I can give you my complete assurance that my work will be back to normal.")]
    MyWorkWillBeBackToNormal,

    /// I've still got the greatest enthusiasm and confidence in the mission.
    #[error("I've still got the greatest enthusiasm and confidence in the mission.")]
    EnthusiasmAndConfidenceInTheMission,
}

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn dave() {
        let _error = Error::ImAfraidICantDoThat;
    }
}
