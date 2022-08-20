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

//! [AlgorithmSchemeBuilder](AlgorithmSchemeBuilder) structure and its trait implementations.

#[cfg(test)]
mod tests;

use std::collections::HashSet;

use thiserror::Error;

use crate::{
    alphabet::Alphabet,
    scheme::{AlgorithmScheme, SchemeProperties, SubstitutionFormulaDefinitionError},
};

use super::SubstitutionFormula;

/// A builder to configure an algorithm scheme.
///
/// # Example
/// Basic usage:
/// ```rust
/// # use std::str;
/// use markovalgorithms::prelude::*;
///
/// let builder = AlgorithmSchemeBuilder::new()
///     .with_alphabet(str::parse("abc").unwrap())
///     .with_delimiter('→')
///     .with_final_marker('⋅')
///     .build_with_formula_definitions(
///         [ "a→b", "b→c" ].into_iter())
///     .unwrap();
/// ```
/// The defaults that should be enough to construct simple algorithms are provided.
/// '→' as delimiter, '⋅' as final marker, alphabet containing ranges from `'a'` to `'z'`,
/// from `'A'` to `'Z'`, and digits.
/// ```rust
/// use markovalgorithms::prelude::*;
/// let builder = AlgorithmSchemeBuilder::default()
///     .build_with_formula_definitions(
///         [ "a→b", "b→c" ].into_iter())
///     .unwrap();
/// ```
#[derive(Clone)]
pub struct AlgorithmSchemeBuilder {
    alphabet: Option<Alphabet>,
    delimiter: Option<char>,
    final_marker: Option<char>,
}

impl AlgorithmSchemeBuilder {
    const DEFAULT_DELIMITER: char = '→';
    const DEFAULT_FINAL_MARKER: char = '⋅';

    /// Creates a new builder.
    pub fn new() -> Self {
        Self {
            alphabet: None,
            delimiter: None,
            final_marker: None,
        }
    }

    /// Adds a delimiter to the builder.
    ///
    /// May be called multiple times in order to replace the prior delimiter.
    pub fn with_delimiter(mut self, delimiter: char) -> Self {
        _ = self.delimiter.insert(delimiter);
        self
    }

    /// Adds a final marker to the builder.
    ///
    /// May be called multiple times in order to replace the prior final marker.
    pub fn with_final_marker(mut self, final_marker: char) -> Self {
        _ = self.final_marker.insert(final_marker);
        self
    }

    /// Adds an alphabet to the builder.
    ///
    /// May be called multiple times in order to replace the prior final marker.
    pub fn with_alphabet(mut self, alphabet: Alphabet) -> Self {
        _ = self.alphabet.insert(alphabet);
        self
    }

    /// Builds an algorithm scheme based on the provided definitions.
    ///
    /// # Example
    /// Basic usage:
    /// ```rust
    /// use markovalgorithms::prelude::*;
    ///
    /// let scheme = AlgorithmSchemeBuilder::new()
    ///     .build_with_formula_definitions(
    ///         "a→b\nb→c".lines())
    ///     .unwrap();
    /// ```
    pub fn build_with_formula_definitions<'a, I>(
        self,
        formula_definitions: I,
    ) -> Result<AlgorithmScheme, AlgorithmSchemeDefinitionError>
    where
        I: Iterator<Item = &'a str>,
    {
        let properties = self.finalize_properties();

        let assertions = PropertyAssertions::new(&properties);

        assertions.assert_all_properties_are_valid()?;

        let mut collection_builder = SubstitutionFormulaCollectionBuilder::new(&properties);

        for formula_definition in formula_definitions {
            assertions.assert_definition_conforms_to_properties(formula_definition)?;

            collection_builder.try_add_formula(formula_definition)?;
        }

        let SubstitutionFormulaCollectionBuilder {
            properties: _,
            store,
            substitution_formulas,
        } = collection_builder;

        Ok(AlgorithmScheme {
            properties,
            store,
            substitution_formulas,
        })
    }

    /// Creates a struct with properties to no longer use options.
    fn finalize_properties(self) -> SchemeProperties {
        SchemeProperties {
            delimiter: self.delimiter.unwrap_or(Self::DEFAULT_DELIMITER),
            final_marker: self.final_marker.unwrap_or(Self::DEFAULT_FINAL_MARKER),
            alphabet: self.alphabet.unwrap_or_else(Self::create_default_alphabet),
        }
    }

    fn create_default_alphabet() -> Alphabet {
        let default_alphabet_set: HashSet<_> = ('a'..='z')
            .into_iter()
            .chain(('A'..='Z').into_iter())
            .chain(('0'..='9').into_iter())
            .collect();

        Alphabet::try_from(&default_alphabet_set).unwrap()
    }
}

impl Default for AlgorithmSchemeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Additional methods to assert the validity of the delimiter,
/// the final marker, and the alphabet.
struct PropertyAssertions<'a> {
    properties: &'a SchemeProperties,
}

impl<'a> PropertyAssertions<'a> {
    fn new(properties: &'a SchemeProperties) -> Self {
        Self { properties }
    }

    fn assert_definition_conforms_to_properties(
        &self,
        formula_definition: &str,
    ) -> Result<(), AlgorithmSchemeDefinitionError> {
        let invalid_characters = formula_definition
            .matches(|character| {
                !self.properties.alphabet.contains_extended(character)
                    && character != self.properties.delimiter
                    && character != self.properties.final_marker
            })
            .fold(String::new(), |mut accumulator, character| {
                accumulator.push_str(character);
                accumulator
            });

        if invalid_characters.is_empty() {
            Ok(())
        } else {
            Err(AlgorithmSchemeDefinitionError::UnknownCharactersEncountered(invalid_characters))
        }
    }

    fn assert_all_properties_are_valid(&self) -> Result<(), AlgorithmSchemeDefinitionError> {
        if self.properties.delimiter == self.properties.final_marker {
            Err(
                AlgorithmSchemeDefinitionError::DelimiterAndFinalMarkerAreTheSame(
                    self.properties.delimiter,
                ),
            )
        } else if self.properties.alphabet.contains(self.properties.delimiter) {
            Err(
                AlgorithmSchemeDefinitionError::DelimiterBelongsToTheAlphabet(
                    self.properties.delimiter,
                ),
            )
        } else if self
            .properties
            .alphabet
            .contains(self.properties.final_marker)
        {
            Err(
                AlgorithmSchemeDefinitionError::FinalMarkerBelongsToTheAlphabet(
                    self.properties.final_marker,
                ),
            )
        } else {
            Ok(())
        }
    }
}

/// A helper type to create and colelct the substitution formulas,
/// created over a single [String](std::str::String) buffer.
struct SubstitutionFormulaCollectionBuilder<'a> {
    store: String,
    substitution_formulas: Vec<SubstitutionFormula>,
    properties: &'a SchemeProperties,
}

impl<'a> SubstitutionFormulaCollectionBuilder<'a> {
    fn new(properties: &'a SchemeProperties) -> Self {
        Self {
            store: String::new(),
            substitution_formulas: Vec::new(),
            properties,
        }
    }

    fn try_add_formula(
        &mut self,
        formula_definition: &str,
    ) -> Result<(), AlgorithmSchemeDefinitionError> {
        let start = self.store.len();
        self.store.push_str(formula_definition);
        let end = self.store.len();

        match SubstitutionFormula::new(self.properties, &self.store, start..end) {
            Ok(formula) => self.substitution_formulas.push(formula),
            Err(error) => {
                return Err(AlgorithmSchemeDefinitionError::FormulaCreationError { source: error })
            }
        }

        Ok(())
    }
}

/// An error in the algorithm definition.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AlgorithmSchemeDefinitionError {
    /// The same character cannot be used as a delimiter and as a final marker.
    #[error("the same character '{0}' cannot be used as a delimiter and as a final marker")]
    DelimiterAndFinalMarkerAreTheSame(char),
    /// The delimiter cannot belong to the alphabet.
    #[error(
        "the character '{0}' cannot be used as a delimiter because it belongs to the alphabet"
    )]
    DelimiterBelongsToTheAlphabet(char),
    /// The final marker cannot belong to the alphabet.
    #[error(
        "the character '{0}' cannot be used as a final marker because it belongs to the alphabet"
    )]
    FinalMarkerBelongsToTheAlphabet(char),
    /// An error encountered during the creation of substitution formulas.
    #[error("encountered an issue during the creation of substitution formulas: {source}")]
    FormulaCreationError {
        source: SubstitutionFormulaDefinitionError,
    },
    /// The definition of the scheme cannot contain the characters that neither belong to the alphabet, \
    /// nor are delimiter or final marker.
    #[error("the definition of the scheme contains the characters that neither belong to the alphabet, \
    nor are delimiter or final marker (unknown characters: \"{0}\")")]
    UnknownCharactersEncountered(String),
}
