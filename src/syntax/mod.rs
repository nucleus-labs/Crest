pub(crate) mod pest_css;

pub use pest_css::CssParser;
pub use pest_css::Rule as CssRule;

use std::cell::Cell;
use std::collections::HashMap;
use std::num::ParseFloatError;
use std::sync::Arc;

use crate::selector::{
    self, SelectorExpectError, SelectorNode, SelectorParser, SelectorRule, SelectorTokenTracker,
};

use crate::style::{
    prop_validation::{types as ValueTypes, CssAttributeValue, CssStyleAttribute, CssValue},
    AtRule, CssStyleProperties, CssStyleValueExpector, Stylesheet,
};

use crate::boo::Boo;
use crate::source::{
    parse_source, SourceInfo, SourceLocation, SourceSlice, StackInfo, TokenTracker,
};
use crate::style::properties::CssStyleProperty;
use crate::unit::{Dimension, Unit};

pub(crate) type CssStackInfo = StackInfo<CssRule>;
pub(crate) type CssToken = crate::source::ParserToken<CssRule>;

pub type CssResult<T> = Result<T, CssExpectError>;

#[derive(Debug, Clone, derive_more::From)]
pub enum CssExpectError {
    #[from]
    ParseError(pest::error::Error<CssRule>),
    #[from]
    InvalidSelector(SelectorExpectError),
    TooFewTokens(String),

    FailedExpectation(CssRule, CssRule),
    InvalidStyleAttrName(SourceSlice),
    InvalidAtRuleName(SourceSlice),
    InvalidAttributeValue(SourceLocation, String),

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
    pub fn new(css_token: &'a CssToken) -> Option<CssTokenTracker<'a>> {
        css_token
            .get_children()
            .map(|x| Self(TokenTracker::new(Boo::Borrowed(x))))
    }

    #[inline]
    pub fn from_vec(vec: &'a Vec<CssToken>) -> Self {
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
        declaration_groups_token: &'a CssToken,
    ) -> CssResult<Box<[CssStyleAttribute]>> {
        let name = attr_name.get();

        match name {
            "color" => CssStyleValueExpector::new(declaration_groups_token)
                .expect::<ValueTypes::CssColor>()
                .resolve(),
            "background-color" => CssStyleValueExpector::new(declaration_groups_token)
                .expect::<ValueTypes::CssBackgroundColor>()
                .resolve(),
            "width" => CssStyleValueExpector::new(declaration_groups_token)
                .expect::<ValueTypes::CssWidth>()
                .resolve(),

            "height" => CssStyleValueExpector::new(declaration_groups_token)
                .expect::<ValueTypes::CssHeight>()
                .resolve(),

            "padding" => CssStyleValueExpector::new(declaration_groups_token)
                .expect::<ValueTypes::CssPadding>()
                .optional::<ValueTypes::CssPadding>()
                .optional::<ValueTypes::CssPadding>()
                .optional::<ValueTypes::CssPadding>()
                .resolve(),

            "spacing" => CssStyleValueExpector::new(declaration_groups_token)
                .expect::<ValueTypes::CssSpacing>()
                .optional::<ValueTypes::CssSpacing>()
                .optional::<ValueTypes::CssSpacing>()
                .optional::<ValueTypes::CssSpacing>()
                .resolve(),

            "font-family" => todo!(),
            "font-size" => CssStyleValueExpector::new(declaration_groups_token)
                .expect::<ValueTypes::CssFontSize>()
                .resolve(),
            "font-shaping" => todo!(),

            "line-height" => CssStyleValueExpector::new(declaration_groups_token)
                .expect::<ValueTypes::CssLineHeight>()
                .resolve(),

            "text-wrap" => todo!(),

            "max-width" => CssStyleValueExpector::new(declaration_groups_token)
                .expect::<ValueTypes::CssMaxWidth>()
                .resolve(),
            "max-height" => CssStyleValueExpector::new(declaration_groups_token)
                .expect::<ValueTypes::CssMaxHeight>()
                .resolve(),

            "justify-content" => todo!(),
            "vertical-align" => CssStyleValueExpector::new(declaration_groups_token)
                .expect::<ValueTypes::CssVerticalAlign>()
                .resolve(),

            "overflow-x" => todo!(),
            "overflow-y" => todo!(),

            "object-fit" => todo!(),

            "image-rendering" => todo!(),
            "rotation" => todo!(),
            "opacity" => todo!(),

            _ => {
                let prop_name = name.to_string();
                let mut prop_groups: Vec<Vec<Unit>> = Vec::new();

                for decl_group in declaration_groups_token.get_children().unwrap().iter() {
                    let decl_values = decl_group.get_children().unwrap();
                    let mut prop_values: Vec<Unit> = Vec::new();

                    for decl_value in decl_values.iter() {
                        if matches!(decl_value.get_rule(), CssRule::DECLARATION_SHORTHAND) {
                        } else {
                            // DECLARATION_VALUE
                            let next_token = &decl_value.get_children().unwrap()[0];
                            // println!("{prop_name}: {:?} -> {:?} -> {:?} -> {:?}", declaration_groups_token.get_rule(), decl_group.get_rule(), decl_value.get_rule(), next_token.get_rule());
                            let component_expector = Self::new(next_token)
                                .ok_or(CssExpectError::TooFewTokens(format!(
                                    "COMPONENT_VALUE [ {} ]",
                                    next_token.get_source()
                                )))
                                .unwrap();

                            let component_rule = next_token.get_children().unwrap()[0].get_rule();
                            match component_rule {
                                CssRule::SIMPLE_BLOCK | CssRule::FUNCTION_BLOCK => todo!("unknown values such as {prop_name} do not currently support SIMPLE_BLOCK or FUNCTION_BLOCK value types"),

                                CssRule::DIMENSION => prop_values.push(component_expector.expect_dimension()?.into()),
                                CssRule::NUMBER => prop_values.push(component_expector.expect_number()?.into()),
                                CssRule::IDENT => prop_values.push(Unit::UnknownIdent(component_expector.expect_identifier()?.to_string())),
                                CssRule::STRING => prop_values.push(component_expector.expect_quoted_string()?.to_string().into()),
                                CssRule::HASH => prop_values.push(Unit::Hash(component_expector.expect_hash()?.to_string().into())),
                                CssRule::PERCENTAGE => prop_values.push(Unit::Percentage(component_expector.expect_percentage()?.into())),

                                _ => unreachable!(),
                            }
                        }
                    }

                    prop_groups.push(prop_values);
                }

                let result_vec = vec![CssStyleAttribute::UnknownProperty(prop_name, prop_groups)];
                Ok(result_vec.into_boxed_slice())
            }
        }
    }

    fn parse_at_rule_attr(
        &self,
        attr_name: SourceSlice,
        component_value_list: &'a CssToken,
    ) -> CssResult<Box<[CssStyleAttribute]>> {
        let css_expector: CssTokenTracker<'a> = Self::new(component_value_list).unwrap();
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

        let rule_expector = Self::new(token).unwrap();

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
            SourceInfo::new(selector_slice.get().into()),
            SelectorRule::SELECTOR_LIST,
        );

        let selector_parser =
            selector_parser_result.map_err(|err| CssExpectError::InvalidSelector(err.into()))?;
        let selector_vec = vec![selector_parser];
        let selector_expector = SelectorTokenTracker::from_vec(&selector_vec);

        match selector_expector.expect_selector_list() {
            Ok(selectors) => {
                let mut sheet = crate::style::properties::CssStyleProperties::default();

                // QUALIFIED_RULE -> DECL_BLOCK -> DECLARATION*
                if let Some(declarations) = components[1].get_children() {
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

                        // QUALIFIED_RULE -> DECL_BLOCK -> DECLARATION -> DECLARATION_VALUES
                        let decl_values =
                            Self::parse_style_attr(decl_name, &decl_components[1]).unwrap();

                        // QUALIFIED_RULE -> DECL_BLOCK -> DECLARATION -> IMPORTANT
                        let important = decl_components.len() > 2;

                        sheet.push((
                            CssStyleProperty::from_attribute_values(decl_values),
                            important,
                        ));
                    }
                }
                Ok((selectors, sheet))
            }
            Err(selector_err) => self.fail_because(CssExpectError::InvalidSelector(selector_err)),
        }
    }

    pub fn expect_component_values(&'a self) -> CssResult<Vec<Unit>> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(CssExpectError::TooFewTokens("COMPONENT_VALUE_LIST".into()))?;

        if !matches!(token.get_rule(), CssRule::COMPONENT_VALUE_LIST) {
            return self.fail_type(CssRule::COMPONENT_VALUE_LIST);
        }

        let values_expector = Self::new(token).unwrap();
        let mut values: Vec<Unit> = Vec::new();

        while let Ok(unit) = values_expector.expect_component_value() {
            values.push(unit);
        }

        Ok(values)
    }

    pub fn expect_component_value(&'a self) -> CssResult<Unit> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(CssExpectError::TooFewTokens("COMPONENT_VALUE".into()))?;

        if !matches!(token.get_rule(), CssRule::COMPONENT_VALUE) {
            return self.fail_type(CssRule::COMPONENT_VALUE);
        }

        let value_expector = Self::new(token)
            .ok_or_else(|| panic!("Failed to create am expector from '{}'", token.get_source()))
            .unwrap();

        match value_expector.peek().unwrap().get_rule() {
            CssRule::SIMPLE_BLOCK | CssRule::FUNCTION_BLOCK => todo!("component values do not currently support SIMPLE_BLOCK or FUNCTION_BLOCK value types"),

            CssRule::DIMENSION => Ok(value_expector.expect_dimension()?.into()),
            CssRule::NUMBER => Ok(value_expector.expect_number()?.into()),
            CssRule::IDENT => Ok(Unit::UnknownIdent(value_expector.expect_identifier()?.to_string())),
            CssRule::STRING => Ok(value_expector.expect_quoted_string()?.to_string().into()),
            CssRule::HASH => Ok(Unit::Hash(value_expector.expect_hash()?.to_string().into())),
            CssRule::PERCENTAGE => Ok(Unit::Percentage(value_expector.expect_percentage()?.into())),

            _ => unreachable!(),
        }
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

        let mut slice = token.get_source();
        slice.end = slice.source_info.location_from_idx(slice.end.idx - 1);

        match slice.parse::<f32>() {
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
    pub fn expect_function_block(&'a self) -> CssResult<(SourceSlice, Vec<Unit>)> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(CssExpectError::TooFewTokens("FUNCTION_BLOCK".into()))?;

        if !matches!(token.get_rule(), CssRule::FUNCTION_BLOCK) {
            return self.fail_type(CssRule::FUNCTION_BLOCK);
        }

        let function_expector: CssTokenTracker<'a> = Self::new(token).unwrap();
        let name = function_expector.expect_function()?;

        if !function_expector.is_empty() {
            let mut params: Vec<Unit> = Vec::new();

            while let Ok(unit) = function_expector.expect_component_value() {
                params.push(unit);
            }

            Ok((name, params))
        } else {
            Ok((name, Vec::new()))
        }
    }

    pub fn expect_function(&'a self) -> CssResult<SourceSlice> {
        let token: &CssToken = self
            .pop_front()
            .ok_or(CssExpectError::TooFewTokens("FUNCTION".into()))?;

        if !matches!(token.get_rule(), CssRule::FUNCTION) {
            return self.fail_type(CssRule::FUNCTION);
        }

        Self::new(token).unwrap().expect_identifier()
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
            Err(source) => {
                self.has_errored = true;

                let expect_err = CssExpectError::InvalidAttributeValue(
                    self.css_expector.get_location().unwrap().start,
                    format!("Failed to parse type {} from '{source}'", T::type_name()),
                );
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
