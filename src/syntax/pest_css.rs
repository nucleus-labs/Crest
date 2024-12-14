use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "css.pest"]
pub(crate) struct CssParser;

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
