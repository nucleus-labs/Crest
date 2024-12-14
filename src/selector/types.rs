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
    Namespace(SelectorNode),
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
        sens: bool,
    },
    PseudoClass,
    PseudoElement,
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
