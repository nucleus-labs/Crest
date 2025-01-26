use super::{CssValue, TokenExpected};
use crate::Unit;

#[derive(Debug, Clone)]
pub struct CssOpacity(Unit);

impl From<Unit> for CssOpacity {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl Into<Unit> for CssOpacity {
    fn into(self) -> Unit {
        self.0
    }
}

impl CssValue for CssOpacity {
    type Keyword = super::KeywordNone;

    fn type_name() -> &'static str {
        "CssOpacity"
    }

    fn type_token() -> TokenExpected {
        TokenExpected::Ident | TokenExpected::Number | TokenExpected::Percentage
    }
}
