use super::{CssValue, TokenExpected};
use crate::Unit;

#[derive(Debug, Clone)]
pub struct CssVerticalAlign(Unit);

#[derive(Debug, Clone, strum_macros::EnumString, strum_macros::Display)]
pub enum KeywordVerticalAlign {
    #[strum(to_string = "top", serialize = "top")]
    Top,

    #[strum(to_string = "middle", serialize = "middle")]
    Middle,

    #[strum(to_string = "bottom", serialize = "bottom")]
    Bottom,
}

impl From<Unit> for CssVerticalAlign {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl Into<Unit> for CssVerticalAlign {
    fn into(self) -> Unit {
        self.0
    }
}

impl CssValue for CssVerticalAlign {
    type Keyword = KeywordVerticalAlign;

    fn type_name() -> &'static str {
        "CssVerticalAlign"
    }

    fn type_token() -> TokenExpected {
        TokenExpected::Ident
    }
}
