#![allow(unused_imports)]

use clap::{arg, command, Parser as Cli};
use pest::iterators::{Pair, Pairs};
use pest::Parser;

use std::collections::HashMap;
use std::fs::read_to_string;

use peacock_crest::css::types::{Generic, PropertyList, RuleSet};
use peacock_crest::parse::{CssParser, Rule};

macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$(($k, $v),)*]))
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$($v,)*]))
    }};
}

#[derive(Cli)]
#[command(
    name = "validate-grammar",
    about = "Run the generated css parser on the provided source"
)]
struct ValidationArgs {
    /// use `source` as a filepath to read instead of raw text
    #[arg(short, long)]
    file: bool,

    /// do not print the children of matches
    #[arg(short, long)]
    no_print_children: bool,

    /// the name of the parser rule to evaluate
    rule: String,

    /// default: raw-text to parse ; --file mode: read file at this path, parse contents
    source: String,
}

fn main() {
    let rule_map: HashMap<&str, Rule> = collection! {
        "css"               => Rule::Css,
        "rule"              => Rule::Rule,
        "selector"          => Rule::Selector,
        "complexselector"   => Rule::ComplexSelector,
        "combinator"        => Rule::Combinator,
        "nextsibling"       => Rule::NextSibling,
        "child"             => Rule::Child,
        "column"            => Rule::Column,
        "subsequentsibling" => Rule::SubsequentSibling,
        "descendent"        => Rule::Descendent,
        "namespace"         => Rule::Namespace,
        "compoundselector"  => Rule::CompoundSelector,
        "simpleselector"    => Rule::SimpleSelector,
        "basicselector"     => Rule::BasicSelector,
        "typeselector"      => Rule::TypeSelector,
        "classselector"     => Rule::ClassSelector,
        "idselector"        => Rule::IdSelector,
        "universalselector" => Rule::UniversalSelector,
        "propertylist"      => Rule::PropertyList,
        "property"          => Rule::Property,
        "valuelist"         => Rule::ValueList,
        "value"             => Rule::Value,
        "number"            => Rule::Number,
        "int"               => Rule::Int,
        "float"             => Rule::Float,
        "unit"              => Rule::Unit,
        "string"            => Rule::String,
        "identifier"        => Rule::Identifier,
    };

    let arguments = ValidationArgs::parse();

    let source: String;

    let match_rule = rule_map
        .get(arguments.rule.to_lowercase().as_str())
        .expect(format!("Could not parse rule type '{}'!", arguments.rule).as_str());

    if arguments.file {
        let result = read_to_string(&arguments.source);
        match result {
            Ok(contents) => source = contents,
            Err(_) => panic!("Failed to read '{}'!", arguments.source),
        }
    } else {
        source = arguments.source;
    }

    CssParser::parse(match_rule.clone(), source.as_str()).unwrap();
}
