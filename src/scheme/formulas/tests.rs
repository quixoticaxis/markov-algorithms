use super::*;

#[test]
fn formula_can_be_created_when_all_characters_are_in_alphabet_and_there_is_a_single_delimiter() {
    let configuration = SubstitutionFormulaConfiguration::default();

    let string = "ab→⋅|cd".to_owned();
    let range = 0..string.len();

    let creation_result = SubstitutionFormula::new(&configuration, &string, range);

    assert!(creation_result.is_ok());
}

#[test]
fn formula_without_final_marker_is_considered_simple_substitution() {
    let configuration = SubstitutionFormulaConfiguration::default();

    let string = "ab→|cd".to_owned();
    let range = 0..string.len();

    let creation_result = SubstitutionFormula::new(&configuration, &string, range);

    assert!(matches!(
        creation_result,
        Ok(SubstitutionFormula::Simple(_))
    ));
}

#[test]
fn formula_with_final_marker_is_considered_final_substitution() {
    let configuration = SubstitutionFormulaConfiguration::default();

    let string = "ab→⋅|cd".to_owned();
    let range = 0..string.len();

    let creation_result = SubstitutionFormula::new(&configuration, &string, range);

    assert!(matches!(creation_result, Ok(SubstitutionFormula::Final(_))));
}

#[test]
fn formula_cannot_be_created_with_multiple_delimiters() {
    let configuration = SubstitutionFormulaConfiguration::default();

    let string = "ab→⋅|cd→c".to_owned();
    let range = 0..string.len();

    let creation_result = SubstitutionFormula::new(&configuration, &string, range);

    assert_eq!(
        SubstitutionFormulaCreationError::MultipleDelimitersFound(string, 2),
        creation_result.unwrap_err()
    );
}

#[test]
fn formula_cannot_be_created_with_no_delimiter() {
    let configuration = SubstitutionFormulaConfiguration::default();

    let string = "ab|cd".to_owned();
    let range = 0..string.len();

    let creation_result = SubstitutionFormula::new(&configuration, &string, range);

    assert_eq!(
        SubstitutionFormulaCreationError::NoDelimiterFound(string),
        creation_result.unwrap_err()
    );
}

#[test]
fn formula_cannot_be_created_with_unknown_characters() {
    let configuration = SubstitutionFormulaConfiguration::default();

    let string = "ab→⋅ф".to_owned();
    let range = 0..string.len();

    let creation_result = SubstitutionFormula::new(&configuration, &string, range);

    assert_eq!(
        SubstitutionFormulaCreationError::UnknownCharacterEncountered(string, 'ф'),
        creation_result.unwrap_err()
    );
}

#[test]
fn formula_cannot_be_created_with_final_marker_on_the_left_side() {
    let configuration = SubstitutionFormulaConfiguration::default();

    let string = "a⋅b→⋅a".to_owned();
    let range = 0..string.len();

    let creation_result = SubstitutionFormula::new(&configuration, &string, range);

    assert_eq!(
        SubstitutionFormulaCreationError::FinalMarkerOnTheLeft(string),
        creation_result.unwrap_err()
    );
}

#[test]
fn formula_cannot_be_created_with_additional_final_marker_the_right_side() {
    let configuration = SubstitutionFormulaConfiguration::default();

    let string = "ab→⋅⋅a".to_owned();
    let range = 0..string.len();

    let creation_result = SubstitutionFormula::new(&configuration, &string, range);

    assert_eq!(
        SubstitutionFormulaCreationError::FinalMarkerOnTheRight(string),
        creation_result.unwrap_err()
    );
}

#[test]
fn formula_cannot_be_created_with_final_marker_out_of_place() {
    let configuration = SubstitutionFormulaConfiguration::default();

    let string = "ab→⋅a⋅".to_owned();
    let range = 0..string.len();

    let creation_result = SubstitutionFormula::new(&configuration, &string, range);

    assert_eq!(
        SubstitutionFormulaCreationError::FinalMarkerOnTheRight(string),
        creation_result.unwrap_err()
    );
}

#[test]
fn formula_is_applied_if_left_side_has_matches_in_a_string() {
    let configuration = SubstitutionFormulaConfiguration::default();

    let formula_definition = "ab→a".to_owned();
    let range = 0..formula_definition.len();

    let formula = SubstitutionFormula::new(&configuration, &formula_definition, range).unwrap();

    let string = "abab";

    let application_result = formula.apply(&formula_definition, string);

    assert_eq!(
        SubstitutionResult::Applied("aab".to_owned()),
        application_result
    );
}

#[test]
fn final_formula_is_applied_as_final_substitution_if_left_side_has_matches_in_a_string() {
    let configuration = SubstitutionFormulaConfiguration::default();

    let formula_definition = "ab→⋅a".to_owned();
    let range = 0..formula_definition.len();

    let formula = SubstitutionFormula::new(&configuration, &formula_definition, range).unwrap();

    let string = "abab";

    let application_result = formula.apply(&formula_definition, string);

    assert_eq!(
        SubstitutionResult::Halt("aab".to_owned()),
        application_result
    );
}

#[test]
fn formula_is_not_applied_if_left_side_has_no_matches_in_a_string() {
    let configuration = SubstitutionFormulaConfiguration::default();

    let formula_definition = "ab→a".to_owned();
    let range = 0..formula_definition.len();

    let formula = SubstitutionFormula::new(&configuration, &formula_definition, range).unwrap();

    let string = "cdcd";

    let application_result = formula.apply(&formula_definition, string);

    assert_eq!(
        SubstitutionResult::NotApplied(string.to_owned()),
        application_result
    );
}
