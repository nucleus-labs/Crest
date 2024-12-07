use super::CssValue;
use crate::Unit;

#[derive(Debug, Clone)]
pub struct CssPadding(Unit);

impl From<Unit> for CssPadding {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl Into<Unit> for CssPadding {
    fn into(self) -> Unit {
        self.0
    }
}

impl CssValue for CssPadding {
    type Keyword = super::KeywordNone;

    fn type_name() -> &'static str {
        "CssPadding"
    }
    fn type_token() -> crate::parse::TokenExpected {
        crate::parse::TokenExpected::Dimension
    }
}
