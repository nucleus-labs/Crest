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
use crate::source::{parse_source, ParserToken, SourceInfo, SourceSlice, StackInfo, TokenTracker};
use crate::syntax::CssToken;
use crate::Unit;

pub(crate) type SelectorToken = ParserToken<SelectorRule>;
pub(crate) type SelectorStackInfo = StackInfo<SelectorRule>;

#[derive(Debug, Clone)]
pub enum SelectorNodeType {
    Namespace(String),
    Universal,
    TypeName(String),
    Id(String),
    Class(String),
    Attribute {
        name: String,
        attr_matcher: SelectorAttributeType,
        case_sensitive: bool,
    },
    NextSibling(Box<SelectorNode>),
    SubsequentSibling(Box<SelectorNode>),
    Child(Box<SelectorNode>),
    Descendent(Box<SelectorNode>),
    PseudoClass(String),
    PseudoElement(String, Option<Box<[Unit]>>),
}

#[derive(Debug, Clone, Default)]
pub struct SelectorNode(Vec<SelectorNodeType>);

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

        while let Ok((mut first, mut rest)) = list_expector.expect_complex() {
            while rest.len() > 1 {
                let combinator = rest.pop().unwrap();
                let prev = rest.last_mut().unwrap().get_node();
                match combinator {
                    SelectorCombinator::SubsequentSibling(selector_node) => {
                        prev.push(SelectorNodeType::SubsequentSibling(Box::new(selector_node)))
                    }
                    SelectorCombinator::NextSibling(selector_node) => {
                        prev.push(SelectorNodeType::NextSibling(Box::new(selector_node)))
                    }
                    SelectorCombinator::Descendent(selector_node) => {
                        prev.push(SelectorNodeType::Descendent(Box::new(selector_node)))
                    }
                    SelectorCombinator::Column(selector_node) => todo!(),
                    SelectorCombinator::Child(selector_node) => {
                        prev.push(SelectorNodeType::Child(Box::new(selector_node)))
                    }
                }
            }

            if !rest.is_empty() {
                let combinator = rest.pop().unwrap();
                match combinator {
                    SelectorCombinator::SubsequentSibling(selector_node) => {
                        first.push(SelectorNodeType::SubsequentSibling(Box::new(selector_node)))
                    }
                    SelectorCombinator::NextSibling(selector_node) => {
                        first.push(SelectorNodeType::NextSibling(Box::new(selector_node)))
                    }
                    SelectorCombinator::Descendent(selector_node) => {
                        first.push(SelectorNodeType::Descendent(Box::new(selector_node)))
                    }
                    SelectorCombinator::Column(selector_node) => todo!(),
                    SelectorCombinator::Child(selector_node) => {
                        first.push(SelectorNodeType::Child(Box::new(selector_node)))
                    }
                }
            }

            selectors.push(first);
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

        while let Some(peek) = complex_expector.peek() {
            match peek.get_rule() {
                SelectorRule::SELECTOR_COMBINATOR => {
                    let combinator_token = complex_expector.pop_front().unwrap();

                    let next = complex_expector.expect_compound()?;

                    let combinator_components = combinator_token.get_children().unwrap();
                    let combinator: SelectorCombinator = match combinator_components[0].get_rule() {
                        SelectorRule::SELECTOR_COMBINATOR__NEXT_SIBLING => {
                            SelectorCombinator::NextSibling(next)
                        }
                        SelectorRule::SELECTOR_COMBINATOR__CHILD => SelectorCombinator::Child(next),
                        SelectorRule::SELECTOR_COMBINATOR__COLUMN => {
                            SelectorCombinator::Column(next)
                        }
                        SelectorRule::SELECTOR_COMBINATOR__SUBSEQUENT_SIBLING => {
                            SelectorCombinator::SubsequentSibling(next)
                        }
                        SelectorRule::SELECTOR_COMBINATOR__DESCENDENT => {
                            SelectorCombinator::Descendent(next)
                        }

                        _ => unreachable!(),
                    };

                    rest.push(combinator);
                }
                SelectorRule::SELECTOR_COMBINATOR__NAMESPACE => {
                    let namespace_token = complex_expector.pop_front().unwrap();
                    let node =
                        SelectorNodeType::Namespace(namespace_token.get_source().to_string());
                    rest.last_mut().unwrap().get_node().push(node);
                }

                _ => unreachable!(),
            }
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

        let compound_expector = Self::new(token).unwrap();
        let mut node = SelectorNode::default();

        if let Ok((namespace_opt, type_name_opt)) = compound_expector.expect_type() {
            if namespace_opt.is_none() && type_name_opt.is_none() {
                node.push(SelectorNodeType::Universal);
            } else {
                if let Some(namespace) = namespace_opt {
                    node.push(SelectorNodeType::Namespace(namespace.to_string()));
                }
                if let Some(type_name) = type_name_opt {
                    node.push(SelectorNodeType::TypeName(type_name.to_string()));
                }
            }
        }

        while let Ok(subclass) = compound_expector.expect_subclass() {
            match subclass {
                SelectorSubclassType::Id(id) => node.push(SelectorNodeType::Id(id)),
                SelectorSubclassType::Class(class) => node.push(SelectorNodeType::Class(class)),
                SelectorSubclassType::Attribute {
                    name,
                    attr_matcher,
                    case_sensitive,
                } => node.push(SelectorNodeType::Attribute {
                    name,
                    attr_matcher,
                    case_sensitive,
                }),
                SelectorSubclassType::PseudoClass(psclass) => {
                    node.push(SelectorNodeType::PseudoClass(psclass))
                }
                SelectorSubclassType::PseudoElement(pselement, params) => {
                    node.push(SelectorNodeType::PseudoElement(pselement, params))
                }
            }
        }

        Ok(node)
    }

    fn expect_subclass(&'a self) -> SelectorResult<SelectorSubclassType> {
        let token: &SelectorToken = self.pop_front().ok_or(SelectorExpectError::TooFewTokens(
            "SELECTOR_SUBCLASS".into(),
        ))?;

        if !matches!(token.get_rule(), SelectorRule::SELECTOR_SUBCLASS) {
            return self.fail_type(SelectorRule::SELECTOR_SUBCLASS);
        }

        let subclass_expector = Self::new(token).unwrap();
        match subclass_expector.peek().unwrap().get_rule() {
            SelectorRule::SELECTOR_ID => {
                let ident = subclass_expector.expect_id()?;

                Ok(SelectorSubclassType::Id(ident.to_string()))
            }
            SelectorRule::SELECTOR_CLASS => {
                let ident = subclass_expector.expect_class()?;

                Ok(SelectorSubclassType::Class(ident.to_string()))
            }
            SelectorRule::SELECTOR_ATTRIBUTE => {
                let (name, attr_matcher, case_sensitive) = subclass_expector.expect_attribute()?;

                Ok(SelectorSubclassType::Attribute {
                    name: name.to_string(),
                    attr_matcher,
                    case_sensitive,
                })
            }
            SelectorRule::SELECTOR_PSEUDOCLASS => {
                let psclass = subclass_expector.expect_pseudoclass()?;

                Ok(SelectorSubclassType::PseudoClass(psclass.to_string()))
            }

            _ => unreachable!(),
        }
    }

    fn expect_pseudoclass(&'a self) -> SelectorResult<SourceSlice> {
        let token: &SelectorToken = self.pop_front().ok_or(SelectorExpectError::TooFewTokens(
            "SELECTOR_PSEUDOCLASS".into(),
        ))?;

        if !matches!(token.get_rule(), SelectorRule::SELECTOR_PSEUDOCLASS) {
            return self.fail_type(SelectorRule::SELECTOR_PSEUDOCLASS);
        }

        let psclass_expector = Self::new(token).unwrap();
        match psclass_expector.peek().unwrap().get_rule() {
            SelectorRule::IDENT => {
                let ident = psclass_expector.expect_identifier()?;
                Ok(ident)
            }
            SelectorRule::FUNCTION => todo!("Functions are not currently supported"),

            _ => unreachable!(),
        }
    }

    fn expect_id(&'a self) -> SelectorResult<SourceSlice> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(SelectorExpectError::TooFewTokens("SELECTOR_ID".into()))?;

        if !matches!(token.get_rule(), SelectorRule::SELECTOR_ID) {
            return self.fail_type(SelectorRule::SELECTOR_ID);
        }

        let id_expector = Self::new(token).unwrap();

        id_expector.expect_hash()
    }

    /// Consumes the next token and extracts its `SourceSlice` representation if it is a
    /// valid hash token. Returns a `CssExpectError` otherwise.
    pub fn expect_hash(&self) -> SelectorResult<SourceSlice> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(SelectorExpectError::TooFewTokens("HASH".into()))?;

        if !matches!(token.get_rule(), SelectorRule::HASH) {
            return self.fail_type(SelectorRule::HASH);
        }

        let mut slice = token.get_source();
        slice.start = slice.source_info.location_from_idx(slice.start.idx + 1);

        Ok(slice)
    }

    pub fn expect_class(&'a self) -> SelectorResult<SourceSlice> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(SelectorExpectError::TooFewTokens("SELECTOR_CLASS".into()))?;

        if !matches!(token.get_rule(), SelectorRule::SELECTOR_CLASS) {
            return self.fail_type(SelectorRule::SELECTOR_CLASS);
        }

        let class_expector = Self::new(token).unwrap();

        class_expector.expect_identifier()
    }

    fn expect_attribute(&'a self) -> SelectorResult<(SourceSlice, SelectorAttributeType, bool)> {
        let token: &SelectorToken = self.pop_front().ok_or(SelectorExpectError::TooFewTokens(
            "SELECTOR_ATTRIBUTE".into(),
        ))?;

        if !matches!(token.get_rule(), SelectorRule::SELECTOR_ATTRIBUTE) {
            return self.fail_type(SelectorRule::SELECTOR_ATTRIBUTE);
        }

        let attribute_expector = Self::new(token).unwrap();

        let attr_name = attribute_expector.expect_identifier()?;
        let matcher_result = attribute_expector
            .expect_attr_matcher()
            .unwrap_or(SelectorAttributeType::Present);
        let sens = attribute_expector.expect_attr_modifier().unwrap_or(false);

        Ok((attr_name, matcher_result, sens))
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

impl SelectorNode {
    pub fn from_source(source_info: SourceInfo) -> Result<Box<[Self]>, SelectorExpectError> {
        let selector_parser_result = parse_source::<SelectorRule, SelectorParser>(
            source_info.into(),
            SelectorRule::SELECTOR_LIST,
        );

        let selector_parser =
            selector_parser_result.map_err(|err| SelectorExpectError::ParseError(err))?;
        let selector_expector = SelectorTokenTracker::new(&selector_parser).unwrap();

        Ok(selector_expector.expect_selector_list()?)
    }
}

impl std::ops::Deref for SelectorNode {
    type Target = Vec<SelectorNodeType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SelectorNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for SelectorNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in self.iter() {
            match node {
                SelectorNodeType::Universal => write!(f, "*"),
                SelectorNodeType::Namespace(namespace) => write!(f, "|{namespace}"),
                SelectorNodeType::TypeName(type_name) => write!(f, "{type_name}"),
                SelectorNodeType::Id(id) => write!(f, "#{id}"),
                SelectorNodeType::Class(class) => write!(f, ".{class}"),
                SelectorNodeType::Attribute {
                    name,
                    attr_matcher,
                    case_sensitive,
                } => {
                    write!(f, "[{name}");
                    match attr_matcher {
                        SelectorAttributeType::ExactMatch(attr) => write!(f, r#"="{attr}""#),
                        SelectorAttributeType::ListContains(attr) => write!(f, r#"~="{attr}""#),
                        SelectorAttributeType::StartsWith(attr) => write!(f, r#"^="{attr}""#),
                        SelectorAttributeType::StartsWithDashed(attr) => write!(f, r#"|="{attr}""#),
                        SelectorAttributeType::Endswith(attr) => write!(f, r#"$="{attr}""#),
                        SelectorAttributeType::RawContains(attr) => write!(f, r#"*="{attr}""#),

                        SelectorAttributeType::Present => Ok(()),
                    };
                    write!(f, "]")
                }
                SelectorNodeType::NextSibling(sibling) => write!(f, " + {sibling}"),
                SelectorNodeType::SubsequentSibling(sibling) => write!(f, " ~ {sibling}"),
                SelectorNodeType::Child(child) => write!(f, " > {child}"),
                SelectorNodeType::Descendent(descendent) => write!(f, " {descendent}"),
                SelectorNodeType::PseudoClass(psclass) => write!(f, ":{psclass}"),
                SelectorNodeType::PseudoElement(pselement, params_opt) => {
                    write!(f, "::{pselement}");
                    if let Some(params) = params_opt {
                        let list: String = params
                            .as_ref()
                            .iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<_>>()
                            .join(", ");
                        write!(f, "({list})");
                    }
                    Ok(())
                }
            };
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
