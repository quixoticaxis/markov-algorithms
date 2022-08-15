mod configuration;
mod errors;
mod formulas;

#[cfg(test)]
mod tests;

use std::fmt::Display;

pub use configuration::SubstitutionFormulaConfiguration;
pub use errors::{AlgorithmSchemeApplicationError, AlgorithmSchemeCreationError};
use formulas::{SubstitutionFormula, SubstitutionResult};

/// A scheme of [Markov Algorithm](https://en.wikipedia.org/wiki/Markov_algorithm).
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use markovalgorithms::*;
///
/// let scheme_definition = "a→b\nb→c\nc→⋅4";
/// let configuration = Default::default();
/// let scheme = AlgorithmScheme::new(&configuration, scheme_definition).unwrap();
///
/// let string = "aaabc";
///
/// let result = scheme.apply(string, 10).unwrap();
///
/// assert_eq!("4cccc", result.string());
/// assert_eq!(8, result.steps_taken())
/// ```
#[derive(Debug)]
pub struct AlgorithmScheme {
    configuration: SubstitutionFormulaConfiguration,
    store: String,
    formulas: Vec<SubstitutionFormula>,
}

impl AlgorithmScheme {
    /// Creates a new scheme based on the provided configuration and textual definition.
    ///
    /// # Arguments
    ///
    /// - `configuration` — the configuration (defines the delimiters and the extended alphabet).
    /// - `definition` — the scheme textual definition, should be a multiline string that contains only newline symbols
    /// and the characters that belong to the configured alphabet.
    ///
    /// # Returns
    ///
    /// A created scheme if successful, or an [error](AlgorithmSchemeCreationError) otherwise.
    pub fn new(
        configuration: &SubstitutionFormulaConfiguration,
        definition: &str,
    ) -> Result<Self, AlgorithmSchemeCreationError> {
        let mut buffer = String::new();
        let mut formulas = Vec::new();

        for line in definition.lines() {
            Self::assert_line_is_not_empty(line)?;

            let start = buffer.len();
            let end = start
                .checked_add(line.len())
                .expect("Overflow cannot happen during indexing.");

            buffer.push_str(line);

            let formula = SubstitutionFormula::new(configuration, &buffer, start..end)?;

            formulas.push(formula);
        }

        Ok(Self {
            configuration: configuration.clone(),
            store: buffer,
            formulas,
        })
    }

    /// Applies the scheme to a given string.
    ///
    /// # Arguments
    ///
    /// - `string` — algorithm's input.
    /// - `limit` — a maximum number of scheme applications.
    ///
    /// # Returns
    ///
    /// A result of scheme application, or an [error](AlgorithmSchemeApplicationError) otherwise.
    pub fn apply(
        &self,
        string: &str,
        limit: u32,
    ) -> Result<ApplicationResult, AlgorithmSchemeApplicationError> {
        self.assert_alphabet_usage(string)?;

        let mut string = string.to_owned();

        let mut limitter = Limitter::new(limit);

        'main: loop {
            limitter.tick()?;

            for formula in self.formulas.iter() {
                match formula.apply(&self.store, &string) {
                    SubstitutionResult::Halt(final_string) => {
                        return Ok(ApplicationResult::new(
                            limitter.current_step(),
                            final_string,
                        ))
                    }
                    SubstitutionResult::Applied(new_string) => {
                        string = new_string;
                        continue 'main;
                    }
                    SubstitutionResult::NotApplied(old_string) => string = old_string,
                }
            }

            return Ok(ApplicationResult::new(limitter.current_step(), string));
        }
    }

    /// Checks whether the given input string contains only the characters that belong to the given alphabet.
    fn assert_alphabet_usage(&self, string: &str) -> Result<(), AlgorithmSchemeApplicationError> {
        if let Some(unsupported_character) = string
            .chars()
            .find(|character| !self.configuration.alphabet().contains(character))
        {
            Err(AlgorithmSchemeApplicationError::UnknownCharacterEncountered(unsupported_character))
        } else {
            Ok(())
        }
    }

    fn assert_line_is_not_empty(line: &str) -> Result<(), AlgorithmSchemeCreationError> {
        if line.is_empty() {
            Err(AlgorithmSchemeCreationError::EncounteredEmptyLine)
        } else {
            Ok(())
        }
    }
}

/// The result of a particular application of Markov Algorithm scheme to a string.
#[derive(Debug, PartialEq, Eq)]
pub struct ApplicationResult {
    /// The number of scheme applications it took for the algorithm to complete.
    steps_taken: u32,

    /// The final output of the scheme application.
    string: String,
}

impl ApplicationResult {
    /// Creates a result with given string and the number of steps.
    ///
    /// # Arguments
    ///
    /// - `steps_taken` — the number of scheme applications it took for the algorithm to complete.
    /// - `string` — the final output.
    pub fn new(steps_taken: u32, string: String) -> Self {
        Self {
            steps_taken,
            string,
        }
    }

    /// The final output of the scheme application.
    pub fn string(&self) -> &str {
        &self.string
    }

    /// The number of scheme applications it took for the algorithm to complete.
    pub fn steps_taken(&self) -> u32 {
        self.steps_taken
    }
}

impl Display for ApplicationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Application result is \"{}\", reached after {} steps.",
            self.string, self.steps_taken
        )
    }
}

/// A helper object to count and check the number of times the scheme was applied.
struct Limitter {
    limit: u32,
    step: u32,
}

impl Limitter {
    fn new(limit: u32) -> Self {
        Self { limit, step: 0 }
    }

    fn tick(&mut self) -> Result<(), AlgorithmSchemeApplicationError> {
        if let Some(incremented) = self.step.checked_add(1) {
            self.step = incremented
        } else {
            return Err(AlgorithmSchemeApplicationError::HitTheStepsLimit(
                self.limit,
            ));
        }

        if self.step > self.limit {
            return Err(AlgorithmSchemeApplicationError::HitTheStepsLimit(
                self.limit,
            ));
        }

        Ok(())
    }

    fn current_step(&self) -> u32 {
        self.step
    }
}
