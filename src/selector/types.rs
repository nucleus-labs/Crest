use super::{SelectorNode, SelectorRule, SelectorToken};

#[derive(Debug, Clone, strum_macros::EnumString)]
pub enum SelectorAttributeType {
    Present, // [attr]
    #[strum(serialize = "=")]
    ExactMatch, // [attr=val]
    #[strum(serialize = "~=")]
    ListContains, // [attr~=val]
    #[strum(serialize = "^=")]
    StartsWith, // [attr^=val]
    #[strum(serialize = "|=")]
    StartsWithDashed, // [attr|=val]
    #[strum(serialize = "$=")]
    Endswith, // [attr$=val]
    #[strum(serialize = "*=")]
    RawContains, // [attr*=val]
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
    //
    Attribute {
        name: String,
        attr_matcher: Option<SelectorAttributeType>,
        attr_val: Option<String>,
        sens: bool,
    },
    PseudoClass,
    PseudoElement,
}

#[derive(Debug, Clone)]
pub(crate) enum SelectorExpectError {
    TooFewTokens(String),
    FailedExpectation(SelectorRule, SelectorRule),
    InvalidCombinator(SelectorToken),
    UnknownSelectorRule(SelectorRule),
}
