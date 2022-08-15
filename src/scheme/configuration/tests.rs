use super::*;

#[test]
fn formula_configuration_can_be_created_if_delimiter_and_marker_do_not_belong_to_alphabet() {
    let creation_result = SubstitutionFormulaConfiguration::new('#', '+');

    assert!(creation_result.is_ok());
}

#[test]
fn formula_configuration_cannot_be_created_if_delimiter_belongs_to_alphabet() {
    let creation_result = SubstitutionFormulaConfiguration::new('a', '+');

    assert_eq!(
        SubstitutionFormulaConfigurationCreationError::DelimiterIsPartOfTheAlphabet,
        creation_result.unwrap_err()
    );
}

#[test]
fn formula_configuration_cannot_be_created_if_final_marker_belongs_to_alphabet() {
    let creation_result = SubstitutionFormulaConfiguration::new('#', 'a');

    assert_eq!(
        SubstitutionFormulaConfigurationCreationError::FinalMarkerIsPartOfTheAlphabet,
        creation_result.unwrap_err()
    );
}

#[test]
fn formula_configuration_can_be_created_over_custom_extended_alphabet() {
    let creation_result = SubstitutionFormulaConfiguration::over_alphabet(
        '#',
        '.',
        ('1'..='9').into_iter().collect(),
    );

    assert!(creation_result.is_ok());
}
