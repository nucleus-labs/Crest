
use strum_macros::EnumString;

use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Selector(pub CompoundSelector);

pub type CompoundSelector = (ComplexSelector, Vec<ComplexSelector>);
pub type ComplexSelector = (SimpleSelector, Vec<Combination>); // value [combination...]
pub type Combination = (Combinator, SimpleSelector); // <operator> value

#[derive(Debug, Clone, EnumString)]
pub enum Combinator {
    None,
    #[strum(serialize = "+")]
    NextSibling,
    #[strum(serialize = ">")]
    Child,
    #[strum(serialize = "||")]
    Column,
    #[strum(serialize = "~")]
    SubsequentSibling,
    #[strum(serialize = "|")]
    Namespace,
    #[strum(serialize = " ")]
    Descendent,
}

#[derive(Clone, Debug)]
pub struct SimpleSelector {
    pub name: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub universal: bool,
}

#[derive(Clone, Debug)]
pub enum BasicSelector {
    Id(String),
    Class(String),
    Type(String),
    Universal,
}

// ============== IMPL ==============

impl BasicSelector {
    pub fn inner(&self) -> Option<String> {
        match self {
            BasicSelector::Id(string) => Some(string.clone()),
            BasicSelector::Class(string) => Some(string.clone()),
            BasicSelector::Type(string) => Some(string.clone()),
            _ => None,
        }
    }
}

impl SimpleSelector {
    pub fn new() -> Self {
        Self {
            name: None,
            id: None,
            classes: Vec::new(),
            universal: false,
        }
    }
}

impl Selector {
    pub fn matches(&self, other: &impl super::Styleable) -> bool {
        false
    }

    pub fn specificity(&self) -> usize {
        0usize
    }
}
