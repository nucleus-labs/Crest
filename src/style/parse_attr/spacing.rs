use super::CssValue;
use crate::parse::TokenExpected;
use crate::Unit;

#[derive(Debug, Clone)]
pub struct CssSpacing(Unit);

#[derive(Debug, Clone, strum_macros::EnumString, strum_macros::Display)]
pub enum KeywordSpacing {
    #[strum(to_string = "auto", serialize = "auto")]
    Auto,
}

impl From<Unit> for CssSpacing {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl Into<Unit> for CssSpacing {
    fn into(self) -> Unit {
        self.0
    }
}

impl CssValue for CssSpacing {
    type Keyword = KeywordSpacing;

    fn type_name() -> &'static str {
        "CssSpacing"
    }
    fn type_token() -> TokenExpected {
        TokenExpected::Ident | TokenExpected::Dimension | TokenExpected::Percentage
    }
}
