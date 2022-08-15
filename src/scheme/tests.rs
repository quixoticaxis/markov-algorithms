use super::*;

#[test]
fn scheme_can_be_created_if_all_characters_belong_to_the_alphabet_and_formulas_are_well_formed() {
    let scheme_definition = "ab→⋅|cd\ndf→ab".to_owned();

    let configuration = Default::default();

    let creation_result = AlgorithmScheme::new(&configuration, &scheme_definition);

    assert!(creation_result.is_ok());
}

#[test]
fn empty_scheme_is_valid() {
    let scheme_definition = "".to_owned();

    let configuration = Default::default();

    let creation_result = AlgorithmScheme::new(&configuration, &scheme_definition);

    assert!(creation_result.is_ok());
}

#[test]
fn scheme_cannot_contain_empty_lines() {
    let scheme_definition = "\n".to_owned();

    let configuration = Default::default();

    let creation_result = AlgorithmScheme::new(&configuration, &scheme_definition);

    assert_eq!(
        AlgorithmSchemeCreationError::EncounteredEmptyLine,
        creation_result.unwrap_err()
    );
}

#[test]
fn scheme_reports_formula_creation_errors() {
    let scheme_definition = "ab|cd".to_owned();

    let configuration = Default::default();

    let creation_result = AlgorithmScheme::new(&configuration, &scheme_definition);

    assert!(matches!(
        creation_result.unwrap_err(),
        AlgorithmSchemeCreationError::FormulaCreationFailed { source: _ }
    ));
}

#[test]
fn empty_scheme_can_be_applied() {
    let scheme_definition = "".to_owned();

    let configuration = Default::default();

    let scheme = AlgorithmScheme::new(&configuration, &scheme_definition).unwrap();

    let application_result = scheme.apply(Default::default(), 1);

    assert_eq!(
        ApplicationResult::new(1, "".to_owned()),
        application_result.unwrap()
    );
}

#[test]
fn application_can_complete_due_to_reaching_termination_rule() {
    let scheme_definition = concat!("c→b\n", "b→a\n", "a→⋅a").to_owned();

    let configuration = Default::default();

    let scheme = AlgorithmScheme::new(&configuration, &scheme_definition).unwrap();

    let string = "c";

    let application_result = scheme.apply(string, 1_000);

    assert_eq!(
        ApplicationResult::new(3, "a".to_owned()),
        application_result.unwrap()
    );
}

#[test]
fn application_can_complete_due_to_no_rules_applicable() {
    let scheme_definition = concat!("c→c\n", "b→b\n", "a→a").to_owned();

    let configuration = Default::default();

    let scheme = AlgorithmScheme::new(&configuration, &scheme_definition).unwrap();

    let string = "dfdfdf".to_owned();

    let application_result = scheme.apply(&string, 1);

    assert_eq!(
        ApplicationResult::new(1, string),
        application_result.unwrap()
    );
}

#[test]
fn application_can_fail_on_strings_that_contain_characters_not_belonging_to_the_alphabet() {
    let scheme_definition = concat!("c→c\n", "b→b\n", "a→a").to_owned();

    let configuration = Default::default();

    let scheme = AlgorithmScheme::new(&configuration, &scheme_definition).unwrap();

    let string = "ф".to_owned();

    let application_result = scheme.apply(&string, 1_000);

    assert_eq!(
        AlgorithmSchemeApplicationError::UnknownCharacterEncountered('ф'),
        application_result.unwrap_err()
    );
}

#[test]
fn application_can_fail_due_to_reaching_step_limit() {
    let scheme_definition = concat!("c→c\n", "b→b\n", "a→a").to_owned();

    let limit = 10;

    let configuration = Default::default();

    let scheme = AlgorithmScheme::new(&configuration, &scheme_definition).unwrap();

    let string = "b".to_owned();

    let application_result = scheme.apply(&string, limit);

    assert_eq!(
        AlgorithmSchemeApplicationError::HitTheStepsLimit(limit),
        application_result.unwrap_err()
    );
}
