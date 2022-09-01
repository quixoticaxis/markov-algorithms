# markov-algorithms
Rust implementation of [Markov algorithms](https://en.wikipedia.org/wiki/Markov_algorithm) executor.

This crate is created purely for educational purposes and is published under GPL-3.0 license.

The documentation can be found on [docs.rs](https://docs.rs/markov-algorithms).

## Library
You can use the crate as a library.

Add the dependency to `Cargo.toml`:
```toml
markov-algorithms = "0.4"
```

Define a scheme of the algorithm:
```rust
use std::str;
use markovalgorithms::prelude::*;

let alphabet = str::parse::<Alphabet>("abc").unwrap().extend('d').unwrap();
let scheme = AlgorithmSchemeBuilder::new()
    .with_alphabet(alphabet)
    .build_with_formula_definitions(["a→⋅d"].into_iter())
    .unwrap();
```
Apply the scheme:
```rust
let result = scheme.apply("abc", 1).unwrap();

assert_eq!("dbc", result.word());
assert_eq!(1, result.steps_done());```
```
You may also apply the scheme once to inspect a single step of the algorithm or get an iterator to apply the scheme step by step:
```rust
let mut iterator = scheme.get_application_iterator("abc").unwrap();

assert_eq!("dbc", iterator.next().unwrap().word());
assert_eq!(None, iterator.next())
```

### Examples
See the `/tests` forlder for more complex schemes.

## Tool
You can use a simple clap-based CLI tool to execute algorithms defined by the schemes loaded from UTF-8 files.

Install with cargo:
```
cargo install markov-algorithms
```
It would install `markovalgorithms-cli` tool. Launch `markovalgorithms-cli` with `--help` flag to see the descriptions of parameters and usage example.
