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

//! [Alphabet](Alphabet) structure and its trait implementations.
use std::{collections::HashSet, str::FromStr};

use thiserror::Error;

#[cfg(test)]
mod tests;

/// An alphabet that contains the main set of characters and an extension.
///
/// # Example
/// Basic usage:
/// ```rust
/// # use std::str;
/// use markovalgorithms::prelude::Alphabet;
///
/// let alphabet = str::parse::<Alphabet>("ab").unwrap()
///     .extend('|').unwrap()
///     .extend('+').unwrap();
///
/// assert!(alphabet.contains('a') && alphabet.contains('b'));
/// assert!(!alphabet.contains('|') && !alphabet.contains('+'));
/// assert!(alphabet.contains_extended('|') && alphabet.contains_extended('+'));
/// ```
///
/// An alphabet can also be created based on non-empty [HashSet](std::collections::HashSet) of `char`s:
/// ```rust
/// # use std::collections::HashSet;
/// use markovalgorithms::prelude::Alphabet;
///
/// let set: HashSet<_> = ('a'..='z').into_iter().collect();
/// let alphabet = Alphabet::try_from(&set).unwrap();
///
/// assert!(alphabet.contains('k'));
/// ```
#[derive(Debug, Clone)]
pub struct Alphabet {
    main: HashSet<char>,
    extension: HashSet<char>,
}

impl Alphabet {
    /// Checks whether the character belongs to the alphabet.
    ///
    /// # Returns
    /// `true`, if the character belongs to the alphabet, and `false` otherwise.
    pub fn contains(&self, character: char) -> bool {
        self.main.contains(&character)
    }

    /// Checks whether the character belongs to the alphabet or its extension.
    ///
    /// # Returns
    /// `true`, if the character belongs to the alphabet or its extension, and `false` otherwise.
    pub fn contains_extended(&self, character: char) -> bool {
        self.main.contains(&character) || self.extension.contains(&character)
    }

    /// Extends the alphabet with a given character.
    ///
    /// # Returns
    /// Consumes and returns `self`.
    ///
    /// # Errors
    /// Returns an [error](AlphabetDefinitionError)
    /// if the character belongs to the alphabet or its extension.
    pub fn extend(mut self, character: char) -> Result<Self, AlphabetDefinitionError> {
        if self.contains_extended(character) {
            Err(AlphabetDefinitionError::ExtendedWithADuplicate)
        } else {
            let inserted = self.extension.insert(character);

            debug_assert!(inserted);

            Ok(self)
        }
    }
}

impl FromStr for Alphabet {
    type Err = AlphabetDefinitionError;

    fn from_str(characters: &str) -> Result<Self, Self::Err> {
        let mut store = HashSet::new();
        let mut duplicates = None;

        for character in characters.chars() {
            if !store.insert(character) {
                duplicates.get_or_insert_with(Vec::new).push(character);
            }
        }

        if let Some(duplicates) = duplicates {
            Err(AlphabetDefinitionError::DuplicatedCharacterEncountered {
                duplicates: String::from_iter(duplicates),
                alphabet_definition: characters.to_owned(),
            })
        } else {
            Ok(Self {
                main: store,
                extension: HashSet::new(),
            })
        }
    }
}

impl<'a> TryFrom<&'a str> for Alphabet {
    type Error = AlphabetDefinitionError;

    fn try_from(characters: &'a str) -> Result<Self, Self::Error> {
        Self::from_str(characters)
    }
}

impl<S> TryFrom<&HashSet<char, S>> for Alphabet {
    type Error = AlphabetDefinitionError;

    fn try_from(other: &HashSet<char, S>) -> Result<Self, Self::Error> {
        if other.is_empty() {
            Err(AlphabetDefinitionError::NoCharacters)
        } else {
            Ok(Self {
                main: HashSet::from_iter(other.iter().cloned()),
                extension: HashSet::new(),
            })
        }
    }
}

/// An error in the alphabet definition.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AlphabetDefinitionError {
    /// The same character cannot be included in the alphabet multiple times.
    #[error(
        "the same character cannot be included in the alphabet multiple times \
        (original definition: \"{alphabet_definition}\"), duplicate characters: \"{duplicates}\""
    )]
    DuplicatedCharacterEncountered {
        duplicates: String,
        alphabet_definition: String,
    },
    /// An alphabet cannot be empty.
    #[error("an alphabet cannot be empty")]
    NoCharacters,
    /// An alphabet cannot be extended with duplicate characters.
    #[error("an alphabet cannot be extended with duplicate characters")]
    ExtendedWithADuplicate,
}
