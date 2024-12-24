use super::{CssValue, TokenExpected};
use crate::Unit;

#[derive(Debug, Clone)]
pub struct CssOverflowX(Unit);

#[derive(Debug, Clone)]
pub struct CssOverflowY(Unit);

#[derive(Debug, Clone, strum_macros::EnumString, strum_macros::Display)]
pub enum KeywordOverflow {
    #[strum(to_string = "visible", serialize = "visible")]
    Visible,

    #[strum(to_string = "clip", serialize = "clip")]
    Clip,

    #[strum(to_string = "scroll", serialize = "scroll")]
    Scroll,
}

impl From<Unit> for CssOverflowX {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl Into<Unit> for CssOverflowX {
    fn into(self) -> Unit {
        self.0
    }
}

impl From<Unit> for CssOverflowY {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl Into<Unit> for CssOverflowY {
    fn into(self) -> Unit {
        self.0
    }
}

impl CssValue for CssOverflowX {
    type Keyword = KeywordOverflow;

    fn type_name() -> &'static str {
        "CssOverflowX"
    }

    fn type_token() -> TokenExpected {
        TokenExpected::Ident
    }
}

impl CssValue for CssOverflowY {
    type Keyword = KeywordOverflow;

    fn type_name() -> &'static str {
        "CssOverflowY"
    }

    fn type_token() -> TokenExpected {
        TokenExpected::Ident
    }
}
