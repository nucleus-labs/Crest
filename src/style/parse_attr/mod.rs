pub mod color;
pub mod font;
pub mod padding;
pub mod size;
pub mod spacing;

pub mod types {
    use super::*;

    pub use color::{CssBackgroundColor, CssColor};
    pub use font::{CssFontFamily, CssFontSize};
    pub use padding::CssPadding;
    pub use size::{CssHeight, CssWidth};
    pub use spacing::CssSpacing;

    #[derive(Debug, Clone, strum_macros::EnumString)]
    pub enum KeywordGlobal {
        #[strum(serialize = "inherit")]
        Inherit,
        #[strum(serialize = "initial")]
        Initial,
        #[strum(serialize = "revert")]
        Revert,
        #[strum(serialize = "revert-layer")]
        RevertLayer,
        #[strum(serialize = "unset")]
        Unset,
    }
}

use std::borrow::Borrow;
use std::str::FromStr;

use crate::parse::{CssRule, CssToken, SourceSlice};
use crate::parse::{CssTokenTracker, TokenExpected};
use crate::{
    unit::{Angle, Dimension, Length, Resolution},
    Unit,
};
use types::KeywordGlobal;

type AsyncHandle<T> = std::sync::Arc<std::sync::RwLock<T>>;

/// CssValue is a standardized interface for css style attribute values. Each css attribute
/// has its own requirements for value types, so this interface is for allowing defined
/// attributes to use the type system to declare what they expect, allowing new css attributes
/// to be added fairly trivially.
pub trait CssValue: Sized + From<Unit> + Into<Unit> {
    type Keyword: std::fmt::Debug + std::fmt::Display + Clone + FromStr;

    fn type_name() -> &'static str;
    fn type_token() -> crate::parse::TokenExpected;

    fn parse(tracker: &CssTokenTracker) -> Result<CssAttributeValue<Self>, String> {
        let valid_tokens = Self::type_token();

        if valid_tokens.intersects(TokenExpected::Ident) {
            if let Ok(ident) = tracker.expect_identifier() {
                if let Ok(keyword) = ident.parse::<Self::Keyword>() {
                    return Ok(CssAttributeValue::Keyword(keyword));
                } else if let Ok(global) = ident.parse::<KeywordGlobal>() {
                    return Ok(CssAttributeValue::Global(global));
                }
            }
        }

        if valid_tokens.intersects(TokenExpected::QuotedString) {
            match &tracker.expect_quoted_string() {
                Ok(string) => {
                    return Ok(CssAttributeValue::Value(Unit::String(string.to_string())))
                }
                Err(_) => (),
            };
        }

        if valid_tokens.intersects(TokenExpected::UnquotedUrl) {
            match tracker.expect_url() {
                Ok(url) => return Ok(CssAttributeValue::Value(Unit::String(url.to_string()))),
                Err(_) => (),
            };
        }

        if valid_tokens.intersects(TokenExpected::Number) {
            match tracker.expect_number() {
                Ok(num) => return Ok(CssAttributeValue::Value(Unit::Number(num))),
                Err(_) => (),
            };
        }

        if valid_tokens.intersects(TokenExpected::Percentage) {
            match tracker.expect_percentage() {
                Ok(percent) => {
                    return Ok(CssAttributeValue::Value(Unit::Percentage(percent.into())))
                }
                Err(_) => (),
            };
        }

        if valid_tokens.intersects(TokenExpected::Dimension) {
            match tracker.expect_dimension() {
                Ok(dim) => return Ok(CssAttributeValue::Value(Unit::Dimension(dim))),
                Err(_) => (),
            };
        }

        Err(format!(
            "Failed to parse CssValue of type '{}' from '{}'",
            Self::type_name(),
            tracker.peek().unwrap().get_source()
        ))
    }
}

#[derive(Debug, Clone)]
pub struct KeywordNone;

#[derive(Debug, Clone)]
pub enum CssAttributeValue<T: CssValue> {
    Value(Unit),
    Keyword(<T as CssValue>::Keyword),
    Global(KeywordGlobal),
}

#[derive(Debug, Clone, derive_more::From)]
pub enum CssStyleAttribute {
    #[from]
    Color(CssAttributeValue<color::CssColor>),
    #[from]
    BackgroundColor(CssAttributeValue<color::CssBackgroundColor>),

    #[from]
    Width(CssAttributeValue<size::CssWidth>),
    #[from]
    Height(CssAttributeValue<size::CssHeight>),

    #[from]
    Padding(CssAttributeValue<padding::CssPadding>),
    #[from]
    Spacing(CssAttributeValue<spacing::CssSpacing>),

    #[from]
    FontFamily(CssAttributeValue<font::CssFontFamily>),
    #[from]
    FontSize(CssAttributeValue<font::CssFontSize>),

    Uri(String),
}

impl FromStr for KeywordNone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err(())
    }
}

impl std::fmt::Display for KeywordNone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl std::fmt::Display for types::KeywordGlobal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeywordGlobal::Inherit => write!(f, "inherit"),
            KeywordGlobal::Initial => write!(f, "initial"),
            KeywordGlobal::Revert => write!(f, "revert"),
            KeywordGlobal::RevertLayer => write!(f, "revert-layer"),
            KeywordGlobal::Unset => write!(f, "unset"),
        }
    }
}

impl<T: CssValue> std::fmt::Display for CssAttributeValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssAttributeValue::Value(unit) => write!(f, "{unit}"),
            CssAttributeValue::Keyword(keyword) => write!(f, "{keyword}"),
            CssAttributeValue::Global(keyword_global) => write!(f, "{keyword_global}"),
        }
    }
}

impl std::fmt::Display for CssStyleAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssStyleAttribute::Width(css_attribute_value) => write!(f, "{css_attribute_value}"),
            CssStyleAttribute::Height(css_attribute_value) => write!(f, "{css_attribute_value}"),
            CssStyleAttribute::Padding(css_attribute_value) => todo!(),
            CssStyleAttribute::Spacing(css_attribute_value) => todo!(),
            CssStyleAttribute::FontFamily(css_attribute_value) => todo!(),
            CssStyleAttribute::FontSize(css_attribute_value) => todo!(),
            CssStyleAttribute::Color(css_attribute_value) => todo!(),
            CssStyleAttribute::BackgroundColor(css_attribute_value) => todo!(),
            CssStyleAttribute::Uri(_) => todo!(),
        }
    }
}
