use crate::parse::TokenExpected;

use super::CssValue;
use crate::Unit;

#[derive(Debug, Clone)]
pub struct CssColor(Unit);

#[derive(Debug, Clone)]
pub struct CssBackgroundColor(Unit);

#[derive(Debug, Clone, strum_macros::EnumString, strum_macros::Display)]
pub enum KeywordColor {
    #[strum(to_string = "currentcolor", serialize = "currentcolor")]
    CurrentColor,

    #[strum(to_string = "black", serialize = "black")]
    Black,
    #[strum(to_string = "silver", serialize = "silver")]
    Silver,
    #[strum(to_string = "gray", serialize = "gray")]
    Gray,
    #[strum(to_string = "white", serialize = "white")]
    White,
    #[strum(to_string = "maroon", serialize = "maroon")]
    Maroon,
    #[strum(to_string = "red", serialize = "red")]
    Red,
    #[strum(to_string = "purple", serialize = "purple")]
    Purple,
    #[strum(to_string = "fuchsia", serialize = "fuchsia")]
    Fuchsia,
    #[strum(to_string = "green", serialize = "green")]
    Green,
    #[strum(to_string = "lime", serialize = "lime")]
    Lime,
    #[strum(to_string = "olive", serialize = "olive")]
    Olive,
    #[strum(to_string = "yellow", serialize = "yellow")]
    Yellow,
    #[strum(to_string = "navy", serialize = "navy")]
    Navy,
    #[strum(to_string = "blue", serialize = "blue")]
    Blue,
    #[strum(to_string = "teal", serialize = "teal")]
    Teal,
    #[strum(to_string = "aqua", serialize = "aqua")]
    Aqua,
    // TODO: extended colours
}

impl From<Unit> for CssColor {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl From<Unit> for CssBackgroundColor {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl Into<Unit> for CssColor {
    fn into(self) -> Unit {
        self.0
    }
}

impl Into<Unit> for CssBackgroundColor {
    fn into(self) -> Unit {
        self.0
    }
}

impl CssValue for CssColor {
    type Keyword = KeywordColor;

    fn type_name() -> &'static str {
        "CssColor"
    }
    fn type_token() -> TokenExpected {
        TokenExpected::QuotedString | TokenExpected::Ident | TokenExpected::Hash
    }
}

impl CssValue for CssBackgroundColor {
    type Keyword = KeywordColor;

    fn type_name() -> &'static str {
        "CssBackgroundColor"
    }
    fn type_token() -> TokenExpected {
        TokenExpected::QuotedString | TokenExpected::Ident
    }
}
