mod errors;

#[cfg(test)]
mod tests;

use std::collections::HashSet;

use self::errors::SubstitutionFormulaConfigurationCreationError;

/// A configuration to create substitution formulas from textual definitions.
#[derive(Clone, Debug)]
pub struct SubstitutionFormulaConfiguration {
    final_marker: char,
    delimiter: char,
    final_delimiter: String,
    extended_alphabet: HashSet<char>,
}

impl SubstitutionFormulaConfiguration {
    /// Creates a new configuraion. Assumes latin letters, digits, and '|' to be used.
    ///
    /// # Arguments
    ///
    /// - `delimiter` — a delimiter to split the substitution formulas.
    /// - `final_marker` — a marker to detect final substitution formulas.
    pub fn new(
        delimiter: char,
        final_marker: char,
    ) -> Result<Self, SubstitutionFormulaConfigurationCreationError> {
        Self::over_alphabet(delimiter, final_marker, Self::default().extended_alphabet)
    }

    /// Creates a new configuraion.
    /// # Arguments
    ///
    /// - `delimiter` — a delimiter to split the substitution formulas.
    /// - `final_marker` — a marker to detect final substitution formulas.
    /// - `extended_alphabet` — an alphabet that is assumed to be used both in the scheme definition and in the input strings.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use std::collections::HashSet;
    /// # use markovalgorithms::*;
    ///  
    /// let default_alphabet: HashSet<_> = ['.', ',', ' ']
    ///     .into_iter()
    ///     .chain(('a'..='z').into_iter())
    ///     .chain(('A'..='Z').into_iter())
    ///     .chain(('0'..='9').into_iter())
    ///     .chain(['|'].into_iter())
    ///     .collect();
    ///
    /// let configuration = SubstitutionFormulaConfiguration::over_alphabet('→', '⋅', default_alphabet)
    ///     .expect("The default alphabet does not contain default delimiters.");
    ///
    /// assert_eq!('→', configuration.delimiter());
    /// assert_eq!('⋅', configuration.final_marker());
    /// assert_eq!("→⋅", configuration.final_delimiter());
    /// ```
    pub fn over_alphabet(
        delimiter: char,
        final_marker: char,
        extended_alphabet: HashSet<char>,
    ) -> Result<Self, SubstitutionFormulaConfigurationCreationError> {
        if extended_alphabet.contains(&delimiter) {
            return Err(
                SubstitutionFormulaConfigurationCreationError::DelimiterIsPartOfTheAlphabet,
            );
        }
        if extended_alphabet.contains(&final_marker) {
            return Err(
                SubstitutionFormulaConfigurationCreationError::FinalMarkerIsPartOfTheAlphabet,
            );
        }

        let mut final_delimiter = String::new();
        final_delimiter.push(delimiter);
        final_delimiter.push(final_marker);

        Ok(Self {
            delimiter,
            final_marker,
            final_delimiter,
            extended_alphabet,
        })
    }

    /// A delimiter to split the substitution formulas.
    pub fn delimiter(&self) -> char {
        self.delimiter
    }

    /// A marker to detect final substitution formulas.
    pub fn final_marker(&self) -> char {
        self.final_marker
    }

    /// The delimiter that slits final formulas, can be composed of delimiter followed by final marker.
    pub fn final_delimiter(&self) -> &str {
        &self.final_delimiter
    }

    /// An alphabet that is assumed to be used both in the scheme definition and in the input strings.
    pub fn alphabet(&self) -> &HashSet<char> {
        &self.extended_alphabet
    }
}

impl Default for SubstitutionFormulaConfiguration {
    fn default() -> Self {
        let default_alphabet: HashSet<_> = ['.', ',', ' ']
            .into_iter()
            .chain(('a'..='z').into_iter())
            .chain(('A'..='Z').into_iter())
            .chain(('0'..='9').into_iter())
            .chain(['|'].into_iter())
            .collect();
        Self::over_alphabet('→', '⋅', default_alphabet)
            .expect("The default alphabet does not contain default delimiters.")
    }
}
