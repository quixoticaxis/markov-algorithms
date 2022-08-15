mod errors;

#[cfg(test)]
mod tests;

use std::ops::{Range, RangeFrom, RangeTo};

pub use errors::SubstitutionFormulaCreationError;

use crate::SubstitutionFormulaConfiguration;

#[derive(Debug)]
pub enum SubstitutionFormula {
    Simple(FormulaView),
    Final(FormulaView),
}

impl SubstitutionFormula {
    pub fn new(
        configuration: &SubstitutionFormulaConfiguration,
        store: &str,
        range: Range<usize>,
    ) -> Result<Self, SubstitutionFormulaCreationError> {
        let formula_definition = &store[range.clone()];

        let assertions = FormulaAssertions {
            formula_definition,
            configuration,
        };

        assertions.assert_alphabet_usage()?;

        assertions.assert_single_simple_delimiter()?;

        let parser = FormulaParser {
            formula_definition,
            configuration,
        };

        let ParseResult {
            is_final,
            left,
            right,
        } = parser.parse();

        assertions.assert_no_more_final_markers(left, right)?;

        let definition = FormulaView {
            full: range,
            left: ..left.len(),
            right: if right.is_empty() {
                None
            } else {
                Some(
                    formula_definition
                        .rfind(right)
                        .expect("The splitter substring is definitely in the original slice.")..,
                )
            },
        };

        Ok(if is_final {
            SubstitutionFormula::Final(definition)
        } else {
            SubstitutionFormula::Simple(definition)
        })
    }

    pub fn apply(&self, store: &str, string: &str) -> SubstitutionResult {
        let definition = self.peek_definition();

        let left = definition.get_left(store);
        let right = definition.get_right(store);

        if string.contains(left) {
            let substitution_result = string.replacen(left, right, 1);

            if self.is_final() {
                SubstitutionResult::Halt(substitution_result)
            } else {
                SubstitutionResult::Applied(substitution_result)
            }
        } else {
            SubstitutionResult::NotApplied(string.to_owned())
        }
    }

    fn peek_definition(&self) -> &FormulaView {
        match &self {
            SubstitutionFormula::Simple(definition) | SubstitutionFormula::Final(definition) => {
                definition
            }
        }
    }

    fn is_final(&self) -> bool {
        matches!(self, SubstitutionFormula::Final(_))
    }
}

#[derive(Debug)]
pub struct FormulaView {
    full: Range<usize>,
    left: RangeTo<usize>,
    right: Option<RangeFrom<usize>>,
}

impl FormulaView {
    pub fn get_left<'a>(&self, store: &'a str) -> &'a str {
        &store[self.full.clone()][self.left]
    }

    pub fn get_right<'a>(&self, store: &'a str) -> &'a str {
        if let Some(right) = self.right.clone() {
            &store[self.full.clone()][right]
        } else {
            ""
        }
    }
}

struct FormulaAssertions<'a> {
    formula_definition: &'a str,
    configuration: &'a SubstitutionFormulaConfiguration,
}

impl<'a> FormulaAssertions<'a> {
    fn assert_alphabet_usage(&self) -> Result<(), SubstitutionFormulaCreationError> {
        if let Some(unsupported_character) = self.formula_definition.chars().find(|character| {
            !self.configuration.alphabet().contains(character)
                && *character != self.configuration.delimiter()
                && *character != self.configuration.final_marker()
        }) {
            Err(
                SubstitutionFormulaCreationError::UnknownCharacterEncountered(
                    self.formula_definition.to_owned(),
                    unsupported_character,
                ),
            )
        } else {
            Ok(())
        }
    }

    fn assert_single_simple_delimiter(&self) -> Result<(), SubstitutionFormulaCreationError> {
        match self
            .formula_definition
            .match_indices(self.configuration.delimiter())
            .count()
        {
            0 => Err(SubstitutionFormulaCreationError::NoDelimiterFound(
                self.formula_definition.to_owned(),
            )),
            1 => Ok(()),
            n => Err(SubstitutionFormulaCreationError::MultipleDelimitersFound(
                self.formula_definition.to_owned(),
                n,
            )),
        }
    }

    fn assert_no_more_final_markers(
        &self,
        left: &str,
        right: &str,
    ) -> Result<(), SubstitutionFormulaCreationError> {
        if left.contains(self.configuration.final_marker()) {
            return Err(SubstitutionFormulaCreationError::FinalMarkerOnTheLeft(
                self.formula_definition.to_owned(),
            ));
        }
        if right.contains(self.configuration.final_marker()) {
            return Err(SubstitutionFormulaCreationError::FinalMarkerOnTheRight(
                self.formula_definition.to_owned(),
            ));
        }
        Ok(())
    }
}

struct FormulaParser<'a> {
    formula_definition: &'a str,
    configuration: &'a SubstitutionFormulaConfiguration,
}

impl<'a> FormulaParser<'a> {
    fn parse(&self) -> ParseResult<'a> {
        let is_final = self
            .formula_definition
            .contains(self.configuration.final_delimiter());

        let splitted: Vec<_> = if is_final {
            self.formula_definition
                .split(self.configuration.final_delimiter())
                .collect()
        } else {
            self.formula_definition
                .split(self.configuration.delimiter())
                .collect()
        };

        debug_assert!(
            splitted.len() == 2,
            "A sanity check for matching and split logic is violated, \
            there are {} parts after splitting by a single delimiter.",
            splitted.len()
        );

        ParseResult {
            is_final,
            left: splitted[0],
            right: splitted[1],
        }
    }
}

#[derive(Debug)]
struct ParseResult<'a> {
    is_final: bool,
    left: &'a str,
    right: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SubstitutionResult {
    Halt(String),
    Applied(String),
    NotApplied(String),
}
