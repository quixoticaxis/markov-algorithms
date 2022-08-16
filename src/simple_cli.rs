use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use clap::Parser;
use markovalgorithms::{AlgorithmScheme, SubstitutionFormulaConfiguration};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CliParser {
    /// A UTF-8 file with scheme definition
    #[clap(short, long, value_parser, value_name = "SCHEME_FILE")]
    scheme_definition_file: PathBuf,
    
    /// A string to apply the scheme to
    #[clap(long, value_parser)]
    string: String,
    
    /// The maximum number of times the scheme would be applied to the string
    #[clap(default_value_t = 10_000, long, value_parser)]
    limit: u32,
    
    /// The character to be used as a delimiter of left and right substitution formula parts
    #[clap(long, value_parser)]
    delimiter: Option<char>,
    
    /// The character to be used to mark a substitution formula as final
    #[clap(long, value_parser)]
    final_marker: Option<char>,

    /// The string containing all of the characters of the extended alphabet to be used
    #[clap(long, value_parser)]
    alphabet: Option<String>,
}

fn main() {
    let parsing_results = CliParser::parse();

    if let Some(scheme_definition) = read_scheme_definition(parsing_results.scheme_definition_file)
    {
        let default_configuration = SubstitutionFormulaConfiguration::default();
        #[allow(clippy::or_fun_call)]
        match SubstitutionFormulaConfiguration::over_alphabet(
            parsing_results
                .delimiter
                .unwrap_or(default_configuration.delimiter()),
            parsing_results
                .final_marker
                .unwrap_or(default_configuration.final_marker()),
            parsing_results
                .alphabet
                .map_or(default_configuration.alphabet().clone(), |string| {
                    string.chars().collect()
                }),
        ) {
            Ok(configuration) => match AlgorithmScheme::new(&configuration, &scheme_definition) {
                Ok(scheme) => match scheme.apply(&parsing_results.string, parsing_results.limit) {
                    Ok(result) => {
                        println!("{result}");
                    }
                    Err(error) => {
                        println!("Failed to apply a scheme. Encountered an error: {error}.")
                    }
                },
                Err(error) => println!("Failed to create a scheme. Encountered an error: {error}."),
            },
            Err(error) => println!("Failed to create a scheme configuration. Encountered an error: {error}.")
        }
        
    }
}

fn read_scheme_definition(path: PathBuf) -> Option<String> {
    if Path::exists(&path) {
        match File::open(path.clone()) {
            Ok(mut file) => {
                let mut buffer = Vec::new();

                match file.read_to_end(&mut buffer) {
                    Ok(_) => match String::from_utf8(buffer) {
                        Ok(sheme_definition) => return Some(sheme_definition),
                        Err(error) => {
                            println!("Failed to decode the file. Encounted an error: {error}.")
                        }
                    },
                    Err(error) => {
                        println!("Failed to read from the file. Encountered an error: {error}.")
                    }
                }
            }
            Err(error) => {
                println!("Could not open the file: {path:?}. Encountered an error: {error}.")
            }
        };
    } else {
        println!("No such file exists: {path:?}.");
    }

    None
}