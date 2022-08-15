use std::iter;

use rand::seq::SliceRandom;

use markovalgorithms::*;

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
a→1"##;

#[test]
fn short_case_works() {
    let scheme = prepare_scheme();

    let result = scheme.apply("aabaccb", 1_000).unwrap();

    assert_eq!("3", result.string());
}

#[test]
fn long_case_works() {
    let scheme = prepare_scheme();
    let mut generator = rand::thread_rng();

    let characters = ['a', 'b', 'c'];
    let string: String = iter::repeat_with(|| {
        characters
            .choose(&mut generator)
            .expect("The slice is not empty.")
    })
    .take(500)
    .collect();

    let expected = string
        .chars()
        .filter(|character| *character == 'a')
        .count()
        .to_string();

    let result = scheme.apply(&string, 50_000).unwrap();

    assert_eq!(expected, result.string());
}

fn prepare_scheme() -> AlgorithmScheme {
    let configuration = SubstitutionFormulaConfiguration::over_alphabet(
        '→',
        '⋅',
        ['a', 'b', 'c']
            .into_iter()
            .chain(('0'..='9').into_iter())
            .chain(iter::once('|'))
            .collect(),
    )
    .unwrap();

    AlgorithmScheme::new(&configuration, SUBSTITUTION_FORMULAS).unwrap()
}
