mod pest_selector;
mod types;

pub use pest_selector::Rule as SelectorRule;
pub use types::{SelectorAttributeType, SelectorCombinator, SelectorResult};

pub(crate) use pest_selector::SelectorParser;
pub(crate) use types::{SelectorExpectError, SelectorSubclassType};

use std::cell::Cell;
use std::collections::HashMap;
use std::sync::Arc;

use crate::boo::Boo;
use crate::source::{ParserToken, SourceSlice, StackInfo, TokenTracker};
use crate::syntax::CssToken;

pub(crate) type SelectorToken = ParserToken<SelectorRule>;
pub(crate) type SelectorStackInfo = StackInfo<SelectorRule>;

#[derive(Debug, Clone, Default)]
pub struct SelectorNode {
    pub universal: bool,
    pub namespace: Option<String>,
    pub type_name: Option<String>,

    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attributes: HashMap<String, Vec<(SelectorAttributeType, bool)>>,

    pub parent: Option<Arc<Self>>,
    pub siblings: Vec<Arc<Self>>,
    pub children: Vec<Arc<Self>>,
    pub descendents: Vec<Arc<Self>>,
}

#[derive(Debug, Clone)]
pub struct SelectorTokenTracker<'a>(crate::source::TokenTracker<'a, SelectorRule>);

impl<'a> SelectorTokenTracker<'a> {
    #[inline]
    pub fn new(selector_token: &'a SelectorToken) -> Option<SelectorTokenTracker<'a>> {
        selector_token
            .get_children()
            .map(|x| Self(TokenTracker::new(Boo::Borrowed(x))))
    }

    #[inline]
    pub fn from_vec(vec: &'a Vec<SelectorToken>) -> Self {
        Self(TokenTracker::new(Boo::Borrowed(vec)))
    }

    #[inline]
    fn fail_type<O>(&self, expected: SelectorRule) -> SelectorResult<O> {
        let current_rule = self.boo.get(0.max(self.idx.get() - 1)).unwrap().get_rule();
        self.0.fail_because(SelectorExpectError::FailedExpectation(
            current_rule,
            expected,
        ))
    }

    /// Consumes the next token, constructs a `Box<[crate::SelectorNode]>` representation
    /// if it is a valid selector token. Returns an `SelectorExpectError` otherwise.
    pub fn expect_selector_list(&'a self) -> SelectorResult<Box<[SelectorNode]>> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(SelectorExpectError::TooFewTokens("SELECTOR_LIST".into()))?;

        if !matches!(token.get_rule(), SelectorRule::SELECTOR_LIST) {
            return self.fail_type(SelectorRule::SELECTOR_LIST);
        }

        // SELF: SELECTOR_LIST -> SELECTOR_COMPLEX_LIST [ children ]
        let list_expector = Self::new(&token.get_children().unwrap()[0]).unwrap();

        let mut selectors: Vec<SelectorNode> = Vec::new();

        while let Ok((selector, combinators)) = list_expector.expect_complex() {
            for combination in combinators.iter() {
                match combination {
                    SelectorCombinator::SubsequentSibling(selector_node) => todo!(),
                    SelectorCombinator::NextSibling(selector_node) => todo!(),
                    SelectorCombinator::Descendent(selector_node) => todo!(),
                    SelectorCombinator::Namespace(selector_node) => todo!(),
                    SelectorCombinator::Column(selector_node) => todo!(),
                    SelectorCombinator::Child(selector_node) => todo!(),
                }
            }
            selectors.push(selector);
        }

        Ok(selectors.into_boxed_slice())
    }

    pub fn expect_complex(&'a self) -> SelectorResult<(SelectorNode, Vec<SelectorCombinator>)> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(SelectorExpectError::TooFewTokens("SELECTOR_COMPLEX".into()))?;

        if !matches!(token.get_rule(), SelectorRule::SELECTOR_COMPLEX) {
            return self.fail_type(SelectorRule::SELECTOR_COMPLEX);
        }

        let complex_expector = Self::new(token).unwrap();

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

    pub fn expect_compound(&'a self) -> SelectorResult<SelectorNode> {
        let token: &SelectorToken = self.pop_front().ok_or(SelectorExpectError::TooFewTokens(
            "SELECTOR_COMPOUND".into(),
        ))?;

        if !matches!(token.get_rule(), SelectorRule::SELECTOR_COMPOUND) {
            return self.fail_type(SelectorRule::SELECTOR_COMPOUND);
        }

        let mut selector = SelectorNode::default();

        let compound_expector = Self::new(token).unwrap();

        while !compound_expector.is_empty() {
            let component = compound_expector.peek().unwrap();
            match component.get_rule() {
                SelectorRule::SELECTOR_TYPE => {
                    let (namespace, name) = compound_expector.expect_type()?;
                    selector.namespace = namespace.map(|x| x.to_string());
                    if name.is_none() {
                        selector.universal = true;
                    }
                    selector.type_name = name.map(|x| x.to_string());
                }
                SelectorRule::SELECTOR_SUBCLASS => {
                    let subclass = compound_expector.expect_subclass()?;
                    match subclass {
                        SelectorSubclassType::Id(id) => {
                            assert!(
                                selector.id.is_none(),
                                "A simple selector cannot have more than one id selector!"
                            );
                            selector.id = Some(id);
                        }
                        SelectorSubclassType::Class(class) => {
                            selector.classes.push(class);
                        }
                        SelectorSubclassType::Attribute {
                            name,
                            attr_matcher,
                            sens,
                        } => {
                            if !selector.attributes.contains_key(&name) {
                                selector.attributes.insert(name.clone(), Vec::new());
                            }
                            selector
                                .attributes
                                .get_mut(&name)
                                .unwrap()
                                .push((attr_matcher, sens))
                        }
                        SelectorSubclassType::PseudoClass => todo!(),
                        SelectorSubclassType::PseudoElement => todo!(),
                    }
                }
                SelectorRule::SELECTOR_PSEUDOCLASS => todo!(),
                SelectorRule::SELECTOR_PSEUDOELEMENT => todo!(),

                _ => (),
            }
        }

        Ok(selector)
    }

    fn expect_subclass(&'a self) -> SelectorResult<SelectorSubclassType> {
        let token: &SelectorToken = self.pop_front().ok_or(SelectorExpectError::TooFewTokens(
            "SELECTOR_SUBCLASS".into(),
        ))?;

        if !matches!(token.get_rule(), SelectorRule::SELECTOR_SUBCLASS) {
            return self.fail_type(SelectorRule::SELECTOR_SUBCLASS);
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
                let subclass_expector = Self::new(subclass).unwrap();

                let ident = subclass_expector.expect_identifier()?;
                if !subclass_expector.is_empty() {
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
                        attr_matcher: matcher_type,
                        sens: case_sens,
                    })
                } else {
                    Ok(SelectorSubclassType::Attribute {
                        name: ident.to_string(),
                        attr_matcher: SelectorAttributeType::Present,
                        sens: false,
                    })
                }
            }
            SelectorRule::SELECTOR_PSEUDOCLASS => todo!(),

            _ => unreachable!(),
        }
    }

    fn expect_attr_matcher(&'a self) -> SelectorResult<SelectorAttributeType> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(SelectorExpectError::TooFewTokens("ATTR_MATCHER".into()))?;

        if !matches!(token.get_rule(), SelectorRule::ATTR_MATCHER) {
            return self.fail_type(SelectorRule::ATTR_MATCHER);
        }

        let matcher_expector = Self::new(token).unwrap();

        let matcher_type = matcher_expector
            .pop_front()
            .ok_or(SelectorExpectError::TooFewTokens("ATTR_MATCHER".into()))?;

        let matcher_value = match matcher_expector.peek().unwrap().get_rule() {
            SelectorRule::STRING => matcher_expector.expect_quoted_string()?,
            SelectorRule::IDENT => matcher_expector.expect_identifier()?,

            _ => unreachable!(),
        };

        match matcher_type.get_rule() {
            SelectorRule::ATTR_MATCHER__EXACT_MATCH => {
                Ok(SelectorAttributeType::ExactMatch(matcher_value.to_string()))
            }
            SelectorRule::ATTR_MATCHER__LIST_CONTAINS => Ok(SelectorAttributeType::ListContains(
                matcher_value.to_string(),
            )),
            SelectorRule::ATTR_MATCHER__STARTS_WITH => {
                Ok(SelectorAttributeType::StartsWith(matcher_value.to_string()))
            }
            SelectorRule::ATTR_MATCHER__STARTS_WITH_DASHED => Ok(
                SelectorAttributeType::StartsWithDashed(matcher_value.to_string()),
            ),
            SelectorRule::ATTR_MATCHER__ENDS_WITH => {
                Ok(SelectorAttributeType::Endswith(matcher_value.to_string()))
            }
            SelectorRule::ATTR_MATCHER__CONTAINS => Ok(SelectorAttributeType::RawContains(
                matcher_value.to_string(),
            )),

            _ => unreachable!(),
        }
    }

    fn expect_attr_modifier(&self) -> SelectorResult<bool> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(SelectorExpectError::TooFewTokens("ATTR_MODIFIER".into()))?;

        if !matches!(token.get_rule(), SelectorRule::ATTR_MODIFIER) {
            return self.fail_type(SelectorRule::ATTR_MODIFIER);
        }

        match token.get_source().get() {
            "s" | "S" => Ok(true),
            "i" | "I" => Ok(false),

            _ => unreachable!(),
        }
    }

    // namespace, name
    fn expect_type(&'a self) -> SelectorResult<(Option<SourceSlice>, Option<SourceSlice>)> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(SelectorExpectError::TooFewTokens("SELECTOR_TYPE".into()))?;

        if !matches!(token.get_rule(), SelectorRule::SELECTOR_TYPE) {
            return self.fail_type(SelectorRule::SELECTOR_TYPE);
        }

        if let Some(type_expector) = Self::new(token) {
            match type_expector.peek().unwrap().get_rule() {
                SelectorRule::PREFIX_NAMESPACE => {
                    let namespace = type_expector.expect_prefix_namespace()?;
                    Ok((namespace, None))
                }
                SelectorRule::WQ_NAME => {
                    let (namespace, name) = type_expector.expect_wq_name()?;
                    Ok((namespace, Some(name)))
                }

                _ => unreachable!(),
            }
        } else {
            Ok((None, None))
        }
    }

    // namespace, name
    pub fn expect_wq_name(&'a self) -> SelectorResult<(Option<SourceSlice>, SourceSlice)> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(SelectorExpectError::TooFewTokens("WQ_NAME".into()))?;

        if !matches!(token.get_rule(), SelectorRule::WQ_NAME) {
            return self.fail_type(SelectorRule::WQ_NAME);
        }

        let wq_expector = Self::new(token).unwrap();

        match wq_expector.peek().unwrap().get_rule() {
            SelectorRule::PREFIX_NAMESPACE => {
                let prefix = wq_expector.expect_prefix_namespace()?;
                let ident = wq_expector.expect_identifier()?;
                Ok((prefix, ident))
            }
            SelectorRule::IDENT => {
                let ident = wq_expector.expect_identifier()?;
                Ok((None, ident))
            }

            _ => unreachable!(),
        }
    }

    pub fn expect_prefix_namespace(&'a self) -> SelectorResult<Option<SourceSlice>> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(SelectorExpectError::TooFewTokens("PREFIX_NAMESPACE".into()))?;

        if !matches!(token.get_rule(), SelectorRule::PREFIX_NAMESPACE) {
            return self.fail_type(SelectorRule::PREFIX_NAMESPACE);
        }

        if let Some(namespace_expector) = Self::new(token) {
            let namespace = namespace_expector.expect_identifier()?;
            Ok(Some(namespace))
        } else {
            Ok(None)
        }
    }

    /// Consumes the next token and extracts its `SourceSlice` representation if it is a valid
    /// identifier. Returns an `SelectorExpectError` otherwise.
    pub fn expect_identifier(&self) -> SelectorResult<SourceSlice> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(SelectorExpectError::TooFewTokens("IDENT".into()))?;

        if !matches!(token.get_rule(), SelectorRule::IDENT) {
            return self.fail_type(SelectorRule::IDENT);
        }

        Ok(token.get_source())
    }

    /// Consumes the next token and extracts its `SourceSlice` representation if it is a valid quoted
    /// string token. Excludes surrounding quotes from the result. Returns an `SelectorExpectError` otherwise.
    pub fn expect_quoted_string(&self) -> SelectorResult<SourceSlice> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(SelectorExpectError::TooFewTokens("STRING".into()))?;

        if !matches!(token.get_rule(), SelectorRule::STRING) {
            return self.fail_type(SelectorRule::STRING);
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
            for (op, sens) in ops.iter() {
                if !sens {
                    match op {
                        SelectorAttributeType::Present => write!(f, r#"[{attr}]"#),
                        SelectorAttributeType::ExactMatch(val) => {
                            write!(f, r#"[{attr} = "{val}"]"#)
                        }
                        SelectorAttributeType::ListContains(val) => {
                            write!(f, r#"[{attr} ~= "{val}"]"#)
                        }
                        SelectorAttributeType::StartsWith(val) => {
                            write!(f, r#"[{attr} ^= "{val}"]"#)
                        }
                        SelectorAttributeType::StartsWithDashed(val) => {
                            write!(f, r#"[{attr} |= "{val}"]"#)
                        }
                        SelectorAttributeType::Endswith(val) => write!(f, r#"[{attr} $= "{val}"]"#),
                        SelectorAttributeType::RawContains(val) => {
                            write!(f, r#"[{attr} *= "{val}"]"#)
                        }
                    };
                } else {
                    match op {
                        SelectorAttributeType::Present => write!(f, r#"[{attr} s]"#),
                        SelectorAttributeType::ExactMatch(val) => {
                            write!(f, r#"[{attr} = "{val}" s]"#)
                        }
                        SelectorAttributeType::ListContains(val) => {
                            write!(f, r#"[{attr} ~= "{val}" s]"#)
                        }
                        SelectorAttributeType::StartsWith(val) => {
                            write!(f, r#"[{attr} ^= "{val}" s]"#)
                        }
                        SelectorAttributeType::StartsWithDashed(val) => {
                            write!(f, r#"[{attr} |= "{val}" s]"#)
                        }
                        SelectorAttributeType::Endswith(val) => {
                            write!(f, r#"[{attr} $= "{val}" s]"#)
                        }
                        SelectorAttributeType::RawContains(val) => {
                            write!(f, r#"[{attr} *= "{val}" s]"#)
                        }
                    };
                }
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
