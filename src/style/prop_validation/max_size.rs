use super::{CssValue, TokenExpected};
use crate::Unit;

#[derive(Debug, Clone)]
pub struct CssMaxWidth(Unit);

#[derive(Debug, Clone)]
pub struct CssMaxHeight(Unit);

#[derive(Debug, Clone, strum_macros::EnumString, strum_macros::Display)]
pub enum KeywordMaxSize {
    #[strum(to_string = "none", serialize = "none")]
    None,

    #[strum(to_string = "max-content", serialize = "max-content")]
    MaxContent,

    #[strum(to_string = "min-content", serialize = "min-content")]
    MinContent,

    #[strum(to_string = "fit-content", serialize = "fit-content")]
    FitContent,

    #[strum(to_string = "stretch", serialize = "stretch")]
    Stretch,
}

impl From<Unit> for CssMaxWidth {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl Into<Unit> for CssMaxWidth {
    fn into(self) -> Unit {
        self.0
    }
}

impl From<Unit> for CssMaxHeight {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl Into<Unit> for CssMaxHeight {
    fn into(self) -> Unit {
        self.0
    }
}

impl CssValue for CssMaxWidth {
    type Keyword = KeywordMaxSize;

    fn type_name() -> &'static str {
        "CssMaxWidth"
    }

    fn type_token() -> TokenExpected {
        TokenExpected::Dimension | TokenExpected::Percentage | TokenExpected::Ident
    }
}

impl CssValue for CssMaxHeight {
    type Keyword = KeywordMaxSize;

    fn type_name() -> &'static str {
        "CssMaxHeight"
    }

    fn type_token() -> TokenExpected {
        TokenExpected::Dimension | TokenExpected::Percentage | TokenExpected::Ident
    }
}
