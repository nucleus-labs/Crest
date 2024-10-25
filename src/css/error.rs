
use super::types::Generic;

#[derive(Debug, Clone)]
pub enum Error {
    InvalidValueType(Generic, Generic),
    IncorrectPropertyName(String, String),
    ParseError(Vec<strum::ParseError>),
    StructureMismatch,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
