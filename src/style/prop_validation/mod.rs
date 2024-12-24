pub mod color;
pub mod font;
pub mod image_rendering;
pub mod justify_content;
pub mod line_height;
pub mod max_size;
pub mod object_fit;
pub mod opacity;
pub mod overflow;
pub mod padding;
pub mod position_horizontal;
pub mod position_vertical;
pub mod size;
pub mod spacing;
pub mod vertical_align;
pub mod word_break;

pub mod types {
    use super::*;

    pub use color::{CssBackgroundColor, CssColor};
    pub use font::{CssFontFamily, CssFontSize};
    pub use image_rendering::CssImageRendering;
    pub use justify_content::CssJustifyContent;
    pub use line_height::CssLineHeight;
    pub use max_size::{CssMaxHeight, CssMaxWidth};
    pub use object_fit::CssObjectFit;
    pub use opacity::CssOpacity;
    pub use overflow::{CssOverflowX, CssOverflowY};
    pub use padding::CssPadding;
    pub use position_horizontal::{CssLeft, CssRight};
    pub use position_vertical::{CssBottom, CssTop};
    pub use size::{CssHeight, CssWidth};
    pub use spacing::CssSpacing;
    pub use vertical_align::CssVerticalAlign;
    pub use word_break::CssWordBreak;

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

use crate::source::SourceSlice;
use crate::syntax::{CssRule, CssToken, CssTokenTracker};
use crate::unit::{Angle, Dimension, Length, Resolution, Unit};
use types::KeywordGlobal;

type AsyncHandle<T> = std::sync::Arc<std::sync::RwLock<T>>;

#[bitmask_enum::bitmask]
pub enum TokenExpected {
    /// A [`<ident-token>`](https://drafts.csswg.org/css-syntax/#ident-token-diagram)
    Ident,

    /// A [`<hash-token>`](https://drafts.csswg.org/css-syntax/#hash-token-diagram) with the type flag set to "unrestricted"
    Hash,

    /// A [`<string-token>`](https://drafts.csswg.org/css-syntax/#string-token-diagram)
    QuotedString,

    /// A [`<url-token>`](https://drafts.csswg.org/css-syntax/#url-token-diagram)
    UnquotedUrl,

    /// A [`<number-token>`](https://drafts.csswg.org/css-syntax/#number-token-diagram)
    Number,

    /// A [`<percentage-token>`](https://drafts.csswg.org/css-syntax/#percentage-token-diagram)
    Percentage,

    /// A [`<dimension-token>`](https://drafts.csswg.org/css-syntax/#dimension-token-diagram)
    Dimension,

    /// A [`<function-token>`](https://drafts.csswg.org/css-syntax/#function-token-diagram)
    Function,
}

/// CssValue is a standardized interface for css style attribute values. Each css attribute
/// has its own requirements for value types, so this interface is for allowing defined
/// attributes to use the type system to declare what they expect, allowing new css attributes
/// to be added fairly trivially.
pub trait CssValue: Sized + From<Unit> + Into<Unit> {
    type Keyword: std::fmt::Debug + std::fmt::Display + Clone + FromStr;

    fn type_name() -> &'static str;
    fn type_token() -> TokenExpected;

    fn parse(tracker: &CssTokenTracker) -> Result<CssAttributeValue<Self>, String> {
        let valid_tokens = Self::type_token();

        if valid_tokens.intersects(TokenExpected::Ident) {
            if let Ok(ident) = tracker.expect_identifier() {
                if let Ok(keyword) = ident.parse::<Self::Keyword>() {
                    return Ok(CssAttributeValue::Keyword(keyword));
                } else if let Ok(global) = ident.parse::<KeywordGlobal>() {
                    return Ok(CssAttributeValue::Global(global));
                } else {
                    return Ok(CssAttributeValue::Value(Unit::UnknownIdent(
                        ident.to_string(),
                    )));
                }
            }
        }

        if valid_tokens.intersects(TokenExpected::QuotedString) {
            if let Ok(string) = tracker.expect_quoted_string() {
                return Ok(CssAttributeValue::Value(Unit::String(string.to_string())));
            };
        }

        if valid_tokens.intersects(TokenExpected::UnquotedUrl) {
            if let Ok(url) = tracker.expect_url() {
                return Ok(CssAttributeValue::Value(Unit::String(url.to_string())));
            };
        }

        if valid_tokens.intersects(TokenExpected::Number) {
            if let Ok(num) = tracker.expect_number() {
                return Ok(CssAttributeValue::Value(Unit::Number(num)));
            };
        }

        if valid_tokens.intersects(TokenExpected::Percentage) {
            if let Ok(percent) = tracker.expect_percentage() {
                return Ok(CssAttributeValue::Value(Unit::Percentage(percent.into())))
            };
        }

        if valid_tokens.intersects(TokenExpected::Hash) {
            if let Ok(hash) =  tracker.expect_hash() {
                return Ok(CssAttributeValue::Value(Unit::Hash(
                    hash.to_string().into(),
                )))
            }
        }

        if valid_tokens.intersects(TokenExpected::Dimension) {
            if let Ok(dim) = tracker.expect_dimension() {
                return Ok(CssAttributeValue::Value(Unit::Dimension(dim)));
            };
        }
        
        if let Ok((name, params)) = tracker.expect_function_block() {
            return Ok(CssAttributeValue::Value(Unit::Function {
                name: name.to_string(),
                required_return_type: valid_tokens.clone(),
                params,
            }));
        }

        let peeked = tracker.peek().unwrap();
        Err(format!(
            "Failed to parse CssValue of type '{}' from '{}'; current token's type: {}",
            Self::type_name(),
            peeked.get_source(),
            peeked.get_rule()
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
    Color(CssAttributeValue<types::CssColor>),
    #[from]
    BackgroundColor(CssAttributeValue<types::CssBackgroundColor>),

    #[from]
    Width(CssAttributeValue<types::CssWidth>),
    #[from]
    Height(CssAttributeValue<types::CssHeight>),

    #[from]
    Padding(CssAttributeValue<types::CssPadding>),
    #[from]
    Spacing(CssAttributeValue<types::CssSpacing>),

    #[from]
    FontFamily(CssAttributeValue<types::CssFontFamily>),
    #[from]
    FontSize(CssAttributeValue<types::CssFontSize>),

    #[from]
    Top(CssAttributeValue<types::CssTop>),
    #[from]
    Bottom(CssAttributeValue<types::CssBottom>),
    #[from]
    Left(CssAttributeValue<types::CssLeft>),
    #[from]
    Right(CssAttributeValue<types::CssRight>),

    #[from]
    LineHeight(CssAttributeValue<types::CssLineHeight>),

    #[from]
    WordBreak(CssAttributeValue<types::CssWordBreak>),

    #[from]
    MaxWidth(CssAttributeValue<types::CssMaxWidth>),
    #[from]
    MaxHeight(CssAttributeValue<types::CssMaxHeight>),

    #[from]
    JustifyContent(CssAttributeValue<types::CssJustifyContent>),
    #[from]
    VerticalAlign(CssAttributeValue<types::CssVerticalAlign>),

    #[from]
    OverflowX(CssAttributeValue<types::CssOverflowX>),
    #[from]
    OverflowY(CssAttributeValue<types::CssOverflowY>),

    #[from]
    ImageRendering(CssAttributeValue<types::CssImageRendering>),
    #[from]
    ObjectFit(CssAttributeValue<types::CssObjectFit>),
    #[from]
    Opacity(CssAttributeValue<types::CssOpacity>),

    #[from]
    UnknownProperty(String, Vec<Vec<Unit>>),
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
            Self::UnknownProperty(prop_name, prop_values) => {
                // [[a, b, c], [d, e, f]] => "a b c, d e f"
                let prop_string = prop_values
                    .iter()
                    .map(|group| {
                        // [a, b, c] => "a b c"
                        group
                            .iter()
                            .map(|values| values.to_string())
                            .collect::<Vec<_>>()
                            .join(" ")
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{prop_name}: {prop_string}")
            }

            Self::Width(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::Height(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::Padding(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::Spacing(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::FontFamily(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::FontSize(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::Color(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::BackgroundColor(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::Top(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::Bottom(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::Left(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::Right(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::LineHeight(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::WordBreak(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::MaxWidth(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::MaxHeight(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::JustifyContent(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::VerticalAlign(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::OverflowX(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::OverflowY(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::ImageRendering(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::ObjectFit(css_attribute_value) => write!(f, "{css_attribute_value}"),
            Self::Opacity(css_attribute_value) => write!(f, "{css_attribute_value}"),
        }
    }
}
