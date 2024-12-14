use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "selector.pest"]
pub(crate) struct SelectorParser;
