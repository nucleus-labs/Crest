use super::{CssValue, TokenExpected};
use crate::Unit;

#[derive(Debug, Clone)]
pub struct CssFontFamily(Unit);

#[derive(Debug, Clone, strum_macros::EnumString, strum_macros::Display)]
pub enum KeywordFontFamily {
    #[strum(to_string = "serif", serialize = "serif")]
    Serif,
    #[strum(to_string = "sans-serif", serialize = "sans-serif")]
    SansSerif,
    #[strum(to_string = "monospace", serialize = "monospace")]
    Monospace,
    #[strum(to_string = "cursive", serialize = "cursive")]
    Cursive,
    #[strum(to_string = "fantasy", serialize = "fantasy")]
    Fantasy,
    #[strum(to_string = "system-ui", serialize = "system-ui")]
    SystemUI,
    #[strum(to_string = "ui-serif", serialize = "ui-serif")]
    UISerif,
    #[strum(to_string = "ui-sans-serif", serialize = "ui-sans-serif")]
    UISansSerif,
    #[strum(to_string = "ui-monospace", serialize = "ui-monospace")]
    UIMonospace,
    #[strum(to_string = "ui-rounded", serialize = "ui-rounded")]
    UIRounded,
    #[strum(to_string = "math", serialize = "math")]
    Math,
    #[strum(to_string = "emoji", serialize = "emoji")]
    Emoji,
    #[strum(to_string = "fangsong", serialize = "fangsong")]
    Fangsong,
}

#[derive(Debug, Clone)]
pub struct CssFontSize(Unit);

#[derive(Debug, Clone, strum_macros::EnumString, strum_macros::Display)]
pub enum KeywordFontSize {
    #[strum(to_string = "xx-small", serialize = "xx-small")]
    XXSmall,
    #[strum(to_string = "x-small", serialize = "x-small")]
    XSmall,
    #[strum(to_string = "small", serialize = "small")]
    Small,
    #[strum(to_string = "medium", serialize = "medium")]
    Medium,
    #[strum(to_string = "large", serialize = "large")]
    Large,
    #[strum(to_string = "x-large", serialize = "x-large")]
    XLarge,
    #[strum(to_string = "xx-large", serialize = "xx-large")]
    XXLarge,
    #[strum(to_string = "xxx-large", serialize = "xxx-large")]
    XXXLarge,
}

impl From<Unit> for CssFontFamily {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl From<Unit> for CssFontSize {
    fn from(value: Unit) -> Self {
        Self(value)
    }
}

impl Into<Unit> for CssFontFamily {
    fn into(self) -> Unit {
        self.0
    }
}

impl Into<Unit> for CssFontSize {
    fn into(self) -> Unit {
        self.0
    }
}

impl CssValue for CssFontFamily {
    type Keyword = KeywordFontFamily;

    fn type_name() -> &'static str {
        "CssFontFamily"
    }
    fn type_token() -> TokenExpected {
        TokenExpected::QuotedString | TokenExpected::Ident
    }
}

impl CssValue for CssFontSize {
    type Keyword = KeywordFontSize;

    fn type_name() -> &'static str {
        "CssFontSize"
    }
    fn type_token() -> TokenExpected {
        TokenExpected::Dimension | TokenExpected::Ident | TokenExpected::Percentage
    }
}
