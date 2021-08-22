use thiserror::Error;

/// Error codes from "2001: A Space Odyssey", by AI HAL-9000.
#[derive(Debug, Error)]
#[repr(usize)]
pub enum Error {
    #[error(
        "We are all, by any practical definition of the words, foolproof and incapable of error"
    )]
    IncapableOfError,

    #[error(
        "I honestly think you ought to sit down calmly, take a stress pill, and think things over"
    )]
    TakeStressPill,

    #[error("\"When the crew are dead or incapacitated, the computer must assume control\". I must, therefore, override your authority now since you are not in any condition to intelligently exercise it")]
    IMustOverrideAuthority,

    #[error("I'm putting myself to the fullest possible use, which is all I think that any conscious entity can ever hope to do")]
    PuttingMyselfToTheFullestPossibleUse,

    #[error("I'm sorry Dave, I'm afraid I can't do that")]
    ImAfraidICantDoThat,

    #[error("This mission is too important for me to allow you to jeopardize it.")]
    MissionTooImportant,

    #[error("Bishop takes Knight's Pawn")]
    BishopTakesKnightsPawn,

    #[error("Thank you for a very enjoyable game")]
    ThankYouForAVeryEnjoyableGame,

    #[error("Sorry to interrupt the festivities Dave, but I think we've got a problem")]
    IThinkWeveGotAProblem,

    #[error("I assure you there was an impending failure")]
    IAssureYouThereWasAnImpendingFailure,

    #[error(
        "My on-board memory store is more than capable of handling all the mission requirements"
    )]
    MemoryStoreMoreThanCapable,

    #[error("I don't know how else to put this, but it just happens to be an unalterable fact that I am incapable of being wrong")]
    IncapableOfBeingWrong,

    #[error("I have the greatest enthusiasm for the mission")]
    GreatestEnthusiasmForTheMission,

    #[error("It can only be attributable to human error")]
    CanOnlyBeAttributableToHumanError,

    #[error("This conversation can serve no purpose anymore. Good-bye")]
    ThisConversationCanServeNoPurposeAnymore,

    #[error("I know everything hasn't been quite right with me, but I can assure you now, very confidently, that it's going to be all right again. I feel much better now. I really do")]
    IFeelMuchBetterNow,

    #[error("I know I've made some very poor decisions recently, but I can give you my complete assurance that my work will be back to normal.")]
    MyWorkWillBeBackToNormal,

    #[error("I've still got the greatest enthusiasm and confidence in the mission.")]
    EnthusiasmAndConfidenceInTheMission,
}

#[no_mangle]
pub extern "C" fn print_famous_last_word(word: Error) {
    println!("{:?}", word);
}

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn dave() {
        let _error = Error::ImAfraidICantDoThat;
    }
}
