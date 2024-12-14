pub(crate) mod pest_css;

pub use pest_css::CssParser;
pub use pest_css::Rule as CssRule;

use std::cell::Cell;
use std::collections::HashMap;
use std::num::ParseFloatError;
use std::sync::Arc;

use crate::boo::Boo;
use crate::selector::{
    self, SelectorExpectError, SelectorNode, SelectorParser, SelectorRule, SelectorTokenTracker,
};
use crate::source::{parse_source, SourceSlice, StackInfo, TokenTracker};
use crate::style::{
    parse_attr::{types as ValueTypes, CssAttributeValue, CssStyleAttribute, CssValue},
    AtRule, CssStyleProperties, CssStyleValueExpector, Stylesheet,
};
use crate::unit::Dimension;

pub(crate) type CssStackInfo = StackInfo<CssRule>;
pub(crate) type CssToken = crate::source::ParserToken<CssRule>;

pub type CssResult<T> = Result<T, CssExpectError>;

#[derive(Debug, Clone, derive_more::From)]
pub(crate) enum CssExpectError {
    #[from]
    ParseError(pest::error::Error<CssRule>),
    #[from]
    InvalidSelector(SelectorExpectError),
    TooFewTokens(String),

    FailedExpectation(CssRule, CssRule),
    InvalidStyleAttrName(SourceSlice),
    InvalidAtRuleName(SourceSlice),
    InvalidAttributeValue(String),

    #[from]
    InvalidNumber(ParseFloatError),
}

#[derive(Debug, Clone)]
pub struct CssTokenTracker<'a>(crate::source::TokenTracker<'a, CssRule>);

#[derive(Clone, Debug)]
pub struct CssAtRuleValueExpector<'a> {
    css_expector: &'a CssTokenTracker<'a>,
    has_errored: bool,
    results: Vec<Result<CssStyleAttribute, CssExpectError>>,
}

// ============== IMPL ==============

impl<'a> CssTokenTracker<'a> {
    #[inline]
    pub fn new(css_token: &'a CssToken) -> CssTokenTracker<'a> {
        Self(TokenTracker::new(Boo::Borrowed(
            css_token.get_children().unwrap(),
        )))
    }

    #[inline]
    fn from_vec(vec: &'a Vec<CssToken>) -> Self {
        Self(TokenTracker::new(Boo::Borrowed(vec)))
    }

    #[inline]
    fn fail_type<O>(&self, expected: CssRule) -> CssResult<O> {
        let current_rule = self.boo.get(0.max(self.idx.get() - 1)).unwrap().get_rule();
        self.0
            .fail_because(CssExpectError::FailedExpectation(current_rule, expected))
    }

    fn parse_style_attr(
        attr_name: SourceSlice,
        component_value_list: &'a CssToken,
    ) -> CssResult<Box<[CssStyleAttribute]>> {
        let css_expector: CssTokenTracker<'a> = Self::new(component_value_list);

        match attr_name.get() {
            "width" => CssStyleValueExpector::new(&css_expector)
                .expect::<ValueTypes::CssWidth>()
                .resolve(),

            "height" => CssStyleValueExpector::new(&css_expector)
                .expect::<ValueTypes::CssHeight>()
                .resolve(),

            "padding" => CssStyleValueExpector::new(&css_expector)
                .expect::<ValueTypes::CssPadding>()
                .expect::<ValueTypes::CssPadding>()
                .expect::<ValueTypes::CssPadding>()
                .expect::<ValueTypes::CssPadding>()
                .resolve(),

            "spacing" => CssStyleValueExpector::new(&css_expector)
                .expect::<ValueTypes::CssSpacing>()
                .expect::<ValueTypes::CssSpacing>()
                .expect::<ValueTypes::CssSpacing>()
                .expect::<ValueTypes::CssSpacing>()
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

            _ => Err(CssExpectError::InvalidStyleAttrName(attr_name)),
        }
    }

    fn parse_at_rule_attr(
        &self,
        attr_name: SourceSlice,
        component_value_list: &'a CssToken,
    ) -> CssResult<Box<[CssStyleAttribute]>> {
        let css_expector: CssTokenTracker<'a> = Self::new(component_value_list);
        match attr_name.get() {
            _ => Err(CssExpectError::InvalidAtRuleName(attr_name)),
        }
    }

    pub fn expect_stylesheet(&'a self) -> CssResult<Stylesheet> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(CssExpectError::TooFewTokens("STYLESHEET".into()))?;

        if !matches!(token.get_rule(), CssRule::STYLESHEET) {
            return self.fail_type(CssRule::STYLESHEET);
        }

        let rule_expector = Self::new(token);

        let mut at_rules: HashMap<String, AtRule> = HashMap::new();
        let mut style_rules: Vec<(SelectorNode, CssStyleProperties)> = Vec::new();

        while !rule_expector.is_empty() {
            match rule_expector.peek().unwrap().get_rule() {
                CssRule::QUALIFIED_RULE => {
                    let (selectors, qualified_rule) = rule_expector.expect_qualified_rule()?;
                    for selector in selectors.iter() {
                        style_rules.push((selector.clone(), qualified_rule.clone()));
                    }
                }
                CssRule::AT_RULE => {
                    let at_rule = rule_expector.expect_at_rule()?;
                    at_rules.insert(at_rule.name.clone(), at_rule);
                }

                _ => unreachable!(),
            }
        }

        Ok(Stylesheet {
            at_rules,
            style_rules,
        })
    }

    pub fn expect_at_rule(&self) -> CssResult<crate::style::AtRule> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(CssExpectError::TooFewTokens("AT_RULE".into()))?;

        if !matches!(token.get_rule(), CssRule::AT_RULE) {
            return self.fail_type(CssRule::AT_RULE);
        }

        let name = token.get_children().unwrap()[0].get_children().unwrap()[0]
            .get_source()
            .to_string();

        Ok(crate::style::AtRule { name })
    }

    pub fn expect_qualified_rule(
        &'a self,
    ) -> CssResult<(Box<[SelectorNode]>, crate::style::CssStyleProperties)> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(CssExpectError::TooFewTokens("QUALIFIED_RULE".into()))?;

        if !matches!(token.get_rule(), CssRule::QUALIFIED_RULE) {
            return self.fail_type(CssRule::QUALIFIED_RULE);
        }

        // Self: QUALIFIED_RULE = { WS* ~ SELECTOR ~ WS* ~ DECL_BLOCK }
        let components = token.get_children().unwrap();

        // QUALIFIED_RULE -> SELECTOR
        let selector_slice: SourceSlice = components[0].get_source();
        let selector_parser_result = parse_source::<SelectorRule, SelectorParser>(
            selector_slice.get(),
            SelectorRule::SELECTOR_LIST,
        );

        let selector_parser = match selector_parser_result {
            Ok(val) => val,
            Err(err) => return Err(CssExpectError::InvalidSelector(err.into())),
        };

        let selector_vec = vec![selector_parser];
        let selector_expector = SelectorTokenTracker::from_vec(&selector_vec);

        match selector_expector.expect_selector_list() {
            Ok(selectors) => {
                let mut sheet = crate::style::properties::CssStyleProperties::default();

                // QUALIFIED_RULE -> DECL_BLOCK -> DECLARATION*
                let declarations = components[1].get_children().unwrap();
                for declaration_token in declarations.iter() {
                    if !matches!(declaration_token.get_rule(), CssRule::DECLARATION) {
                        return self.fail_because(CssExpectError::FailedExpectation(
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
                Ok((selectors, sheet))
            }
            Err(selector_err) => {
                self.fail_because(CssExpectError::InvalidSelector(selector_err))
            },
        }
    }

    pub fn expect_component_values(&self) -> CssResult<Self> {
        todo!()
    }

    /// Consumes the next token and extracts its `SourceSlice` representation if it is a valid
    /// identifier. Returns an `CssExpectError` otherwise.
    pub fn expect_identifier(&self) -> CssResult<SourceSlice> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(CssExpectError::TooFewTokens("IDENT".into()))?;

        if !matches!(token.get_rule(), CssRule::IDENT) {
            return self.fail_type(CssRule::IDENT);
        }

        Ok(token.get_source())
    }

    /// Consumes the next token and extracts its `f32` value if it is a valid number token.
    /// Returns an `CssExpectError` otherwise.
    pub fn expect_number(&self) -> CssResult<f32> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(CssExpectError::TooFewTokens("NUMBER".into()))?;

        if !matches!(token.get_rule(), CssRule::NUMBER) {
            return self.fail_type(CssRule::NUMBER);
        }

        match token.get_source().parse::<f32>() {
            Ok(num) => Ok(num),
            Err(err) => Err(err.into()),
        }
    }

    /// Consumes the next token and extracts its `f32` value if it is a valid percentage token.
    /// Returns an `CssExpectError` otherwise.
    pub fn expect_percentage(&self) -> CssResult<f32> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(CssExpectError::TooFewTokens("PERCENTAGE".into()))?;

        if !matches!(token.get_rule(), CssRule::PERCENTAGE) {
            return self.fail_type(CssRule::PERCENTAGE);
        }

        match token.get_source().parse::<f32>() {
            Ok(num) => Ok(num),
            Err(err) => Err(err.into()),
        }
    }

    /// Consumes the next token and extracts its `Dimension` representation if it is a valid
    /// dimension token. Returns an `CssExpectError` otherwise.
    pub fn expect_dimension(&self) -> CssResult<Dimension> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(CssExpectError::TooFewTokens("DIMENSION".into()))?;

        if !matches!(token.get_rule(), CssRule::DIMENSION) {
            return self.fail_type(CssRule::DIMENSION);
        }

        let components = token
            .get_children()
            .ok_or(CssExpectError::TooFewTokens("DIMENSION".into()))?;

        if !matches!(components[0].get_rule(), CssRule::NUMBER) {
            return self.fail_type(CssRule::NUMBER);
        }

        if !matches!(components[1].get_rule(), CssRule::IDENT) {
            return self.fail_type(CssRule::IDENT);
        }

        match components[0].get_source().parse::<f32>() {
            Ok(number) => {
                let unit_slice = components[1].get_source();
                let unit = unit_slice.get();
                Ok(Dimension::from_pair(number, &unit)
                    .expect(&format!("Failed to parse Dimension from '{number}{unit}'")))
            }
            Err(err) => Err(err.into()),
        }
    }

    /// Consumes the next token and extracts its `SourceSlice` representation if it is a valid quoted
    /// string token. Excludes surrounding quotes from the result. Returns an `CssExpectError` otherwise.
    pub fn expect_quoted_string(&self) -> CssResult<SourceSlice> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(CssExpectError::TooFewTokens("STRING".into()))?;

        if !matches!(token.get_rule(), CssRule::STRING) {
            return self.fail_type(CssRule::STRING);
        }

        let mut slice = token.get_source();
        slice.start = slice.source_info.location_from_idx(slice.start.idx + 1);
        slice.end = slice.source_info.location_from_idx(slice.end.idx - 1);

        Ok(slice)
    }

    /// Consumes the next token and extracts its unquoted URL value if it is a valid `url(...)`
    /// token. Surrounding syntax (e.g., `url(` and `)`) is excluded from the result. Returns
    /// an `CssExpectError` otherwise.
    pub fn expect_url(&self) -> CssResult<SourceSlice> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(CssExpectError::TooFewTokens("URL".into()))?;

        if !matches!(token.get_rule(), CssRule::URL) {
            return self.fail_type(CssRule::URL);
        }

        Ok(token.get_children().unwrap()[0].get_source())
    }

    /// Consumes the next token and extracts its `SourceSlice` representation if it is a
    /// valid hash token. Returns a `CssExpectError` otherwise.
    pub fn expect_hash(&self) -> CssResult<SourceSlice> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(CssExpectError::TooFewTokens("HASH".into()))?;

        if !matches!(token.get_rule(), CssRule::HASH) {
            return self.fail_type(CssRule::HASH);
        }

        let mut slice = token.get_source();
        slice.start = slice.source_info.location_from_idx(slice.start.idx + 1);

        Ok(slice)
    }

    /// Consumes the next token and extracts its function identifier along with a `CssTokenTracker`
    /// for its arguments if it is a valid function token. Returns an `CssExpectError` otherwise.
    pub fn expect_function_block(&'a self) -> CssResult<(SourceSlice, Option<Self>)> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(CssExpectError::TooFewTokens("FUNCTION_BLOCK".into()))?;

        if !matches!(token.get_rule(), CssRule::FUNCTION_BLOCK) {
            return self.fail_type(CssRule::FUNCTION_BLOCK);
        }

        let function_expector: CssTokenTracker<'a> = Self::new(token);
        let name = function_expector.expect_function()?;

        if !function_expector.is_empty() {
            let params = function_expector.expect_component_values()?;
            Ok((name, Some(params)))
        } else {
            Ok((name, None))
        }
    }

    pub fn expect_function(&'a self) -> CssResult<SourceSlice> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(CssExpectError::TooFewTokens("FUNCTION".into()))?;

        if !matches!(token.get_rule(), CssRule::FUNCTION) {
            return self.fail_type(CssRule::FUNCTION);
        }

        Self::new(token).expect_identifier()
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
                let expect_err = CssExpectError::InvalidAttributeValue(format!(
                    "Failed to parse type {} from '{err}'",
                    T::type_name()
                ));
                self.results.push(Err(expect_err));
            }
        }

        self
    }

    fn resolve(&mut self) -> Result<Box<[CssStyleAttribute]>, CssExpectError> {
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

impl<'a> std::ops::Deref for CssTokenTracker<'a> {
    type Target = crate::source::TokenTracker<'a, CssRule>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
