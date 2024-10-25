
use clap::{command, Parser as Cli};
use pest::Parser as _;

use std::ffi::OsString;
use std::fs::read_to_string;

// use peacock_crest::parse::{CssParser, Rule, gen_token};
use peacock_crest as crest;

#[derive(Cli)]
#[command(
    name = "validate-grammar",
    about = "Run the generated css parser on the provided source"
)]
struct ValidationArgs {
    // /// the name of the parser rule to evaluate
    // rule: String,

    /// parse contents of file at this path
    source_path: String,
}

fn main() {
    let arguments = ValidationArgs::parse();
    let path = OsString::from(arguments.source_path);
    let source = read_to_string(path).unwrap();

    let rules = crest::parse(source);
    for _rule in rules.iter() {
        // print!("{}, ", token.get_source().unwrap());
    }
}
