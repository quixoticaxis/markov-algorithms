use thiserror::Error;

/// An error that occures during substitution formula creation.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum SubstitutionFormulaCreationError {
    /// An unsupported character that neither belongs to the alphabet,
    /// nor is a delimiter is encountered in formula.
    #[error("an unsupported character '{1}' that neither belongs to the alphabet, nor is a delimiter is encountered in the substitution formula \"{0}\"")]
    UnknownCharacterEncountered(String, char),
    /// No delimiters are found in the formula definition.
    #[error("no delimiters are found in the substitution formula \"{0}\"")]
    NoDelimiterFound(String),
    /// Multiple delimiters are found in the formula definition.
    #[error("multiple delimiters are found in the substitution formula \"{0}\"")]
    MultipleDelimitersFound(String, usize),
    /// Final marker is on the left side of the subsstitution formula.
    #[error("final marker is on the left side of the substitution formula \"{0}\"")]
    FinalMarkerOnTheLeft(String),
    /// Final marker is on the right side of the subsstitution formula.
    #[error("Final marker is on the right side of the substitution formula \"{0}\"")]
    FinalMarkerOnTheRight(String),
}
