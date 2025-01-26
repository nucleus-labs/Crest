use super::{CssValue, TokenExpected};
use crate::Unit;

#[derive(Debug, Clone)]
pub struct CssLineHeight(Unit);

#[derive(Debug, Clone, strum_macros::EnumString, strum_macros::Display)]
pub enum KeywordLineHeight {
    #[strum(to_string = "normal", serialize = "normal")]
    Normal,
}

impl From<Unit> for CssLineHeight {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl Into<Unit> for CssLineHeight {
    fn into(self) -> Unit {
        self.0
    }
}

impl CssValue for CssLineHeight {
    type Keyword = KeywordLineHeight;

    fn type_name() -> &'static str {
        "CssLineHeight"
    }

    fn type_token() -> TokenExpected {
        TokenExpected::Dimension
            | TokenExpected::Percentage
            | TokenExpected::Ident
            | TokenExpected::Number
    }
}
