use super::{CssValue, TokenExpected};
use crate::Unit;

#[derive(Debug, Clone)]
pub struct CssLeft(Unit);

#[derive(Debug, Clone)]
pub struct CssRight(Unit);

#[derive(Debug, Clone, strum_macros::EnumString, strum_macros::Display)]
pub enum KeywordPositionHorizontal {
    #[strum(to_string = "auto", serialize = "auto")]
    Auto,
}

impl From<Unit> for CssLeft {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl Into<Unit> for CssLeft {
    fn into(self) -> Unit {
        self.0
    }
}

impl From<Unit> for CssRight {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl Into<Unit> for CssRight {
    fn into(self) -> Unit {
        self.0
    }
}

impl CssValue for CssLeft {
    type Keyword = KeywordPositionHorizontal;

    fn type_name() -> &'static str {
        "CssLeft"
    }

    fn type_token() -> TokenExpected {
        TokenExpected::Dimension | TokenExpected::Percentage | TokenExpected::Ident
    }
}

impl CssValue for CssRight {
    type Keyword = KeywordPositionHorizontal;

    fn type_name() -> &'static str {
        "CssRight"
    }

    fn type_token() -> TokenExpected {
        TokenExpected::Dimension | TokenExpected::Percentage | TokenExpected::Ident
    }
}
