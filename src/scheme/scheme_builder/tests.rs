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

use super::*;

#[test]
fn scheme_builder_can_be_created() {
    let _builder = AlgorithmSchemeBuilder::new();
}

#[test]
fn default_scheme_builder_can_be_created() {
    let _builder = AlgorithmSchemeBuilder::default();
}

#[test]
fn a_delimiter_can_be_added_to_a_scheme_builder() {
    let builder = AlgorithmSchemeBuilder::new();

    let _builder = builder.with_delimiter('→');
}

#[test]
fn a_final_marker_can_be_added_to_a_scheme_builder() {
    let builder = AlgorithmSchemeBuilder::new();

    let _builder = builder.with_final_marker('⋅');
}

#[test]
fn an_alphabet_can_be_added_to_a_scheme_builder() {
    let builder = AlgorithmSchemeBuilder::new();

    let _builder = builder.with_alphabet("str".try_into().unwrap());
}

#[test]
fn the_scheme_can_be_built_with_well_formed_formula_definitions() {
    let builder = AlgorithmSchemeBuilder::new()
        .with_alphabet("ab".try_into().unwrap())
        .with_delimiter('→')
        .with_final_marker('⋅');

    let building_result = builder.build_with_formula_definitions(["a→b"].into_iter());

    assert!(building_result.is_ok())
}

#[test]
fn the_scheme_cannot_be_built_if_delimiter_and_final_marker_are_the_same() {
    let builder = AlgorithmSchemeBuilder::new()
        .with_alphabet("ab".try_into().unwrap())
        .with_delimiter('→')
        .with_final_marker('→');

    let error = builder
        .build_with_formula_definitions(["a→b"].into_iter())
        .unwrap_err();

    let expected_error = AlgorithmSchemeDefinitionError::DelimiterAndFinalMarkerAreTheSame('→');

    assert_eq!(expected_error, error);
}

#[test]
fn an_error_is_reported_if_delimiter_and_final_marker_are_the_same() {
    let builder = AlgorithmSchemeBuilder::new()
        .with_alphabet("ab".try_into().unwrap())
        .with_delimiter('→')
        .with_final_marker('→');

    let error = builder
        .build_with_formula_definitions(["a→b"].into_iter())
        .unwrap_err();

    assert_eq!(
        "the same character '→' cannot be used as a delimiter and as a final marker",
        format!("{error}")
    );
}

#[test]
fn the_scheme_cannot_be_built_if_the_delimiter_belongs_to_the_alphabet() {
    let builder = AlgorithmSchemeBuilder::new()
        .with_alphabet("ab→".try_into().unwrap())
        .with_delimiter('→')
        .with_final_marker('⋅');

    let error = builder
        .build_with_formula_definitions(["a→b"].into_iter())
        .unwrap_err();

    let expected_error = AlgorithmSchemeDefinitionError::DelimiterBelongsToTheAlphabet('→');

    assert_eq!(expected_error, error);
}

#[test]
fn an_error_is_reported_if_the_delimiter_belongs_to_the_alphabet() {
    let builder = AlgorithmSchemeBuilder::new()
        .with_alphabet("ab→".try_into().unwrap())
        .with_delimiter('→')
        .with_final_marker('⋅');

    let error = builder
        .build_with_formula_definitions(["a→b"].into_iter())
        .unwrap_err();

    assert_eq!(
        "the character '→' cannot be used as a delimiter because it belongs to the alphabet",
        format!("{error}")
    );
}

#[test]
fn the_scheme_cannot_be_built_if_the_final_marker_belongs_to_the_alphabet() {
    let builder = AlgorithmSchemeBuilder::new()
        .with_alphabet("ab⋅".try_into().unwrap())
        .with_delimiter('→')
        .with_final_marker('⋅');

    let error = builder
        .build_with_formula_definitions(["a→b"].into_iter())
        .unwrap_err();

    let expected_error = AlgorithmSchemeDefinitionError::FinalMarkerBelongsToTheAlphabet('⋅');

    assert_eq!(expected_error, error);
}

#[test]
fn an_error_is_reported_if_the_final_marker_belongs_to_the_alphabet() {
    let builder = AlgorithmSchemeBuilder::new()
        .with_alphabet("ab⋅".try_into().unwrap())
        .with_delimiter('→')
        .with_final_marker('⋅');

    let error = builder
        .build_with_formula_definitions(["a→b"].into_iter())
        .unwrap_err();

    assert_eq!(
        "the character '⋅' cannot be used as a final marker because it belongs to the alphabet",
        format!("{error}")
    );
}

#[test]
fn the_scheme_cannot_be_built_if_there_are_unknown_characters_in_the_definition() {
    let builder = AlgorithmSchemeBuilder::new()
        .with_alphabet(Alphabet::try_from("a").unwrap().extend('b').unwrap())
        .with_delimiter('→')
        .with_final_marker('⋅');

    let error = builder
        .build_with_formula_definitions(["aб→фb"].into_iter())
        .unwrap_err();

    let expected_error =
        AlgorithmSchemeDefinitionError::UnknownCharactersEncountered("бф".to_owned());

    assert_eq!(expected_error, error);
}

#[test]
fn an_error_is_reported_if_there_are_unknown_characters_in_the_definition() {
    let builder = AlgorithmSchemeBuilder::new()
        .with_alphabet("ab".try_into().unwrap())
        .with_delimiter('→')
        .with_final_marker('⋅');

    let error = builder
        .build_with_formula_definitions(["ну→ぬ"].into_iter())
        .unwrap_err();

    assert_eq!(
        "the definition of the scheme contains the characters that neither belong to the alphabet, \
    nor are delimiter or final marker (unknown characters: \"нуぬ\")",
        format!("{error}")
    );
}

#[test]
fn the_scheme_cannot_be_built_if_the_formula_definitions_are_not_well_formed() {
    let builder = AlgorithmSchemeBuilder::new()
        .with_alphabet("ab".try_into().unwrap())
        .with_delimiter('→')
        .with_final_marker('⋅');

    let error = builder
        .build_with_formula_definitions(["a→→b"].into_iter())
        .unwrap_err();

    assert!(matches!(
        error,
        AlgorithmSchemeDefinitionError::FormulaCreationError { source: _ }
    ));
}

#[test]
fn an_error_is_reported_if_the_formula_definitions_are_not_well_formed() {
    let builder = AlgorithmSchemeBuilder::new()
        .with_alphabet("ab".try_into().unwrap())
        .with_delimiter('→')
        .with_final_marker('⋅');

    let error = builder
        .build_with_formula_definitions(["a→→b"].into_iter())
        .unwrap_err();

    assert!(format!("{error}")
        .starts_with("encountered an issue during the creation of substitution formulas: "));
}

#[test]
fn the_scheme_builder_can_be_cloned() {
    let builder = AlgorithmSchemeBuilder::new();

    #[allow(clippy::redundant_clone)]
    let _clone = builder.clone();
}
