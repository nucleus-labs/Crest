//! `XTokenTracker`s are utilities for managing and parsing a sequence of `ParserToken`s.
//!
//! This struct provides two primary features:
//! 1. A mechanism to "fake" token pops without mutating the underlying token vector, enabling
//!    references to popped tokens to outlive the scope of the popping function.
//! 2. Convenient parsing functions (`expect_*`) to validate and extract specific token types,
//!    returning descriptive errors (`ExpectError`) on failure.
//!
//! Token popping is achieved by maintaining an internal offset index. Each pop operation
//! increments this index, effectively "consuming" tokens while preserving the underlying
//! vector's immutability. Once a token is "consumed," it cannot be revisited.
//!
//! # Behavior
//! - Tokens are "consumed" in order, starting from the current offset.
//! - If an expectation fails, the offset remains unchanged.
//! - Tokens retain their ownership within `CssTokenTracker`, ensuring their lifetime matches
//!   that of the tracker.
//!
//! # Example Usage
//! ```rust
//! let tokens = vec![CssToken::Identifier("example".into())];
//! let tracker = CssTokenTracker::new(Boo::Owned(tokens));
//!
//! assert_eq!(tracker.len(), 1);
//! if let Ok(identifier) = tracker.expect_identifier() {
//!     assert_eq!(identifier.as_str(), "example");
//! }
//! assert_eq!(tracker.len(), 0); // Token has been "consumed".
//! ```

use std::cell::Cell;
use std::{collections::HashMap, sync::Arc};

use super::{
    boo::Boo, parse_selector, CssToken, SelectorParser, SelectorRule, SelectorToken, SourceSlice,
};
use crate::{
    parse::CssRule,
    style::parse_attr::{
        size::{CssHeight, CssWidth},
        types::{CssPadding, CssSpacing},
        CssAttributeValue, CssStyleAttribute, CssValue,
    },
};
use crate::{unit::*, SelectorNode};

#[derive(Debug, Clone)]
pub struct CssTokenTracker<'a> {
    /// A `Boo` wrapper around a `Vec<CssToken>`. Neither `Boo` nor `CssTokenTracker` guarantee
    /// that the contained vec is owned by a `CssTokenTracker`, but the design of `CssTokenTracker`
    /// assumes that any `boo` field adheres to this contract.
    ///
    /// The `boo` field enables the use of either owned or borrowed tokens, reducing
    /// redundancy in parsing implementations. It is essential that this field is only
    /// initialized within `CssTokenTracker` or similar contexts that maintain this contract.
    ///
    /// ## Notes:
    /// - Any modifications to the `boo` field should only occur within the
    ///   `CssTokenTracker` implementation to ensure the contract remains valid.
    /// - This abstraction avoids the need for separate types (e.g., `TokenTrackerBorrowed`)
    ///   by enabling flexible token ownership semantics.
    ///
    /// Misuse of the `boo` field outside its intended context can lead to undefined behavior
    /// in parsing operations. Use with caution and respect the ownership semantics.
    boo: Boo<'a, Vec<CssToken>>,

    idx: Cell<usize>,
}

#[derive(Debug, Clone)]
pub struct SelectorTokenTracker<'a> {
    /// A `Boo` wrapper around a `Vec<SelectorToken>`. Neither `Boo` nor `SelectorTokenTracker`
    /// guarantee that the contained vec is owned by a `SelectorTokenTracker`, but the design of
    /// `SelectorTokenTracker` assumes that any `boo` field adheres to this contract.
    ///
    /// The `boo` field enables the use of either owned or borrowed tokens, reducing
    /// redundancy in parsing implementations. It is essential that this field is only
    /// initialized within `SelectorTokenTracker` or similar contexts that maintain this contract.
    ///
    /// ## Notes:
    /// - Any modifications to the `boo` field should only occur within the
    ///   `SelectorTokenTracker` implementation to ensure the contract remains valid.
    /// - This abstraction avoids the need for separate types (e.g., `TokenTrackerBorrowed`)
    ///   by enabling flexible token ownership semantics.
    ///
    /// Misuse of the `boo` field outside its intended context can lead to undefined behavior
    /// in parsing operations. Use with caution and respect the ownership semantics.
    boo: Boo<'a, Vec<SelectorToken>>,

    idx: Cell<usize>,
}

#[derive(Clone, Debug)]
pub struct CssStyleValueExpector<'a> {
    css_expector: &'a CssTokenTracker<'a>,
    has_errored: bool,
    results: Vec<Result<CssStyleAttribute, ExpectError>>,
}

#[derive(Clone, Debug)]
pub struct CssAtRuleValueExpector<'a> {
    css_expector: &'a CssTokenTracker<'a>,
    has_errored: bool,
    results: Vec<Result<CssStyleAttribute, ExpectError>>,
}

/// A placeholder error type for token expectation failures.
#[derive(Debug, Clone)]
pub enum ExpectError {
    TooFewTokens(String),

    CssFailedExpectation(CssRule, CssRule),
    CssInvalidSelector(SourceSlice),
    CssInvalidStyleAttrName(SourceSlice),
    CssInvalidAtRuleName(SourceSlice),

    SelectorFailedExpectation(SelectorRule, SelectorRule),

    Generic(String),
}

impl<'a> CssTokenTracker<'a> {
    /// Creates a new `CssTokenTracker` from a `Boo` wrapping a vector of `CssToken`s.
    pub fn new(tokens_boo: Boo<'a, Vec<CssToken>>) -> Self {
        Self {
            boo: tokens_boo,
            idx: Cell::new(0),
        }
    }

    /// Retrieves a reference to the next token in the vector without consuming it.
    ///
    /// This method allows peeking at the current token for introspection, such as determining
    /// its type or length of child tokens, without advancing the offset index.
    #[inline]
    pub fn peek(&self) -> Option<&CssToken> {
        self.boo.get(self.idx.get())
    }

    #[inline]
    fn fail_because<T>(&self, error: ExpectError) -> Result<T, ExpectError> {
        self.idx.set(0.max(self.idx.get() - 1));
        Err(error)
    }

    #[inline]
    fn fail_type<T>(&self, expected: CssRule) -> Result<T, ExpectError> {
        let current_rule = self.boo.get(0.max(self.idx.get() - 1)).unwrap().get_rule();
        self.fail_because(ExpectError::CssFailedExpectation(current_rule, expected))
    }

    /// Consumes the next token and returns a reference to it, or `None` if no tokens remain.
    ///
    /// This increments the internal offset index, marking the token as consumed.
    fn pop_front(&self) -> Option<&CssToken> {
        let result = self.boo.get(self.idx.get());
        if result.is_some() {
            self.idx.set(self.idx.get() + 1);
        }
        result
    }

    /// Consumes and returns references to the next `count` tokens, or `None` if fewer tokens
    /// remain than requested.
    ///
    /// This method advances the offset index by `count` if successful.
    fn pop_front_count(&self, count: usize) -> Option<Box<[&CssToken]>> {
        if self.boo.len() < self.idx.get() + count {
            return None;
        }

        let mut tokens = Vec::new();
        tokens.reserve(count);
        for i in 0..count {
            tokens.push(self.boo.get(self.idx.get() + i).unwrap());
        }
        self.idx.set(self.idx.get() + count);
        Some(tokens.into_boxed_slice())
    }

    /// Returns the number of unconsumed tokens remaining in the vector.
    #[inline]
    pub fn len(&self) -> usize {
        self.boo.len() - self.idx.get()
    }

    #[inline]
    fn read_component_values(component_value_list: &CssToken) -> Box<[&CssToken]> {
        let values = component_value_list.get_children().unwrap();
        let value_refs = values.iter().collect::<Vec<_>>();
        value_refs.into_boxed_slice()
    }

    fn parse_style_attr(
        attr_name: SourceSlice,
        component_value_list: &'a CssToken,
    ) -> Result<Box<[CssStyleAttribute]>, ExpectError> {
        let component_values = component_value_list.get_children().unwrap();
        let css_expector: CssTokenTracker<'a> = Self::new(Boo::Borrowed(component_values));
        match attr_name.get() {
            "width" => CssStyleValueExpector::new(&css_expector)
                .expect::<CssWidth>()
                .resolve(),

            "height" => CssStyleValueExpector::new(&css_expector)
                .expect::<CssHeight>()
                .resolve(),

            "padding" => CssStyleValueExpector::new(&css_expector)
                .expect::<CssPadding>()
                .expect::<CssPadding>()
                .expect::<CssPadding>()
                .expect::<CssPadding>()
                .resolve(),

            "spacing" => CssStyleValueExpector::new(&css_expector)
                .expect::<CssSpacing>()
                .resolve(),

            "font-family" => todo!(),
            "font-size" => todo!(),
            "font-shaping" => todo!(),

            "line-height" => todo!(),

            "text-wrap" => todo!(),

            "max-width" => todo!(),
            "max-height" => todo!(),

            "justify-content" => todo!(),
            "vertical-align" => todo!(),

            "overflow" => todo!(),

            "margin" => todo!(),

            "left" => todo!(),
            "right" => todo!(),
            "top" => todo!(),
            "bottom" => todo!(),

            "object-fit" => todo!(),

            "image-rendering" => todo!(),
            "rotation" => todo!(),
            "opacity" => todo!(),

            _ => Err(ExpectError::CssInvalidStyleAttrName(attr_name)),
        }
    }

    fn parse_at_rule_attr(
        &self,
        attr_name: SourceSlice,
        component_value_list: &'a CssToken,
    ) -> Result<Box<[CssStyleAttribute]>, ExpectError> {
        let css_expector: CssTokenTracker<'a> =
            Self::new(Boo::Borrowed(&component_value_list.get_children().unwrap()));
        match attr_name.get() {
            _ => Err(ExpectError::CssInvalidAtRuleName(attr_name)),
        }
    }

    pub fn expect_css_stylesheet(&'a self) -> Result<Self, ExpectError> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(ExpectError::TooFewTokens("STYLESHEET".into()))?;

        if matches!(token.get_rule(), CssRule::CSS) {
            let token_children = token.get_children().unwrap();
            let sheet_children = token_children[0].get_children().unwrap();

            let boo = Boo::Borrowed(sheet_children);
            return Ok(Self::new(boo));
        }

        self.fail_type(CssRule::CSS)
    }

    pub fn expect_at_rule(&self) -> Result<crate::style::AtRule, ExpectError> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(ExpectError::TooFewTokens("AT_RULE".into()))?;

        if matches!(token.get_rule(), CssRule::AT_RULE) {
            let name = token.get_children().unwrap()[0].get_children().unwrap()[0]
                .get_source()
                .get()
                .to_string();
            return Ok(crate::style::AtRule { name });
        }

        self.fail_type(CssRule::AT_RULE)
    }

    pub fn expect_style_rule(
        &'a self,
    ) -> Result<(Box<[crate::SelectorNode]>, crate::style::CssStyleProperties), ExpectError> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(ExpectError::TooFewTokens("QUALIFIED_RULE".into()))?;

        if matches!(token.get_rule(), CssRule::QUALIFIED_RULE) {
            // Self: QUALIFIED_RULE = { WS* ~ SELECTOR ~ WS* ~ DECL_BLOCK }
            let components = token.get_children().unwrap();

            // QUALIFIED_RULE -> SELECTOR
            let selector_slice: SourceSlice = components[0].get_source();
            let selector_parser = parse_selector(selector_slice.get()).unwrap();
            let selector_expector = SelectorTokenTracker::new(Boo::Owned(vec![selector_parser]));

            if let Ok(selectors) = selector_expector.expect_selector() {
                let mut sheet = crate::style::properties::CssStyleProperties::default();

                // QUALIFIED_RULE -> DECL_BLOCK -> DECLARATION*
                let declarations = components[1].get_children().unwrap();
                for declaration_token in declarations.iter() {
                    if !matches!(declaration_token.get_rule(), CssRule::DECLARATION) {
                        return self.fail_because(ExpectError::CssFailedExpectation(
                            declaration_token.get_rule(),
                            CssRule::DECLARATION,
                        ));
                    }

                    // QUALIFIED_RULE -> DECL_BLOCK -> DECLARATION
                    let decl_components = declaration_token.get_children().unwrap();

                    // QUALIFIED_RULE -> DECL_BLOCK -> DECLARATION -> IDENT [ source ]
                    let decl_name = decl_components[0].get_source();

                    // we ignore IMPORTANT for now

                    // QUALIFIED_RULE -> DECL_BLOCK -> DECLARATION -> COMPONENT_VALUE_LIST
                    let decl_values =
                        Self::parse_style_attr(decl_name, &decl_components[1]).unwrap();

                    sheet.update(decl_values);
                }
                return Ok((selectors, sheet));
            } else {
                return self.fail_because(ExpectError::CssInvalidSelector(selector_slice));
            }
        }

        self.fail_type(CssRule::QUALIFIED_RULE)
    }

    /// Consumes the next token and extracts its `SourceSlice` representation if it is a valid
    /// identifier. Returns an `ExpectError` otherwise.
    pub fn expect_identifier(&self) -> Result<SourceSlice, ExpectError> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(ExpectError::TooFewTokens("IDENT".into()))?;

        if matches!(token.get_rule(), CssRule::IDENT) {
            return Ok(token.get_source());
        }

        self.fail_type(CssRule::IDENT)
    }

    /// Consumes the next token and extracts its `f32` value if it is a valid number token.
    /// Returns an `ExpectError` otherwise.
    pub fn expect_number(&self) -> Result<f32, ExpectError> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(ExpectError::TooFewTokens("NUMBER".into()))?;

        if matches!(token.get_rule(), CssRule::NUMBER) {
            if let Ok(num) = token.get_source().parse::<f32>() {
                return Ok(num);
            }
        }

        self.fail_type(CssRule::NUMBER)
    }

    /// Consumes the next token and extracts its `f32` value if it is a valid percentage token.
    /// Returns an `ExpectError` otherwise.
    pub fn expect_percentage(&self) -> Result<f32, ExpectError> {
        let tokens: Box<[&CssToken]> = self
            .pop_front_count(2)
            .ok_or(ExpectError::TooFewTokens("PERCENTAGE".into()))?;

        if matches!(tokens[0].get_rule(), CssRule::PERCENTAGE) {
            let number = tokens[1];
            if let Ok(num) = number.get_source().parse::<f32>() {
                return Ok(num);
            }
        }

        self.fail_type(CssRule::PERCENTAGE)
    }

    /// Consumes the next token and extracts its `Dimension` representation if it is a valid
    /// dimension token. Returns an `ExpectError` otherwise.
    pub fn expect_dimension(&self) -> Result<Dimension, ExpectError> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(ExpectError::TooFewTokens("DIMENSION".into()))?;

        if matches!(token.get_rule(), CssRule::DIMENSION) {
            let components = token
                .get_children()
                .ok_or(ExpectError::TooFewTokens("DIMENSION".into()))?;
            if matches!(
                (components[0].get_rule(), components[1].get_rule()),
                (CssRule::NUMBER, CssRule::IDENT)
            ) {
                if let Ok(number) = components[0].get_source().parse::<f32>() {
                    let unit_slice = components[1].get_source();
                    let unit = unit_slice.get();
                    return Ok(Dimension::from_pair(number, &unit)
                        .expect(&format!("Failed to parse Dimension from '{number}{unit}'")));
                }
            }
        }

        self.fail_type(CssRule::DIMENSION)
    }

    /// Consumes the next token and extracts its `SourceSlice` representation if it is a valid quoted
    /// string token. Excludes surrounding quotes from the result. Returns an `ExpectError` otherwise.
    pub fn expect_quoted_string(&self) -> Result<SourceSlice, ExpectError> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(ExpectError::TooFewTokens("QUOTED_STRING".into()))?;

        if matches!(token.get_rule(), CssRule::STRING) {
            return Ok(token.get_source());
        }

        self.fail_type(CssRule::STRING)
    }

    /// Consumes the next token and extracts its unquoted URL value if it is a valid `url(...)`
    /// token. Surrounding syntax (e.g., `url(` and `)`) is excluded from the result. Returns
    /// an `ExpectError` otherwise.
    pub fn expect_url(&self) -> Result<SourceSlice, ExpectError> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(ExpectError::TooFewTokens("URL".into()))?;

        if matches!(token.get_rule(), CssRule::URL) {
            let url_contents = &token.get_children().unwrap()[0];
            return Ok(url_contents.get_source());
        }

        self.fail_type(CssRule::URL)
    }

    /// Consumes the next token and extracts its `<color>` representation if it is a valid
    /// hash token. Returns an `(r, g, b, a)` tuple or an `ExpectError` otherwise.
    pub fn expect_hash(&self) -> Result<(u8, u8, u8, u8), ExpectError> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(ExpectError::TooFewTokens("HASH".into()))?;

        if matches!(token.get_rule(), CssRule::HASH) {
            let source = token.get_source();

            // direct indexing into strs are safe for rgb because a minimum of 6 hex characters are
            // required for a Hash token, as defined in the PEG.
            let r = u8::from_str_radix(&source[1..3], 16).unwrap();
            let g = u8::from_str_radix(&source[3..5], 16).unwrap();
            let b = u8::from_str_radix(&source[5..7], 16).unwrap();
            let a: u8 = if let Some(val) = (*source).get(7..9) {
                u8::from_str_radix(val, 16).unwrap()
            } else {
                0u8
            };
            return Ok((r, g, b, a));
        }

        self.fail_type(CssRule::HASH)
    }

    /// Consumes the next token and extracts its function identifier along with a `CssTokenTracker`
    /// for its arguments if it is a valid function token. Returns an `ExpectError` otherwise.
    pub fn expect_function(&self) -> Result<(SourceSlice, CssTokenTracker), ExpectError> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(ExpectError::TooFewTokens("FUNCTION_BLOCK".into()))?;

        if matches!(token.get_rule(), CssRule::FUNCTION_BLOCK) {
            let components = token
                .get_children()
                .expect("Tried to parse url from too few tokens!");
            let boo: Boo<Vec<CssToken>> = Boo::Borrowed(components);
            let slice = CssTokenTracker::new(boo);
            let name = slice.expect_identifier().unwrap();
            return Ok((name, slice));
        }

        self.fail_type(CssRule::FUNCTION_BLOCK)
    }
}

impl<'a> SelectorTokenTracker<'a> {
    /// Creates a new `SelectorTokenTracker` from a `Boo` wrapping a vector of `SelectorToken`s.
    pub fn new(tokens_boo: Boo<'a, Vec<SelectorToken>>) -> Self {
        Self {
            boo: tokens_boo,
            idx: Cell::new(0),
        }
    }

    /// Retrieves a reference to the next token in the vector without consuming it.
    ///
    /// This method allows peeking at the current token for introspection, such as determining
    /// its type or length of child tokens, without advancing the offset index.
    #[inline]
    pub fn peek(&self) -> Option<&SelectorToken> {
        self.boo.get(self.idx.get())
    }

    #[inline]
    fn fail_because<T>(&self, error: ExpectError) -> Result<T, ExpectError> {
        self.idx.set(0.max(self.idx.get() - 1));
        Err(error)
    }

    #[inline]
    fn fail_type<T>(&self, expected: SelectorRule) -> Result<T, ExpectError> {
        let current_rule = self.boo.get(0.max(self.idx.get() - 1)).unwrap().get_rule();
        self.fail_because(ExpectError::SelectorFailedExpectation(
            current_rule,
            expected,
        ))
    }

    /// Consumes the next token and returns a reference to it, or `None` if no tokens remain.
    ///
    /// This increments the internal offset index, marking the token as consumed.
    fn pop_front(&self) -> Option<&SelectorToken> {
        let result = self.boo.get(self.idx.get());
        if result.is_some() {
            self.idx.set(self.idx.get() + 1);
        }
        result
    }

    /// Consumes and returns references to the next `count` tokens, or `None` if fewer tokens
    /// remain than requested.
    ///
    /// This method advances the offset index by `count` if successful.
    fn pop_front_count(&self, count: usize) -> Option<Box<[&SelectorToken]>> {
        if self.boo.len() < self.idx.get() + count {
            return None;
        }

        let mut tokens = Vec::new();
        tokens.reserve(count);
        for i in 0..count {
            tokens.push(self.boo.get(self.idx.get() + i).unwrap());
        }
        self.idx.set(self.idx.get() + count);
        Some(tokens.into_boxed_slice())
    }

    /// Returns the number of unconsumed tokens remaining in the vector.
    #[inline]
    pub fn len(&self) -> usize {
        self.boo.len() - self.idx.get()
    }

    /// Consumes the next token, constructs a `Box<[crate::SelectorNode]>` representation
    /// if it is a valid selector token. Returns an `ExpectError` otherwise.
    pub fn expect_selector(&'a self) -> Result<Box<[crate::SelectorNode]>, ExpectError> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(ExpectError::TooFewTokens("SELECTOR".into()))?;

        if matches!(token.get_rule(), SelectorRule::SELECTOR) {
            let components = token.get_children().unwrap();
            let compound_expector = Self::new(Boo::Borrowed(components));

            let mut selectors: Vec<SelectorNode> = Vec::new();
            // SELF: SELECTOR -> SELECTOR_COMPOUND ->
            let complex_expector = compound_expector.expect_compound()?;

            while let Ok(complex_expector) = complex_expector.expect_complex() {
                let mut selector = SelectorNode::default();
                // todo
                selectors.push(selector);
            }

            return Ok(selectors.into_boxed_slice());
        }

        self.fail_type(SelectorRule::SELECTOR)
    }

    pub fn expect_compound(&'a self) -> Result<Self, ExpectError> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(ExpectError::TooFewTokens("SELECTOR_COMPOUND".into()))?;

        if matches!(token.get_rule(), SelectorRule::SELECTOR_COMPOUND) {
            let complex_expector = Self::new(Boo::Borrowed(token.get_children().unwrap()));
            return Ok(complex_expector);
        }

        self.fail_type(SelectorRule::SELECTOR_COMPOUND)
    }

    pub fn expect_complex(&self) -> Result<Self, ExpectError> {
        let token: &SelectorToken = self
            .pop_front()
            .ok_or(ExpectError::TooFewTokens("SELECTOR_COMPOUND".into()))?;

        if matches!(token.get_rule(), SelectorRule::SELECTOR_COMPLEX) {}

        self.fail_type(SelectorRule::SELECTOR_COMPLEX)
    }
}

impl<'a> CssStyleValueExpector<'a> {
    fn new(css_expector: &'a CssTokenTracker) -> Self {
        Self {
            css_expector,
            has_errored: false,
            results: Vec::new(),
        }
    }

    fn expect<T: CssValue>(&mut self) -> &mut Self
    where
        CssStyleAttribute: From<CssAttributeValue<T>>,
    {
        if self.has_errored {
            return self;
        }

        match T::parse(self.css_expector) {
            Ok(attr_val) => self.results.push(Ok(attr_val.into())),
            Err(err) => {
                self.has_errored = true;
                self.results.push(Err(ExpectError::Generic(err)));
            }
        }

        self
    }

    fn resolve(&mut self) -> Result<Box<[CssStyleAttribute]>, ExpectError> {
        if self.has_errored {
            Err(self.results.pop().unwrap().unwrap_err())
        } else {
            let mut results = Vec::new();
            results.append(&mut self.results);

            let mut attrs: Vec<CssStyleAttribute> =
                results.into_iter().map(|x| x.unwrap()).collect::<Vec<_>>();

            Ok(attrs.into_boxed_slice())
        }
    }
}

impl<'a> CssAtRuleValueExpector<'a> {
    fn new(css_expector: &'a CssTokenTracker) -> Self {
        Self {
            css_expector,
            has_errored: false,
            results: Vec::new(),
        }
    }

    fn expect<T: CssValue>(&mut self) -> &mut Self
    where
        CssStyleAttribute: From<CssAttributeValue<T>>,
    {
        if self.has_errored {
            return self;
        }

        match T::parse(self.css_expector) {
            Ok(attr_val) => self.results.push(Ok(attr_val.into())),
            Err(err) => {
                self.has_errored = true;
                let expect_err = ExpectError::Generic(format!(
                    "Failed to parse type {} from '{err}'",
                    T::type_name()
                ));
                self.results.push(Err(expect_err));
            }
        }

        self
    }

    fn resolve(&mut self) -> Result<Box<[CssStyleAttribute]>, ExpectError> {
        if self.has_errored {
            Err(self.results.pop().unwrap().unwrap_err())
        } else {
            let mut results = Vec::new();
            results.append(&mut self.results);

            let mut attrs: Vec<CssStyleAttribute> =
                results.into_iter().map(|x| x.unwrap()).collect::<Vec<_>>();

            Ok(attrs.into_boxed_slice())
        }
    }
}
