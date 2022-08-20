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

pub mod alphabet;
pub mod scheme;

pub mod prelude {
    //! Re-exported types to simplify the usage of the library.

    pub use crate::alphabet::{Alphabet, AlphabetDefinitionError};

    pub use crate::scheme::{
        scheme_builder::{AlgorithmSchemeBuilder, AlgorithmSchemeDefinitionError},
        AlgorithmScheme, AlgorithmSchemeFullApplicationError, AlgorithmSchemeInputValidationError,
        ApplicationIterator, FullApplicationResult, SingleApplicationData, SingleApplicationResult,
        SubstitutionFormulaDefinitionError,
    };
}
