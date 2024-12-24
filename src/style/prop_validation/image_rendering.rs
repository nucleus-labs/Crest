use super::{CssValue, TokenExpected};
use crate::Unit;

#[derive(Debug, Clone)]
pub struct CssImageRendering(Unit);

#[derive(Debug, Clone, strum_macros::EnumString, strum_macros::Display)]
pub enum KeywordImageRendering {
    #[strum(to_string = "smooth", serialize = "smooth")]
    Smooth,

    #[strum(to_string = "crisp-edges", serialize = "crisp-edges")]
    CrispEdges,
}

impl From<Unit> for CssImageRendering {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl Into<Unit> for CssImageRendering {
    fn into(self) -> Unit {
        self.0
    }
}

impl CssValue for CssImageRendering {
    type Keyword = KeywordImageRendering;

    fn type_name() -> &'static str {
        "CssImageRendering"
    }

    fn type_token() -> TokenExpected {
        TokenExpected::Ident
    }
}
