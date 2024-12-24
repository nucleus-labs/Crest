use super::{CssValue, TokenExpected};
use crate::Unit;

#[derive(Debug, Clone)]
pub struct CssTop(Unit);

#[derive(Debug, Clone)]
pub struct CssBottom(Unit);

#[derive(Debug, Clone, strum_macros::EnumString, strum_macros::Display)]
pub enum KeywordPositionVertical {
    #[strum(to_string = "auto", serialize = "auto")]
    Auto,
}

impl From<Unit> for CssTop {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl Into<Unit> for CssTop {
    fn into(self) -> Unit {
        self.0
    }
}

impl From<Unit> for CssBottom {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl Into<Unit> for CssBottom {
    fn into(self) -> Unit {
        self.0
    }
}

impl CssValue for CssTop {
    type Keyword = KeywordPositionVertical;

    fn type_name() -> &'static str {
        "CssTop"
    }

    fn type_token() -> TokenExpected {
        TokenExpected::Dimension | TokenExpected::Percentage | TokenExpected::Ident
    }
}

impl CssValue for CssBottom {
    type Keyword = KeywordPositionVertical;

    fn type_name() -> &'static str {
        "CssBottom"
    }

    fn type_token() -> TokenExpected {
        TokenExpected::Dimension | TokenExpected::Percentage | TokenExpected::Ident
    }
}
