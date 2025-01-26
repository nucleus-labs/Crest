use super::{CssValue, TokenExpected};
use crate::Unit;

#[derive(Debug, Clone)]
pub struct CssObjectFit(Unit);

#[derive(Debug, Clone, strum_macros::EnumString, strum_macros::Display)]
pub enum KeywordObjectFit {
    #[strum(to_string = "contain", serialize = "contain")]
    Contain,

    #[strum(to_string = "cover", serialize = "cover")]
    Cover,

    #[strum(to_string = "fill", serialize = "fill")]
    Fill,

    #[strum(to_string = "none", serialize = "none")]
    None,

    #[strum(to_string = "scale-down", serialize = "scale-down")]
    ScaleDown,
}

impl From<Unit> for CssObjectFit {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl Into<Unit> for CssObjectFit {
    fn into(self) -> Unit {
        self.0
    }
}

impl CssValue for CssObjectFit {
    type Keyword = KeywordObjectFit;

    fn type_name() -> &'static str {
        "CssObjectFit"
    }

    fn type_token() -> TokenExpected {
        TokenExpected::Ident
    }
}
