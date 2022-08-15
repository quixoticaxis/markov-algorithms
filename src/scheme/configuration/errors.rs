use thiserror::Error;

/// An error that occures during substitution formula configuration creation.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum SubstitutionFormulaConfigurationCreationError {
    /// The formula delimiter belongs to the alphabet.
    #[error("the formula delimiter belongs to the alphabet")]
    DelimiterIsPartOfTheAlphabet,
    /// The final marker belongs to the alphabet.
    #[error("the final marker belongs to the alphabet")]
    FinalMarkerIsPartOfTheAlphabet,
}
