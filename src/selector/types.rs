use crate::source::SourceSlice;

use super::{SelectorNode, SelectorRule, SelectorToken};

#[derive(Debug, Clone)]
pub enum SelectorAttributeType {
    Present,                  // [attr]
    ExactMatch(String),       // [attr=val]
    ListContains(String),     // [attr~=val]
    StartsWith(String),       // [attr^=val]
    StartsWithDashed(String), // [attr|=val]
    Endswith(String),         // [attr$=val]
    RawContains(String),      // [attr*=val]
}

#[derive(Debug, Clone)]
pub enum SelectorCombinator {
    SubsequentSibling(SelectorNode),
    NextSibling(SelectorNode),
    Descendent(SelectorNode),
    Column(SelectorNode),
    Child(SelectorNode),
}

#[derive(Debug, Clone)]
pub(crate) enum SelectorSubclassType {
    Id(String),
    Class(String),
    Attribute {
        name: String,
        attr_matcher: SelectorAttributeType,
        case_sensitive: bool,
    },
    PseudoClass(String),
    PseudoElement(String, Option<Box<[crate::Unit]>>),
}

#[derive(Debug, Clone, derive_more::From)]
pub(crate) enum SelectorExpectError {
    #[from]
    ParseError(pest::error::Error<SelectorRule>),
    TooFewTokens(String),
    FailedExpectation(SelectorRule, SelectorRule),
    InvalidCombinator(SelectorToken),
    UnknownSelectorRule(SelectorRule),
}

pub type SelectorResult<T> = Result<T, SelectorExpectError>;

impl SelectorCombinator {
    pub fn get_node<'a>(&'a mut self) -> &'a mut SelectorNode {
        match self {
            SelectorCombinator::SubsequentSibling(selector_node) => selector_node,
            SelectorCombinator::NextSibling(selector_node) => selector_node,
            SelectorCombinator::Descendent(selector_node) => selector_node,
            SelectorCombinator::Column(selector_node) => selector_node,
            SelectorCombinator::Child(selector_node) => selector_node,
        }
    }
}
