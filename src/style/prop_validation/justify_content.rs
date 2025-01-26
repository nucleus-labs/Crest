use super::{CssValue, TokenExpected};
use crate::Unit;

#[derive(Debug, Clone)]
pub struct CssJustifyContent(Unit);

#[derive(Debug, Clone, strum_macros::EnumString, strum_macros::Display)]
pub enum KeywordJustifyContent {
    #[strum(to_string = "left", serialize = "left")]
    Left,

    #[strum(to_string = "center", serialize = "center")]
    Center,

    #[strum(to_string = "right", serialize = "right")]
    Right,
}

impl From<Unit> for CssJustifyContent {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl Into<Unit> for CssJustifyContent {
    fn into(self) -> Unit {
        self.0
    }
}

impl CssValue for CssJustifyContent {
    type Keyword = KeywordJustifyContent;

    fn type_name() -> &'static str {
        "CssJustifyContent"
    }

    fn type_token() -> TokenExpected {
        TokenExpected::Ident
    }
}
