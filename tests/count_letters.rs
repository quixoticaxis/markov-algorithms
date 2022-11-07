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

/// A scheme of the algorithm that, when applied to a string in alphabet `[ 'a', 'b', 'c' ]`,
/// returns the number of characters `'a'` in the input string.
///
/// The scheme uses the alphabet extended with digits and `'|'`.
///
/// The text may contain zero 'a' characters or may be empty.
const SUBSTITUTION_FORMULAS: &str = r##"b→
c→
0|0→10
1|0→20
2|0→30
3|0→40
4|0→50
5|0→60
6|0→70
7|0→80
8|0→90
9|0→|00
|0→10
0a→1
1a→2
2a→3
3a→4
4a→5
5a→6
6a→7
7a→8
8a→9
9a→|0
a→1
1→⋅1
2→⋅2
3→⋅3
4→⋅4
5→⋅5
6→⋅6
7→⋅7
8→⋅8
9→⋅9
→⋅0"##;

#[test]
fn the_count_is_correct_even_if_there_are_no_occurences() {
    let scheme = prepare_scheme();

    let result = scheme.apply("bccb", 1_000).unwrap();

    assert_eq!("0", result.word());
    assert_eq!(5, result.steps_done());
}

#[test]
fn the_algorithm_works_on_the_empty_input() {
    let scheme = prepare_scheme();

    let result = scheme.apply("", 1_000).unwrap();

    assert_eq!("0", result.word());
    assert_eq!(1, result.steps_done());
}

#[test]
fn the_number_of_characters_a_is_correctly_counted() {
    let scheme = prepare_scheme();

    let result = scheme.apply("aabaccb", 1_000).unwrap();

    assert_eq!("3", result.word());
    assert_eq!(8, result.steps_done());
}

#[test]
fn the_algorithm_works_on_fuzzed_input() {
    let scheme = prepare_scheme();
    let mut generator = rand::thread_rng();
    let characters = ['a', 'b', 'c'];

    let mut test = move || {
        let string: String = iter::repeat_with(|| {
            characters
                .choose(&mut generator)
                .expect("The slice to choose from is not empty.")
        })
        .take(1_000)
        .collect();

        let expected = string
            .chars()
            .filter(|character| *character == 'a')
            .count()
            .to_string();

        let result = scheme.apply(&string, 50_000).unwrap();

        assert_eq!(expected, result.word());
    };

    for _ in 1..1_00 {
        test();
    }
}

fn prepare_scheme() -> AlgorithmScheme {
    let set: HashSet<_> = ['a', 'b', 'c'].into_iter().collect();

    let alphabet = ('0'..='9')
        .into_iter()
        .fold(
            Alphabet::try_from(&set).unwrap(),
            |accumulator, character| accumulator.extend(character).unwrap(),
        )
        .extend('|')
        .unwrap();

    AlgorithmSchemeBuilder::new()
        .with_alphabet(alphabet)
        .build_with_formula_definitions(SUBSTITUTION_FORMULAS.lines())
        .unwrap()
}
