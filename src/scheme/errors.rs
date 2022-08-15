use thiserror::Error;

use super::formulas::SubstitutionFormulaCreationError;

/// An error that occures during the creation of a scheme.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum AlgorithmSchemeCreationError {
    /// There is an empty line in the scheme definition.
    #[error("an empty line was encountered in the scheme definition")]
    EncounteredEmptyLine,
    /// One of the lines fails to be parsed as a substitution formula. See the source error.
    #[error("failed to created substitution formula: \"{source}\"")]
    FormulaCreationFailed {
        #[from]
        source: SubstitutionFormulaCreationError,
    },
}

/// An error that occures during the application of a scheme.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum AlgorithmSchemeApplicationError {
    /// An unsupported character that is not part of the alphabet is found in the input.
    #[error("an unsupported character '{0}' that is not part of the alphabet is found in the input")]
    UnknownCharacterEncountered(char),
    /// The executor stops after the limit of applications is reached.
    #[error("the executor is not completed after reaching step {0}")]
    HitTheStepsLimit(u32),
}
