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

mod input;

use std::{fs::File, io::Read, path::PathBuf};

use anyhow::{Context, Ok, Result};
use clap::{ArgGroup, Parser};

use markovalgorithms::prelude::*;

use crate::input::UserInputHandler;

fn main() -> Result<()> {
    let parsed = Cli::parse();

    let builder = parsed.create_builder()?;

    let scheme_definition = parsed.read_scheme()?;

    let scheme = builder
        .build_with_formula_definitions(scheme_definition.lines())
        .with_context(|| "Failed to create the algorithm scheme")?;

    if parsed.interactive {
        iterate_over_scheme_results(&scheme, &parsed.string)
    } else {
        apply_scheme(
            &scheme,
            &parsed.string,
            parsed
                .limit
                .expect("Either interactive flag or limit are provided."),
        )
    }
}

/// A derived parser to handle console arguments.
#[derive(Parser)]
#[clap(
    author = "Sergey Ivanov <quixoticaxisgit@gmail.com>, <quixoticaxisgit@mail.ru>",
    version,
    about = "A CLI utility to apply Markov algorithm schemes.",
    long_about = "A CLI utility to apply Markov algorithm schemes. \
    Enables both full and interactive application of algorithm schemes. \
    Licensed under GPL-3.0.",
    group(
        ArgGroup::new("application_arguments")
            .required(true)
            .args(&["limit", "interactive", ]),
    )
)]
struct Cli {
    /// An optional string of characters to be used as an alphabet.
    #[clap(
        short,
        long,
        value_parser,
        value_name = "ALPHABET_CHARACTERS",
        display_order = 1
    )]
    alphabet: Option<String>,

    /// An optional string of characters to be used as an alphabet extension.
    /// This option can be used only together with --alphabet.
    #[clap(
        short = 'e',
        long,
        value_parser,
        value_name = "EXTENSION_CHARACTERS",
        requires = "alphabet",
        display_order = 2
    )]
    alphabet_extension: Option<String>,

    /// An optional character to be used as a delimiter.
    #[clap(short, long, value_parser, value_name = "CHARACTER", display_order = 3)]
    delimiter: Option<char>,

    /// An optional character to be used as a final marker.
    #[clap(short, long, value_parser, value_name = "CHARACTER", display_order = 4)]
    final_marker: Option<char>,

    /// The UTF-8 file that contains the algorithm scheme. Each rule should take its own line. Empty lines are forbidden.
    #[clap(
        short,
        long,
        value_parser,
        value_name = "PATH-TO-FILE",
        display_order = 0
    )]
    scheme: PathBuf,

    /// An input string.
    #[clap(value_parser, value_name = "INPUT")]
    string: String,

    /// When set, defines the limit of steps the algorithm is allowed to take.
    #[clap(short, long, value_parser = clap::value_parser!(u32).range(1..), value_name = "NUMBER-OF-STEPS", display_order = 5)]
    limit: Option<u32>,

    /// When set, enables interactive iteration through algorithm steps.
    #[clap(short, long, action, display_order = 6)]
    interactive: bool,
}

impl Cli {
    fn create_builder(&self) -> Result<AlgorithmSchemeBuilder> {
        let builder = AlgorithmSchemeBuilder::default();

        let builder = if let Some(delimiter) = self.delimiter {
            builder.with_delimiter(delimiter)
        } else {
            builder
        };
        let builder = if let Some(final_marker) = self.final_marker {
            builder.with_final_marker(final_marker)
        } else {
            builder
        };
        let builder = if let Some(alphabet) = self
            .create_alphabet()
            .with_context(|| "Failed to parse the alphabet provided by the user")?
        {
            builder.with_alphabet(alphabet)
        } else {
            builder
        };

        Ok(builder)
    }

    fn create_alphabet(&self) -> Result<Option<Alphabet>> {
        use std::result::Result::Ok;
        if let Some(alphabet) = &self.alphabet {
            let alphabet: Alphabet =
                str::parse(alphabet).with_context(|| "Failed to parse the alphabet definition")?;

            let alphabet = if let Some(extension) = &self.alphabet_extension {
                extension
                    .chars()
                    .fold(Ok(alphabet), |result, character| match result {
                        Ok(alphabet) => alphabet.extend(character),
                        error => error,
                    })
                    .with_context(|| "Failed to extend the alphabet definition")?
            } else {
                alphabet
            };

            Ok(Some(alphabet))
        } else {
            Ok(None)
        }
    }

    fn read_scheme(&self) -> Result<String> {
        let path = || self.scheme.clone();

        let mut file = File::options().read(true).open(path()).with_context(|| {
            format!(
                "Failed to open the algorithm scheme definition from file: {:?}",
                path()
            )
        })?;

        let mut buffer = String::new();
        _ = file.read_to_string(&mut buffer).with_context(|| {
            format!(
                "Failed to read the algorithm scheme definition from file: {:?}",
                path()
            )
        })?;

        Ok(buffer)
    }
}

fn apply_scheme(scheme: &AlgorithmScheme, word: &str, limit: u32) -> Result<()> {
    let result = scheme
        .apply(word, limit)
        .with_context(|| "Failed to apply the algorithm scheme to the input")?;

    println!(
        "The algorithm is finished after taking {} steps. The output string is \"{}\".",
        result.steps_done(),
        result.word()
    );

    Ok(())
}

fn iterate_over_scheme_results(scheme: &AlgorithmScheme, word: &str) -> Result<()> {
    let mut old_word = word.to_owned();

    let iterator = scheme
        .get_application_iterator(word)
        .with_context(|| "Failed to apply the algorithm scheme to the input")?;

    let mut final_step = 0;
    let mut input_handler = UserInputHandler::setup()?;

    for (step, result) in iterator.enumerate() {
        final_step = step
            .checked_add(1)
            .with_context(|| "Too many steps taken")?;

        let new_word = result.word();

        if let Some(forumula_definition) = result.applied_formula_definition() {
            println!(
                "Transformed the word \"{old_word}\" to the word \"{new_word}\" \
                by applying the substitution formula \"{forumula_definition}\"."
            );

            old_word = new_word.to_owned();

            if !input_handler.should_continue()? {
                println!("Stopping due to the received Ctrl-C signal.");

                return Ok(());
            }
        } else {
            println!("No transformaion was made, no rules were applied.");
            break;
        }
    }

    println!(
        "The algorithm is finished after taking {final_step} steps. The output string is \"{old_word}\".");

    Ok(())
}
