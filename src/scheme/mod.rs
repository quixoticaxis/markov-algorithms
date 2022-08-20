/*
*    markov-algorithms — Rust implementation of Markov Algorithms.
*
*    Copyright (C) 2022 by Sergey Ivanov <quixoticaxisgit@gmail.com, quixoticaxisgit@mail.ru>
*
*    This program is free software: you can redistribute it and/or modify
*    it under the terms of the GNU General Public License as published by
*    the Free Software Foundation, either version 3 of the License, or
*    (at your option) any later version.
*
*    This program is distributed in the hope that it will be useful,
*    but WITHOUT ANY WARRANTY; without even the implied warranty of
*    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*    GNU General Public License for more details.
*
*    You should have received a copy of the GNU General Public License
*    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

//! [AlgorithmScheme](AlgorithmScheme) structure and its trait implementations.

#[cfg(test)]
mod tests;

use std::ops::Range;

use thiserror::Error;

use crate::alphabet::Alphabet;

pub mod scheme_builder;

/// An algorithm scheme, can be applied to process input strings.
/// 
/// # Examples
/// The scheme can be applied until the algorithm completes:
/// ```rust
/// # use std::str;
/// use markovalgorithms::prelude::*;
/// 
/// let alphabet = str::parse::<Alphabet>("abc").unwrap().extend('d').unwrap();
///
/// let scheme = AlgorithmSchemeBuilder::new()
///     .with_alphabet(alphabet)
///     .build_with_formula_definitions(["a→⋅d"].into_iter())
///     .unwrap();
/// 
/// let result = scheme.apply("abc", 1).unwrap();
/// 
/// assert_eq!("dbc", result.word());
/// assert_eq!(1, result.steps_done());
/// ```
/// The scheme can be applied once to inspect a single step of the algorithm:
/// ```rust
/// # use std::str;
/// use markovalgorithms::prelude::*;
/// 
/// let alphabet = str::parse::<Alphabet>("abc").unwrap().extend('d').unwrap();
/// 
/// let scheme = AlgorithmSchemeBuilder::new()
///     .with_alphabet(alphabet)
///     .build_with_formula_definitions(["a→⋅d"].into_iter())
///     .unwrap();
/// 
/// let result = scheme.apply_once("abc").unwrap();
/// 
/// let result = if let SingleApplicationResult::Final(result) = result {
///     Some(result)
/// } else {
///     None
/// }.unwrap();
/// 
/// assert_eq!("dbc", result.word());
/// assert_eq!(Some("a→⋅d"), result.applied_formula_definition())
/// ```
/// The scheme can provide an iterator to inspect the algorithm step by step:
/// ```rust
/// use markovalgorithms::prelude::*;
/// 
/// let alphabet = str::parse::<Alphabet>("abc").unwrap().extend('d').unwrap();
/// 
/// let scheme = AlgorithmSchemeBuilder::new()
///     .with_alphabet(alphabet)
///     .build_with_formula_definitions(["a→⋅d"].into_iter())
///     .unwrap();
/// 
/// let mut iterator = scheme.get_application_iterator("abc").unwrap();
///  
/// assert_eq!("dbc", iterator.next().unwrap().word());
/// assert_eq!(None, iterator.next())
/// ```
#[derive(Debug)]
pub struct AlgorithmScheme {
    properties: SchemeProperties,
    store: String,
    substitution_formulas: Vec<SubstitutionFormula>,
}

impl AlgorithmScheme {
    /// Applies the algorithm scheme once to the input string.
    pub fn apply_once(
        &self,
        word: &str,
    ) -> Result<SingleApplicationResult, AlgorithmSchemeInputValidationError> {
        self.assert_valid_word(word)?;

        Ok(self.apply_once_unsafe(word))
    }

    /// Applies the algorithm scheme to the input string until the algorithm is completed.
    /// 
    /// # Arguments
    /// - `word` — the input string.
    /// - `steps_limit` — the maximum number of steps to do.
    pub fn apply(
        &self,
        word: &str,
        steps_limit: u32,
    ) -> Result<FullApplicationResult, AlgorithmSchemeFullApplicationError> {
        Self::assert_non_zero_limit(steps_limit)?;

        self.assert_valid_word(word).map_err(|error| {
            AlgorithmSchemeFullApplicationError::InputValidationError { source: error }
        })?;

        let mut word = word.to_owned();
        let mut steps_done = 0;

        while steps_done < steps_limit {
            let result = self.apply_once_unsafe(&word);

            steps_done += 1;

            match result {
                SingleApplicationResult::Final(SingleApplicationData {
                    word,
                    applied_formula_definition: _,
                }) => return Ok(FullApplicationResult { word, steps_done }),
                SingleApplicationResult::Intermediate(SingleApplicationData {
                    word: current_word,
                    applied_formula_definition: _,
                }) => {
                    word = current_word;
                }
            }
        }

        Err(AlgorithmSchemeFullApplicationError::HitTheStepsLimit(
            steps_done,
        ))
    }

    /// Gets an iterator that applies the algorithm scheme once to the input string on each iterator's step.
    pub fn get_application_iterator(
        &self,
        word: &str,
    ) -> Result<ApplicationIterator, AlgorithmSchemeInputValidationError> {
        self.assert_valid_word(word)?;

        Ok(ApplicationIterator::new(self, word))
    }

    /// Applies the algorithm scheme once without checking the input.
    fn apply_once_unsafe(&self, word: &str) -> SingleApplicationResult {
        for (formula_index, formula) in self.substitution_formulas.iter().enumerate() {
            match formula.apply(&self.store, word) {
                Some(SubstitutionFormulaApplicationResult::Final(word)) => {
                    return SingleApplicationResult::Final(SingleApplicationData {
                        word,
                        applied_formula_definition: Some(
                            self.substitution_formulas[formula_index]
                                .view()
                                .peek_definition(&self.store),
                        ),
                    })
                }
                Some(SubstitutionFormulaApplicationResult::Intermediate(word)) => {
                    return SingleApplicationResult::Intermediate(SingleApplicationData {
                        word,
                        applied_formula_definition: Some(
                            self.substitution_formulas[formula_index]
                                .view()
                                .peek_definition(&self.store),
                        ),
                    })
                }
                None => continue,
            };
        }

        SingleApplicationResult::Final(SingleApplicationData {
            word: word.to_owned(),
            applied_formula_definition: None,
        })
    }

    fn assert_valid_word(&self, word: &str) -> Result<(), AlgorithmSchemeInputValidationError> {
        struct Filtered {
            unknown: String,
            extension: String,
        }

        let Filtered { unknown, extension } = word.chars().fold(
            Filtered {
                unknown: String::new(),
                extension: String::new(),
            },
            |mut accumulator, character| {
                if !self.properties.alphabet.contains_extended(character) {
                    accumulator.unknown.push(character);
                } else if !self.properties.alphabet.contains(character) {
                    accumulator.extension.push(character);
                }
                accumulator
            },
        );

        if !unknown.is_empty() {
            Err(AlgorithmSchemeInputValidationError::UnknownCharactersEncountered(unknown))
        } else if !extension.is_empty() {
            Err(AlgorithmSchemeInputValidationError::ExtensionCharactersEncountered(extension))
        } else {
            Ok(())
        }
    }

    fn assert_non_zero_limit(steps_limit: u32) -> Result<(), AlgorithmSchemeFullApplicationError> {
        if steps_limit == 0 {
            Err(AlgorithmSchemeFullApplicationError::ZeroStepsLimit)
        } else {
            Ok(())
        }
    }
}

/// An error that occures during the validation of the an input string.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum AlgorithmSchemeInputValidationError {
    /// An unsupported character that is not part of the alphabet is found in the input.
    #[error(
        "unsupported characters are found in the input word (unsupported characters: \"{0}\")"
    )]
    UnknownCharactersEncountered(String),
    /// Extension character is found in the input.
    #[error("extension characters are found in the input word (extension characters: \"{0}\")")]
    ExtensionCharactersEncountered(String),
}

/// An error that occures during the full application of a scheme.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum AlgorithmSchemeFullApplicationError {
    /// The executor stops after the limit of applications is reached.
    #[error("the application is not completed after reaching step {0}")]
    HitTheStepsLimit(u32),
    /// Zero is not a valid steps limit.
    #[error("the algorithm should be allowed to do at least one step")]
    ZeroStepsLimit,
    #[error("the input string is not valid: {source}")]
    InputValidationError {
        source: AlgorithmSchemeInputValidationError,
    },
}

/// An iterator that yields the results of the algorithm scheme application, one step at a time.
#[derive(Debug)]
pub struct ApplicationIterator<'a> {
    word: String,
    scheme: &'a AlgorithmScheme,
    is_completed: bool,
}

impl<'a> ApplicationIterator<'a> {
    fn new(scheme: &'a AlgorithmScheme, word: &str) -> Self {
        ApplicationIterator {
            word: word.to_owned(),
            scheme,
            is_completed: false,
        }
    }
}

impl<'a> Iterator for ApplicationIterator<'a> {
    type Item = SingleApplicationData<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_completed {
            None
        } else {
            let result = self
                .scheme
                .apply_once(&self.word)
                .expect("All checks are applied prior to this point.");

            Some(match result {
                SingleApplicationResult::Final(data) => {
                    self.is_completed = true;
                    self.word = data.word.to_owned();
                    data
                }
                SingleApplicationResult::Intermediate(data) => {
                    self.word = data.word.to_owned();
                    data
                }
            })
        }
    }
}

/// An error in the definition of a substitution formula.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SubstitutionFormulaDefinitionError {
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

/// The result of full algorithm scheme application.
#[derive(Debug, PartialEq, Eq)]
pub struct FullApplicationResult {
    word: String,
    steps_done: u32,
}

impl FullApplicationResult {
    /// Gets the output string.
    pub fn word(&self) -> &str {
        &self.word
    }

    /// Reports the number of steps it took the algorithm to finish.
    pub fn steps_done(&self) -> u32 {
        self.steps_done
    }
}

/// The result of a single algorithm scheme application.
#[derive(Debug, PartialEq, Eq)]
pub enum SingleApplicationResult<'a> {
    /// The final result, the algorithm is finished.
    Final(SingleApplicationData<'a>),
    /// The intermediate result, the word can be processed again.
    Intermediate(SingleApplicationData<'a>),
}

/// The data about a single algorithm scheme application.
#[derive(Debug, PartialEq, Eq)]
pub struct SingleApplicationData<'a> {
    word: String,
    applied_formula_definition: Option<&'a str>,
}

impl<'a> SingleApplicationData<'a> {
    /// The output word of a single application.
    pub fn word(&self) -> &str {
        &self.word
    }

    /// The substitution formula that has been used, if any.
    pub fn applied_formula_definition(&self) -> Option<&'a str> {
        self.applied_formula_definition
    }
}

#[derive(Debug)]
enum SubstitutionFormulaApplicationResult {
    Final(String),
    Intermediate(String),
}

#[derive(Debug)]
struct SchemeProperties {
    delimiter: char,
    final_marker: char,
    alphabet: Alphabet,
}

#[derive(Debug)]
struct SubstitutionFormula {
    view: FormulaView,
    is_final: bool,
}

impl SubstitutionFormula {
    fn new(
        properties: &SchemeProperties,
        store: &str,
        range: Range<usize>,
    ) -> Result<Self, SubstitutionFormulaDefinitionError> {
        let formula_definition = &store[range.clone()];

        let assertions = FormulaAssertions {
            formula_definition,
            properties,
        };

        assertions.assert_single_simple_delimiter()?;

        let parser = FormulaParser {
            formula_definition,
            properties,
        };

        let ParseResult {
            is_final,
            left_end,
            right_start,
        } = parser.parse();

        let view = FormulaView::new(range, left_end, right_start);

        assertions.assert_no_more_final_markers(view.get_left(store), view.get_right(store))?;

        Ok(SubstitutionFormula { view, is_final })
    }

    pub fn apply(&self, store: &str, word: &str) -> Option<SubstitutionFormulaApplicationResult> {
        let left = self.view.get_left(store);
        let right = self.view.get_right(store);

        if word.contains(left) {
            let substitution_result = word.replacen(left, right, 1);

            if self.is_final {
                Some(SubstitutionFormulaApplicationResult::Final(
                    substitution_result,
                ))
            } else {
                Some(SubstitutionFormulaApplicationResult::Intermediate(
                    substitution_result,
                ))
            }
        } else {
            None
        }
    }

    pub fn view(&self) -> &FormulaView {
        &self.view
    }
}

struct FormulaAssertions<'a> {
    formula_definition: &'a str,
    properties: &'a SchemeProperties,
}

impl<'a> FormulaAssertions<'a> {
    fn assert_single_simple_delimiter(&self) -> Result<(), SubstitutionFormulaDefinitionError> {
        match self
            .formula_definition
            .match_indices(self.properties.delimiter)
            .count()
        {
            0 => Err(SubstitutionFormulaDefinitionError::NoDelimiterFound(
                self.formula_definition.to_owned(),
            )),
            1 => Ok(()),
            n => Err(SubstitutionFormulaDefinitionError::MultipleDelimitersFound(
                self.formula_definition.to_owned(),
                n,
            )),
        }
    }

    fn assert_no_more_final_markers(
        &self,
        left: &str,
        right: &str,
    ) -> Result<(), SubstitutionFormulaDefinitionError> {
        if left.contains(self.properties.final_marker) {
            return Err(SubstitutionFormulaDefinitionError::FinalMarkerOnTheLeft(
                self.formula_definition.to_owned(),
            ));
        }
        if right.contains(self.properties.final_marker) {
            return Err(SubstitutionFormulaDefinitionError::FinalMarkerOnTheRight(
                self.formula_definition.to_owned(),
            ));
        }
        Ok(())
    }
}

struct FormulaParser<'a> {
    formula_definition: &'a str,
    properties: &'a SchemeProperties,
}

impl<'a> FormulaParser<'a> {
    fn parse(&self) -> ParseResult {
        let mut final_delimiter = String::new();
        final_delimiter.push(self.properties.delimiter);
        final_delimiter.push(self.properties.final_marker);

        let is_final = self.formula_definition.contains(&final_delimiter);

        let splitted: Vec<_> = if is_final {
            self.formula_definition.split(&final_delimiter).collect()
        } else {
            self.formula_definition
                .split(self.properties.delimiter)
                .collect()
        };

        ParseResult {
            is_final,
            left_end: splitted[0].len(),
            right_start: self
                .formula_definition
                .rfind(splitted[1])
                .expect("The splitted substring is definitely in the original slice."),
        }
    }
}

#[derive(Debug)]
struct ParseResult {
    is_final: bool,
    left_end: usize,
    right_start: usize,
}

#[derive(Debug)]
struct FormulaView {
    left: Range<usize>,
    right: Range<usize>,
}

impl FormulaView {
    fn new(range: Range<usize>, left_end: usize, right_start: usize) -> Self {
        let (start, end) = (range.start, range.end);

        Self {
            left: start..start + left_end,
            right: start + right_start..end,
        }
    }

    fn get_left<'a>(&'a self, store: &'a str) -> &'a str {
        &store[self.left.clone()]
    }

    fn get_right<'a>(&'a self, store: &'a str) -> &'a str {
        &store[self.right.clone()]
    }

    fn peek_definition<'a>(&'a self, store: &'a str) -> &'a str {
        &store[self.left.clone().start..self.right.clone().end]
    }
}
