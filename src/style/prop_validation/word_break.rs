use super::{CssValue, TokenExpected};
use crate::Unit;

#[derive(Debug, Clone)]
pub struct CssWordBreak(Unit);

#[derive(Debug, Clone, strum_macros::EnumString, strum_macros::Display)]
pub enum KeywordWordBreak {
    #[strum(to_string = "normal", serialize = "normal")]
    Normal,

    #[strum(to_string = "break-all", serialize = "break-all")]
    BreakAll,

    #[strum(to_string = "keep-all", serialize = "keep-all")]
    KeepAll,

    #[strum(to_string = "break-word", serialize = "break-word")]
    BreakWord,
}

impl From<Unit> for CssWordBreak {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl Into<Unit> for CssWordBreak {
    fn into(self) -> Unit {
        self.0
    }
}

impl CssValue for CssWordBreak {
    type Keyword = KeywordWordBreak;

    fn type_name() -> &'static str {
        "CssWordBreak"
    }

    fn type_token() -> TokenExpected {
        TokenExpected::Ident
    }
}
