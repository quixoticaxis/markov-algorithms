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

use std::str::FromStr;

use crate::prelude::{AlgorithmSchemeBuilder, Alphabet};

use super::*;

#[test]
fn a_scheme_can_be_applied_if_the_input_string_contains_only_characters_that_belong_to_the_alphabet(
) {
    let alphabet = Alphabet::from_str("abc").unwrap().extend('d').unwrap();

    let scheme = AlgorithmSchemeBuilder::new()
        .with_alphabet(alphabet)
        .build_with_formula_definitions(["a→⋅d"].into_iter())
        .unwrap();

    let result = scheme.apply("abc", 1).unwrap();

    let expected = FullApplicationResult {
        word: "dbc".to_owned(),
        steps_done: 1,
    };

    assert_eq!(expected, result);
}

#[test]
fn a_scheme_cannot_be_applied_if_the_input_string_contains_extension_characters() {
    let alphabet = Alphabet::from_str("abc")
        .unwrap()
        .extend('d')
        .unwrap()
        .extend('e')
        .unwrap();

    let scheme = AlgorithmSchemeBuilder::new()
        .with_alphabet(alphabet)
        .build_with_formula_definitions(["a→⋅d"].into_iter())
        .unwrap();

    let error = scheme.apply("abcde", 1).unwrap_err();

    let extpected_error = AlgorithmSchemeFullApplicationError::InputValidationError {
        source: AlgorithmSchemeInputValidationError::ExtensionCharactersEncountered(
            "de".to_owned(),
        ),
    };

    assert_eq!(extpected_error, error);
}

#[test]
fn an_error_is_reported_if_the_input_string_contains_extension_characters() {
    let alphabet = Alphabet::from_str("abc")
        .unwrap()
        .extend('d')
        .unwrap()
        .extend('e')
        .unwrap();

    let scheme = AlgorithmSchemeBuilder::new()
        .with_alphabet(alphabet)
        .build_with_formula_definitions(["a→⋅d"].into_iter())
        .unwrap();

    let error = scheme.apply("abcde", 1).unwrap_err();

    assert_eq!(
        "the input string is not valid: extension characters are found in the input word (extension characters: \"de\")",
        format!("{error}")
    );
}

#[test]
fn a_scheme_cannot_be_applied_if_the_input_string_contains_unknown_characters() {
    let alphabet = Alphabet::from_str("abc").unwrap().extend('d').unwrap();

    let scheme = AlgorithmSchemeBuilder::new()
        .with_alphabet(alphabet)
        .build_with_formula_definitions(["a→⋅d"].into_iter())
        .unwrap();

    let error = scheme.apply("abcef", 1).unwrap_err();

    let extpected_error = AlgorithmSchemeFullApplicationError::InputValidationError {
        source: AlgorithmSchemeInputValidationError::UnknownCharactersEncountered("ef".to_owned()),
    };

    assert_eq!(extpected_error, error);
}

#[test]
fn an_error_is_reported_if_the_input_string_contains_unknown_characters() {
    let alphabet = Alphabet::from_str("abc").unwrap().extend('d').unwrap();

    let scheme = AlgorithmSchemeBuilder::new()
        .with_alphabet(alphabet)
        .build_with_formula_definitions(["a→⋅d"].into_iter())
        .unwrap();

    let error = scheme.apply("abcef", 1).unwrap_err();

    assert_eq!(
        "the input string is not valid: unsupported characters are found in the input word (unsupported characters: \"ef\")",
        format!("{error}")
    );
}

#[test]
fn a_scheme_cannot_be_fully_applied_if_the_steps_limit_is_zero() {
    let alphabet = Alphabet::from_str("abc").unwrap().extend('d').unwrap();

    let scheme = AlgorithmSchemeBuilder::new()
        .with_alphabet(alphabet)
        .build_with_formula_definitions(["b→b"].into_iter())
        .unwrap();

    let error = scheme.apply("abc", 0).unwrap_err();

    let extpected_error = AlgorithmSchemeFullApplicationError::ZeroStepsLimit;

    assert_eq!(extpected_error, error);
}

#[test]
fn an_error_is_reported_if_the_steps_limit_is_zero() {
    let alphabet = Alphabet::from_str("abc").unwrap().extend('d').unwrap();

    let scheme = AlgorithmSchemeBuilder::new()
        .with_alphabet(alphabet)
        .build_with_formula_definitions(["b→a"].into_iter())
        .unwrap();

    let error = scheme.apply("abc", 0).unwrap_err();

    assert_eq!(
        "the algorithm should be allowed to do at least one step",
        format!("{error}")
    );
}

#[test]
fn a_scheme_cannot_be_fully_applied_if_the_algorithm_does_not_complete_in_a_set_number_of_steps() {
    let alphabet = Alphabet::from_str("abc").unwrap().extend('d').unwrap();

    let scheme = AlgorithmSchemeBuilder::new()
        .with_alphabet(alphabet)
        .build_with_formula_definitions(["b→b"].into_iter())
        .unwrap();

    let error = scheme.apply("abc", 1).unwrap_err();

    let extpected_error = AlgorithmSchemeFullApplicationError::HitTheStepsLimit(1);

    assert_eq!(extpected_error, error);
}

#[test]
fn an_error_is_reported_if_the_algorithm_does_not_complete_in_a_set_number_of_steps() {
    let alphabet = Alphabet::from_str("abc").unwrap().extend('d').unwrap();

    let scheme = AlgorithmSchemeBuilder::new()
        .with_alphabet(alphabet)
        .build_with_formula_definitions(["b→b"].into_iter())
        .unwrap();

    let error = scheme.apply("abc", 1).unwrap_err();

    assert_eq!(
        "the application is not completed after reaching step 1",
        format!("{error}")
    );
}

#[test]
fn a_scheme_can_be_applied_at_least_once_if_the_input_string_contains_only_characters_that_belong_to_the_alphabet(
) {
    let alphabet = Alphabet::from_str("abc").unwrap().extend('d').unwrap();

    let scheme = AlgorithmSchemeBuilder::new()
        .with_alphabet(alphabet)
        .build_with_formula_definitions(["a→⋅d"].into_iter())
        .unwrap();

    let result = scheme.apply_once("abc");

    assert!(result.is_ok());
}

#[test]
fn a_scheme_application_may_yield_the_final_result() {
    let alphabet = Alphabet::from_str("abc").unwrap().extend('d').unwrap();

    let scheme = AlgorithmSchemeBuilder::new()
        .with_alphabet(alphabet)
        .build_with_formula_definitions(["a→⋅d"].into_iter())
        .unwrap();

    let result = scheme.apply_once("abc").unwrap();

    let expected = SingleApplicationResult::Final(SingleApplicationData {
        word: "dbc".to_owned(),
        applied_formula_definition: Some("a→⋅d"),
    });

    assert_eq!(expected, result);
}

#[test]
fn a_scheme_application_yields_the_final_with_no_formula_if_no_substitution_formula_is_applied() {
    let alphabet = Alphabet::from_str("abc").unwrap().extend('d').unwrap();

    let scheme = AlgorithmSchemeBuilder::new()
        .with_alphabet(alphabet)
        .build_with_formula_definitions(["a→⋅d"].into_iter())
        .unwrap();

    let result = scheme.apply_once("bbb").unwrap();

    let expected = SingleApplicationResult::Final(SingleApplicationData {
        word: "bbb".to_owned(),
        applied_formula_definition: None,
    });

    assert_eq!(expected, result);
}

#[test]
fn a_scheme_application_may_yield_the_intermediate_result() {
    let alphabet = Alphabet::from_str("abc").unwrap().extend('d').unwrap();

    let scheme = AlgorithmSchemeBuilder::new()
        .with_alphabet(alphabet)
        .build_with_formula_definitions(["a→d"].into_iter())
        .unwrap();

    let result = scheme.apply_once("abc").unwrap();

    let expected = SingleApplicationResult::Intermediate(SingleApplicationData {
        word: "dbc".to_owned(),
        applied_formula_definition: Some("a→d"),
    });

    assert_eq!(expected, result);
}

#[test]
fn a_scheme_cannot_be_applied_even_once_if_the_input_string_contains_extension_characters() {
    let alphabet = Alphabet::from_str("abc")
        .unwrap()
        .extend('d')
        .unwrap()
        .extend('e')
        .unwrap();

    let scheme = AlgorithmSchemeBuilder::new()
        .with_alphabet(alphabet)
        .build_with_formula_definitions(["a→⋅d"].into_iter())
        .unwrap();

    let error = scheme.apply_once("abcde").unwrap_err();

    let extpected_error =
        AlgorithmSchemeInputValidationError::ExtensionCharactersEncountered("de".to_owned());

    assert_eq!(extpected_error, error);
}

#[test]
fn a_scheme_cannot_be_applied_even_once_if_the_input_string_contains_unknown_characters() {
    let alphabet = Alphabet::from_str("abc").unwrap().extend('d').unwrap();

    let scheme = AlgorithmSchemeBuilder::new()
        .with_alphabet(alphabet)
        .build_with_formula_definitions(["a→⋅d"].into_iter())
        .unwrap();

    let error = scheme.apply_once("abcef").unwrap_err();

    let extpected_error =
        AlgorithmSchemeInputValidationError::UnknownCharactersEncountered("ef".to_owned());

    assert_eq!(extpected_error, error);
}

#[test]
fn scheme_may_yield_an_iterator_if_the_input_string_contains_only_characters_that_belong_to_the_alphabet(
) {
    let alphabet = Alphabet::from_str("abc").unwrap().extend('d').unwrap();

    let scheme = AlgorithmSchemeBuilder::new()
        .with_alphabet(alphabet)
        .build_with_formula_definitions(["a→⋅d"].into_iter())
        .unwrap();

    let result = scheme.get_application_iterator("abc");

    assert!(result.is_ok());
}

#[test]
fn a_scheme_cannot_yield_an_iterator_if_the_input_string_contains_extension_characters() {
    let alphabet = Alphabet::from_str("abc")
        .unwrap()
        .extend('d')
        .unwrap()
        .extend('e')
        .unwrap();

    let scheme = AlgorithmSchemeBuilder::new()
        .with_alphabet(alphabet)
        .build_with_formula_definitions(["a→⋅d"].into_iter())
        .unwrap();

    let error = scheme.get_application_iterator("abcde").unwrap_err();

    let extpected_error =
        AlgorithmSchemeInputValidationError::ExtensionCharactersEncountered("de".to_owned());

    assert_eq!(extpected_error, error);
}

#[test]
fn a_scheme_cannot_yield_an_iterator_if_the_input_string_contains_unknown_characters() {
    let alphabet = Alphabet::from_str("abc").unwrap().extend('d').unwrap();

    let scheme = AlgorithmSchemeBuilder::new()
        .with_alphabet(alphabet)
        .build_with_formula_definitions(["a→⋅d"].into_iter())
        .unwrap();

    let error = scheme.get_application_iterator("abcef").unwrap_err();

    let extpected_error =
        AlgorithmSchemeInputValidationError::UnknownCharactersEncountered("ef".to_owned());

    assert_eq!(extpected_error, error);
}

#[test]
fn a_scheme_appication_may_be_viewed_through_iterator_step_by_step() {
    let alphabet = Alphabet::from_str("abc").unwrap().extend('d').unwrap();

    let scheme = AlgorithmSchemeBuilder::new()
        .with_alphabet(alphabet)
        .build_with_formula_definitions(["a→b", "b→c", "ccc→⋅d"].into_iter())
        .unwrap();

    let mut iterator = scheme.get_application_iterator("abc").unwrap();

    assert_eq!(
        Some(SingleApplicationData {
            word: "bbc".to_owned(),
            applied_formula_definition: Some("a→b")
        }),
        iterator.next()
    );

    assert_eq!(
        Some(SingleApplicationData {
            word: "cbc".to_owned(),
            applied_formula_definition: Some("b→c")
        }),
        iterator.next()
    );

    assert_eq!(
        Some(SingleApplicationData {
            word: "ccc".to_owned(),
            applied_formula_definition: Some("b→c")
        }),
        iterator.next()
    );

    assert_eq!(
        Some(SingleApplicationData {
            word: "d".to_owned(),
            applied_formula_definition: Some("ccc→⋅d")
        }),
        iterator.next()
    );

    assert_eq!(None, iterator.next());
}
