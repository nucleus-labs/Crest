mod pest_selector;
mod types;

pub use pest_selector::Rule as SelectorRule;
pub use types::{SelectorAttributeType, SelectorCombinator};

pub(crate) use pest_selector::SelectorParser;
pub(crate) use types::{SelectorExpectError, SelectorSubclassType};

use std::cell::Cell;
use std::collections::HashMap;
use std::sync::Arc;

use crate::boo::Boo;
use crate::source::{SourceSlice, StackInfo, TokenTracker};
use crate::syntax::CssToken;

pub(crate) type SelectorStackInfo = StackInfo<SelectorRule>;
pub(crate) type SelectorToken = crate::source::ParserToken<SelectorRule>;

#[derive(Debug, Clone, Default)]
pub struct SelectorNode {
    pub universal: bool,
    pub namespace: Option<String>,
    pub type_name: Option<String>,

    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attributes: HashMap<String, Vec<(SelectorAttributeType, String)>>,

    pub parent: Option<Arc<Self>>,
    pub siblings: Vec<Arc<Self>>,
    pub children: Vec<Arc<Self>>,
    pub descendents: Vec<Arc<Self>>,
}

#[derive(Debug, Clone)]
pub struct SelectorTokenTracker<'a>(crate::source::TokenTracker<'a, SelectorRule>);

impl<'a> SelectorTokenTracker<'a> {
    #[inline]
    pub fn new(tokens_boo: Boo<'a, Vec<SelectorToken>>) -> Self {
        Self(TokenTracker::new(tokens_boo))
    }

    #[inline]
    fn fail_type<O>(&self, expected: SelectorRule) -> Result<O, SelectorExpectError> {
        let current_rule = self.boo.get(0.max(self.idx.get() - 1)).unwrap().get_rule();
        self.0.fail_because(SelectorExpectError::FailedExpectation(
            current_rule,
            expected,
        ))
    }

    /// Consumes the next token, constructs a `Box<[crate::SelectorNode]>` representation
    /// if it is a valid selector token. Returns an `SelectorExpectError` otherwise.
    pub fn expect_selector_list(&'a self) -> Result<Box<[SelectorNode]>, SelectorExpectError> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(SelectorExpectError::TooFewTokens("SELECTOR_LIST".into()))?;

        if !matches!(token.get_rule(), SelectorRule::SELECTOR_LIST) {
            self.fail_type(SelectorRule::SELECTOR_LIST)?;
        }

        // SELF: SELECTOR_LIST -> SELECTOR_COMPLEX_LIST [ children ]
        let components = token.get_children().unwrap()[0].get_children().unwrap();
        let list_expector = Self::new(Boo::Borrowed(components));

        let selectors: Vec<SelectorNode> = Vec::new();

        while let Ok((selectors, combinators)) = list_expector.expect_complex() {
            // let compound_expector = Self::new(Boo::Owned(selectors));
            // let mut selector = compound_expector.expect_compound()?;

            // let mut prev: &SelectorNode = &selector;
            // for compound
        }

        Ok(selectors.into_boxed_slice())
    }

    pub fn expect_complex(
        &'a self,
    ) -> Result<(SelectorNode, Vec<SelectorCombinator>), SelectorExpectError> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(SelectorExpectError::TooFewTokens("SELECTOR_COMPLEX".into()))?;

        if !matches!(token.get_rule(), SelectorRule::SELECTOR_COMPLEX) {
            self.fail_type(SelectorRule::SELECTOR_COMPLEX)?;
        }

        let components = token.get_children().unwrap();
        let complex_expector = Self::new(Boo::Borrowed(components));

        let first = complex_expector.expect_compound()?;
        let mut rest: Vec<SelectorCombinator> = Vec::new();

        while !complex_expector.is_empty() {
            let combinator_token: &SelectorToken =
                complex_expector
                    .pop_front()
                    .ok_or(SelectorExpectError::TooFewTokens(
                        "SELECTOR_COMPOUND".into(),
                    ))?;
            let next = complex_expector.expect_compound()?;

            let combinator: SelectorCombinator = match combinator_token.get_rule() {
                SelectorRule::SELECTOR_COMBINATOR__NEXT_SIBLING => {
                    SelectorCombinator::NextSibling(next)
                }
                SelectorRule::SELECTOR_COMBINATOR__CHILD => SelectorCombinator::Child(next),
                SelectorRule::SELECTOR_COMBINATOR__COLUMN => SelectorCombinator::Column(next),
                SelectorRule::SELECTOR_COMBINATOR__SUBSEQUENT_SIBLING => {
                    SelectorCombinator::SubsequentSibling(next)
                }
                SelectorRule::SELECTOR_COMBINATOR__NAMESPACE => SelectorCombinator::Namespace(next),
                SelectorRule::SELECTOR_COMBINATOR__DESCENDENT => {
                    SelectorCombinator::Descendent(next)
                }

                _ => unreachable!(),
            };

            rest.push(combinator);
        }

        Ok((first, rest))
    }

    pub fn expect_compound(&'a self) -> Result<SelectorNode, SelectorExpectError> {
        let token: &SelectorToken = self.pop_front().ok_or(SelectorExpectError::TooFewTokens(
            "SELECTOR_COMPOUND".into(),
        ))?;

        if !matches!(token.get_rule(), SelectorRule::SELECTOR_COMPOUND) {
            self.fail_type(SelectorRule::SELECTOR_COMPOUND)?;
        }

        let components = token.get_children().unwrap();
        let mut selector = SelectorNode::default();

        let selector_expector = Self::new(Boo::Borrowed(components));

        while !selector_expector.is_empty() {
            let component = selector_expector.peek().unwrap();
            match component.get_rule() {
                SelectorRule::SELECTOR_TYPE => {
                    let (name, is_namespace) = selector_expector.expect_type()?;
                    if is_namespace {
                        selector.namespace = name;
                        selector.universal = true;
                    } else {
                        selector.type_name = name;
                    }
                }
                SelectorRule::SELECTOR_SUBCLASS => {
                    let subclass = selector_expector.expect_subclass()?;
                    todo!();
                }
                SelectorRule::SELECTOR_PSEUDOCLASS => todo!(),
                SelectorRule::SELECTOR_PSEUDOELEMENT => todo!(),

                _ => (),
            }
        }

        Ok(selector)
    }

    fn expect_subclass(&'a self) -> Result<SelectorSubclassType, SelectorExpectError> {
        let token: &SelectorToken = self.pop_front().ok_or(SelectorExpectError::TooFewTokens(
            "SELECTOR_SUBCLASS".into(),
        ))?;

        if !matches!(token.get_rule(), SelectorRule::SELECTOR_SUBCLASS) {
            self.fail_type(SelectorRule::SELECTOR_SUBCLASS)?;
        }

        let subclass = &token.get_children().unwrap()[0];
        match subclass.get_rule() {
            SelectorRule::SELECTOR_ID => {
                let hash_token = &subclass.get_children().unwrap()[0];
                let ident_token = &hash_token.get_children().unwrap()[0];

                Ok(SelectorSubclassType::Id(
                    ident_token.get_source().to_string(),
                ))
            }
            SelectorRule::SELECTOR_CLASS => {
                let ident_token = &subclass.get_children().unwrap()[0];

                Ok(SelectorSubclassType::Class(
                    ident_token.get_source().to_string(),
                ))
            }
            SelectorRule::SELECTOR_ATTRIBUTE => {
                let subclass_components = subclass.get_children().unwrap();
                let subclass_expector = Self::new(Boo::Borrowed(subclass_components));

                let ident = subclass_expector.expect_identifier()?;
                if subclass_components.len() > 1 {
                    let matcher_type = subclass_expector.expect_attr_matcher()?;

                    let attr_val = match subclass_expector.peek().unwrap().get_rule() {
                        SelectorRule::STRING => subclass_expector.expect_quoted_string()?,
                        SelectorRule::IDENT => subclass_expector.expect_identifier()?,

                        _ => unreachable!(),
                    };

                    let case_sens = if subclass_expector.len() > 0 {
                        subclass_expector.expect_attr_modifier()?
                    } else {
                        false
                    };

                    Ok(SelectorSubclassType::Attribute {
                        name: ident.to_string(),
                        attr_matcher: Some(matcher_type),
                        attr_val: Some(attr_val.to_string()),
                        sens: case_sens,
                    })
                } else {
                    Ok(SelectorSubclassType::Attribute {
                        name: ident.to_string(),
                        attr_matcher: None,
                        attr_val: None,
                        sens: false,
                    })
                }
            }
            SelectorRule::SELECTOR_PSEUDOCLASS => todo!(),

            _ => unreachable!(),
        }
    }

    fn expect_attr_matcher(&self) -> Result<SelectorAttributeType, SelectorExpectError> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(SelectorExpectError::TooFewTokens("ATTR_MATCHER".into()))?;

        if !matches!(token.get_rule(), SelectorRule::ATTR_MATCHER) {
            self.fail_type(SelectorRule::ATTR_MATCHER)?;
        }

        match token.get_source().parse::<SelectorAttributeType>() {
            Ok(matcher) => Ok(matcher),
            Err(_) => Ok(SelectorAttributeType::Present),
        }
    }

    fn expect_attr_modifier(&self) -> Result<bool, SelectorExpectError> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(SelectorExpectError::TooFewTokens("ATTR_MODIFIER".into()))?;

        if !matches!(token.get_rule(), SelectorRule::ATTR_MODIFIER) {
            self.fail_type(SelectorRule::ATTR_MODIFIER)?;
        }

        match token.get_source().get() {
            "s" | "S" => Ok(true),
            "i" | "I" => Ok(false),

            _ => unreachable!(),
        }
    }

    // namespace, type_name
    fn expect_type(&self) -> Result<(Option<String>, bool), SelectorExpectError> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(SelectorExpectError::TooFewTokens("SELECTOR_TYPE".into()))?;

        if !matches!(token.get_rule(), SelectorRule::SELECTOR_TYPE) {
            self.fail_type(SelectorRule::SELECTOR_TYPE)?;
        }

        todo!()
    }

    /// Consumes the next token and extracts its `SourceSlice` representation if it is a valid
    /// identifier. Returns an `SelectorExpectError` otherwise.
    pub fn expect_identifier(&self) -> Result<SourceSlice, SelectorExpectError> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(SelectorExpectError::TooFewTokens("IDENT".into()))?;

        if !matches!(token.get_rule(), SelectorRule::IDENT) {
            self.fail_type(SelectorRule::IDENT)?;
        }

        Ok(token.get_source())
    }

    /// Consumes the next token and extracts its `SourceSlice` representation if it is a valid quoted
    /// string token. Excludes surrounding quotes from the result. Returns an `SelectorExpectError` otherwise.
    pub fn expect_quoted_string(&self) -> Result<SourceSlice, SelectorExpectError> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(SelectorExpectError::TooFewTokens("STRING".into()))?;

        if !matches!(token.get_rule(), SelectorRule::STRING) {
            self.fail_type(SelectorRule::STRING)?;
        }

        let mut slice = token.get_source();
        slice.start = slice.source_info.location_from_idx(slice.start.idx + 1);
        slice.end = slice.source_info.location_from_idx(slice.end.idx - 1);

        Ok(slice)
    }
}

impl std::fmt::Display for SelectorNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.universal {
            write!(f, "*");
        } else {
            if let Some(namespace) = &self.namespace {
                write!(f, "{namespace}:");
            }
            if let Some(type_name) = &self.type_name {
                write!(f, "{type_name}");
            }
            if let Some(id) = &self.id {
                write!(f, "#{id}");
            }
        }

        for (attr, ops) in self.attributes.iter() {
            for (op, val) in ops.iter() {
                match op {
                    SelectorAttributeType::Present => write!(f, "[{attr}]"),
                    SelectorAttributeType::ExactMatch => write!(f, "[{attr} = {val}]"),
                    SelectorAttributeType::ListContains => write!(f, "[{attr} ~= {val}]"),
                    SelectorAttributeType::StartsWith => write!(f, "[{attr} ^= {val}]"),
                    SelectorAttributeType::StartsWithDashed => write!(f, "[{attr} |= {val}]"),
                    SelectorAttributeType::Endswith => write!(f, "[{attr} $= {val}]"),
                    SelectorAttributeType::RawContains => write!(f, "[{attr} *= {val}]"),
                };
            }
        }

        for class in self.classes.iter() {
            write!(f, ".{class}");
        }

        for child in self.children.iter() {
            write!(f, " {child}");
        }

        Ok(())
    }
}

impl<'a> std::ops::Deref for SelectorTokenTracker<'a> {
    type Target = crate::source::TokenTracker<'a, SelectorRule>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
