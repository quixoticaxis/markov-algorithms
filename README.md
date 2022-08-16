# markov-algorithms
Rust implementation of [Markov algorithms](https://en.wikipedia.org/wiki/Markov_algorithm).

This crate is created purely for educational purposes.

## Library
You can use the crate as a library.

Add the dependency to `Cargo.toml`:
```toml
markov-algorithms = "0.*"
```

Define a scheme of the algorithm:
```rust
use markovalgorithms::*;

// scheme definition should have only the characters
// that belong to the extended alphabet (whitespace is also a character)
let scheme_definition = "a→b\nb→c\nc→⋅4";

// default configuration uses '→' as delimiter,'⋅' as final marker,
// and includes latin letters, digits, and '|' in the alphabet 
let configuration = Default::default();
let scheme = AlgorithmScheme::new(&configuration, scheme_definition).unwrap();
```
Apply the scheme:
```rust
let string = "aaabc";
// the application attempts are limited by the second argument
let result = scheme.apply(string, 10).unwrap();

assert_eq!("4cccc", result.string());
assert_eq!(8, result.steps_taken())
```

## Tool
You can use a simple clap-based CLI tool to execute algorithms defined by the schemes loaded from a UTF-8 files.

Install with cargo:
```
cargo install markov-algorithms
```
It would install markovalgorithms-cli tool.
