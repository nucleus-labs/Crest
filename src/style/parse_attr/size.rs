use super::{CssValue, TokenExpected};
use crate::Unit;

#[derive(Debug, Clone)]
pub struct CssWidth(Unit);

#[derive(Debug, Clone)]
pub struct CssHeight(Unit);

#[derive(Debug, Clone, strum_macros::EnumString, strum_macros::Display)]
pub enum KeywordSize {
    #[strum(to_string = "max-content", serialize = "max-content")]
    MaxContent,
    #[strum(to_string = "min-content", serialize = "min-content")]
    MinContent,
    #[strum(to_string = "fit-content", serialize = "fit-content")]
    FitContent,
    #[strum(to_string = "auto", serialize = "auto")]
    Auto,
    #[strum(to_string = "stretch", serialize = "stretch")]
    Stretch,
}

impl From<Unit> for CssWidth {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl Into<Unit> for CssWidth {
    fn into(self) -> Unit {
        self.0
    }
}

impl From<Unit> for CssHeight {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl Into<Unit> for CssHeight {
    fn into(self) -> Unit {
        self.0
    }
}

impl CssValue for CssWidth {
    type Keyword = KeywordSize;

    fn type_name() -> &'static str {
        "CssWidth"
    }
    fn type_token() -> TokenExpected {
        TokenExpected::Ident | TokenExpected::Dimension
    }
}

impl CssValue for CssHeight {
    type Keyword = KeywordSize;

    fn type_name() -> &'static str {
        "CssHeight"
    }
    fn type_token() -> TokenExpected {
        TokenExpected::Ident | TokenExpected::Dimension
    }
}
