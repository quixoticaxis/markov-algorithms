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

use std::{collections::HashSet, iter};

use rand::seq::SliceRandom;

use markovalgorithms::prelude::*;

/// A scheme of the algorithm that, when applied to a string in alphabet `[ 'a', 'b', 'c', 'A', 'B', 'C', '"' ]`,
/// capitalizes the first letter and capitalizes each quoted segment.
///
/// The scheme uses the alphabet extended with `[ '|', '+', '-', ';', '_', 'e' ]`.
///
/// The text may start  with a quotation mark and may end with a quotation mark.
/// The quoted segments should not be empty, and should not immediately follow one another.
/// Any violation of assumptions leads to the algorithm leaving a single `'e'` in the output string.
const SUBSTITUTION_FORMULAS: &str = r##"_a→a_
_A→A_
_b→b_
_B→B_
_c→c_
_C→C_
"_;→e
_;a→"A_
_;b→"B_
_;c→"C_
_;-→e
_;→"_
_-→"_
_→⋅
+a→a+
+A→A+
+b→b+
+B→B+
+c→c+
+C→C+
+"→-
+→e
ae→e
Ae→e
be→e
Be→e
ce→e
Ce→e
;e→e
-e→e
ea→e
eA→e
eb→e
eB→e
ec→e
eC→e
e;→e
e-→e
"e→e
e→⋅e
"→;+
|a→_A
|A→_A
|b→_B
|B→_B
|c→_C
|c→_C
|→_
→|"##;

#[test]
fn the_algorithm_works_on_empty_input() {
    let scheme = prepare_scheme();

    let result = scheme.apply("", 1_000).unwrap();

    assert_eq!("", result.word());
    assert_eq!(3, result.steps_done());
}

#[test]
fn the_quoted_text_and_first_word_are_capitalized() {
    let scheme = prepare_scheme();

    let result = scheme.apply("ac\"bbb\"ca", 1_000).unwrap();

    assert_eq!("Ac\"Bbb\"ca", result.word());
    assert_eq!(16, result.steps_done());
}

#[test]
fn the_sting_can_end_with_a_quotation_mark() {
    let scheme = prepare_scheme();

    let result = scheme.apply("ac\"bbb\"ca\"Ab\"", 1_000).unwrap();

    assert_eq!("Ac\"Bbb\"ca\"Ab\"", result.word());
    assert_eq!(24, result.steps_done());
}

#[test]
fn the_quotation_marks_should_be_balanced() {
    let scheme = prepare_scheme();

    let result = scheme.apply("ac\"bbb\"c\"Aa", 1_000).unwrap();

    assert_eq!("e", result.word());
    assert_eq!(21, result.steps_done());
}

#[test]
fn quoted_string_cannot_be_empty() {
    let scheme = prepare_scheme();

    let result = scheme.apply("ac\"\"ca\"Ab\"c", 1_000).unwrap();

    assert_eq!("e", result.word());
    assert_eq!(21, result.steps_done());
}

#[test]
fn quoted_text_has_to_be_separated() {
    let scheme = prepare_scheme();

    let result = scheme.apply("ac\"c\"\"Ab\"c", 1_000).unwrap();

    assert_eq!("e", result.word());
    assert_eq!(23, result.steps_done());
}

#[test]
fn the_algorithm_works_on_fuzzed_input() {
    let scheme = prepare_scheme();
    let mut generator = rand::thread_rng();
    let characters = ['a', 'b', 'c', 'A', 'B', 'C', '"'];

    let mut test = move || {
        let string: String = iter::repeat_with(|| {
            characters
                .choose(&mut generator)
                .expect("The slice to choose from is not empty.")
        })
        .take(50)
        .collect();

        let expected = capitalize_the_first_word_and_quoted_text(&string);

        let result = scheme.apply(&string, 50_000).unwrap();

        assert_eq!(expected, result.word());
    };

    for _ in 1..1_000 {
        test();
    }
}

fn prepare_scheme() -> AlgorithmScheme {
    let set: HashSet<_> = ['a', 'A', 'b', 'B', 'c', 'C', '"'].into_iter().collect();

    let alphabet = Alphabet::try_from(&set)
        .unwrap()
        .extend('|')
        .unwrap()
        .extend('+')
        .unwrap()
        .extend('-')
        .unwrap()
        .extend(';')
        .unwrap()
        .extend('_')
        .unwrap()
        .extend('e')
        .unwrap();

    AlgorithmSchemeBuilder::new()
        .with_alphabet(alphabet)
        .build_with_formula_definitions(SUBSTITUTION_FORMULAS.lines())
        .unwrap()
}

fn capitalize_the_first_word_and_quoted_text(string: &str) -> String {
    let error = "e".to_owned();
    let quotes = "\"".to_owned();

    fn first_to_upper(part: &str) -> String {
        if !part.is_empty() {
            let mut string = part[0..1].to_uppercase();
            if part.len() > 1 {
                string.push_str(&part[1..]);
            }
            string
        } else {
            part.to_owned()
        }
    }

    if string.matches('"').count() != 0 {
        let parts: Vec<_> = string.split('"').collect();

        if parts.len() % 2 == 0 {
            // unbalanced quotes
            return error;
        }

        if parts
            .iter()
            .skip(1)
            .take(parts.len() - 2)
            .any(|part| part.is_empty())
        {
            // quotes follow quotes
            return error;
        }

        parts
            .iter()
            .take(parts.len() - 1)
            .enumerate()
            .flat_map(|(index, &part)| {
                if index == 0 || index % 2 == 1 {
                    [first_to_upper(part), quotes.clone()]
                } else {
                    [part.to_owned(), quotes.clone()]
                }
            })
            .chain(iter::once(parts[parts.len() - 1].to_owned()))
            .collect::<String>()
    } else {
        first_to_upper(string)
    }
}
