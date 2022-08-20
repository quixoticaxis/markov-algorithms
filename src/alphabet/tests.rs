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

use std::{
    collections::HashSet,
    str::{self, FromStr},
};

use super::*;

#[test]
fn alphabet_can_be_created_from_a_hash_set() {
    let set: HashSet<_> = ('a'..='z').into_iter().collect();

    let creation_result = Alphabet::try_from(&set);

    assert!(creation_result.is_ok());
}

#[test]
fn alphabet_cannot_be_created_from_an_empty_hash_set() {
    let set: HashSet<_> = Default::default();

    let error = Alphabet::try_from(&set).unwrap_err();

    let expected_error = AlphabetDefinitionError::NoCharacters;

    assert_eq!(expected_error, error);
}

#[test]
fn creation_from_hashset_error_is_correctly_displayed() {
    let set: HashSet<_> = Default::default();

    let error = Alphabet::try_from(&set).unwrap_err();

    let expected_error = "an alphabet cannot be empty";

    assert_eq!(expected_error, &format!("{error}"));
}

#[test]
fn alphabet_can_be_parsed_from_a_string_if_it_has_no_duplicate_characters() {
    let definition = "abc";

    let parsing_result = str::parse::<Alphabet>(definition);

    assert!(parsing_result.is_ok());
}

#[test]
fn alphabet_cannot_be_parsed_from_a_string_if_it_has_duplicate_characters() {
    let definition = "abbcdde";

    let error = str::parse::<Alphabet>(definition).unwrap_err();

    let expected_error = AlphabetDefinitionError::DuplicatedCharacterEncountered {
        duplicates: "bd".to_owned(),
        alphabet_definition: definition.to_owned(),
    };

    assert_eq!(expected_error, error);
}

#[test]
fn parsing_error_is_correctly_displayed() {
    let definition = "abbcdde";

    let error = str::parse::<Alphabet>(definition).unwrap_err();

    let expected_error = "the same character cannot be included in the alphabet multiple times \
        (original definition: \"abbcdde\"), duplicate characters: \"bd\"";

    assert_eq!(expected_error, &format!("{error}"));
}

#[test]
fn alphabet_can_be_created_from_a_string_if_it_has_no_duplicate_characters() {
    let definition = "abc";

    let parsing_result = Alphabet::try_from(definition);

    assert!(parsing_result.is_ok());
}

#[test]
fn alphabet_cannot_be_created_from_a_string_if_it_has_duplicate_characters() {
    let definition = "abbcdde";

    let error = Alphabet::try_from(definition).unwrap_err();

    let expected_error = AlphabetDefinitionError::DuplicatedCharacterEncountered {
        duplicates: "bd".to_owned(),
        alphabet_definition: definition.to_owned(),
    };

    assert_eq!(expected_error, error);
}

#[test]
fn creation_from_string_error_is_correctly_displayed() {
    let definition = "abbcdde";

    let error = Alphabet::try_from(definition).unwrap_err();

    let expected_error = "the same character cannot be included in the alphabet multiple times \
        (original definition: \"abbcdde\"), duplicate characters: \"bd\"";

    assert_eq!(expected_error, &format!("{error}"));
}

#[test]
fn alphabet_contains_the_characters_as_defined() {
    let definition = "abc";

    let alphabet = Alphabet::from_str(definition).unwrap();

    assert!(alphabet.contains('a'));
    assert!(alphabet.contains('b'));
    assert!(alphabet.contains('c'));
}

#[test]
fn alphabet_does_not_contain_characters_missing_from_its_definition() {
    let definition = "abc";

    let alphabet = Alphabet::from_str(definition).unwrap();

    for character in ('d'..='z')
        .into_iter()
        .chain(('A'..='Z').into_iter())
        .chain(('0'..='9').into_iter())
        .chain(('а'..='я').into_iter())
        .chain(('ぁ'..='ゖ').into_iter())
        .chain(('ؠ'..='ۓ').into_iter())
    {
        assert!(!alphabet.contains(character))
    }
}

#[test]
fn alphabet_can_be_extended() {
    let definition = "abc";

    let alphabet = Alphabet::from_str(definition).unwrap();

    let extension_result = alphabet.extend('d');

    assert!(extension_result.is_ok());
}

#[test]
fn alphabet_cannot_be_extended_with_characters_that_belong_to_the_alphabet() {
    let definition = "abc";

    let alphabet = Alphabet::from_str(definition).unwrap();

    let error = alphabet.extend('c').unwrap_err();

    let expected_error = AlphabetDefinitionError::ExtendedWithADuplicate;

    assert_eq!(expected_error, error);
}

#[test]
fn alphabet_cannot_be_extended_with_characters_that_belong_to_the_extended_alphabet() {
    let definition = "abc";

    let alphabet = Alphabet::from_str(definition).unwrap().extend('d').unwrap();

    let error = alphabet.extend('d').unwrap_err();

    let expected_error = AlphabetDefinitionError::ExtendedWithADuplicate;

    assert_eq!(expected_error, error);
}

#[test]
fn extension_error_is_correctly_displayed() {
    let definition = "abc";

    let alphabet = Alphabet::from_str(definition).unwrap();

    let error = alphabet.extend('c').unwrap_err();

    let expected_error = "an alphabet cannot be extended with duplicate characters";

    assert_eq!(expected_error, &format!("{error}"));
}

#[test]
fn extended_alphabet_contains_the_originally_defined_characters_and_the_added_ones() {
    let definition = "abc";

    let alphabet = Alphabet::from_str(definition).unwrap();

    let alphabet = alphabet.extend('ф').unwrap();

    assert!(alphabet.contains('a'));
    assert!(alphabet.contains('b'));
    assert!(alphabet.contains('c'));
    assert!(alphabet.contains_extended('ф'));
}

#[test]
fn extended_alphabet_does_not_contain_characters_missing_from_its_definition() {
    let definition = "abc";

    let alphabet = Alphabet::from_str(definition).unwrap().extend('d').unwrap();

    for character in ('d'..='z')
        .into_iter()
        .chain(('A'..='Z').into_iter())
        .chain(('0'..='9').into_iter())
        .chain(('а'..='я').into_iter())
        .chain(('ぁ'..='ゖ').into_iter())
        .chain(('ؠ'..='ۓ').into_iter())
    {
        assert!(!alphabet.contains(character))
    }
}

#[test]
fn extended_alphabet_does_not_contain_characters_missing_from_either_its_definition_or_extension() {
    let definition = "abc";

    let alphabet = Alphabet::from_str(definition).unwrap().extend('d').unwrap();

    for character in ('e'..='z')
        .into_iter()
        .chain(('A'..='Z').into_iter())
        .chain(('0'..='9').into_iter())
        .chain(('а'..='я').into_iter())
        .chain(('ぁ'..='ゖ').into_iter())
        .chain(('ؠ'..='ۓ').into_iter())
    {
        assert!(!alphabet.contains_extended(character))
    }
}

#[test]
fn alphabet_can_be_cloned() {
    let set: HashSet<_> = ('a'..='z').into_iter().collect();

    let alphabet = Alphabet::try_from(&set).unwrap();

    #[allow(clippy::redundant_clone)]
    let _clone = alphabet.clone();
}
