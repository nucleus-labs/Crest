use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    CssParseError(pest::error::Error<crate::parse::CssRule>),
    #[from]
    SelectorParseError(pest::error::Error<crate::parse::SelectorRule>),

    Generic(String),
}

impl Error {
    pub fn generic(val: impl std::fmt::Display) -> Self {
        Self::Generic(val.to_string())
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
