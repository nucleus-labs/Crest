
pub use super::selectors::*;
pub use super::unit::*;

pub trait FromProperty: Sized {
    fn get_name() -> &'static str;
    fn from_property(property_values: Vec<Generic>) -> Result<Self, crate::css::Error>;
}

#[derive(Debug, Clone)]
pub struct Function(pub String, pub Vec<Generic>);
#[derive(Debug, Clone)]
pub struct Number(pub f32, pub Option<Unit>);

#[derive(Debug, Clone)]
pub struct Property(pub String, pub Vec<Generic>);

#[derive(Debug, Clone)]
pub struct PropertyList(pub std::collections::HashMap<String, Vec<Generic>>);

#[derive(Clone, Debug)]
pub struct RuleSet {
    pub selector: Selector,
    pub properties: PropertyList,
}

#[derive(Debug, Clone)]
pub enum Generic {
    Identifier(String),
    Number(Number),
    String(String),
    Color([u8; 3]),
    Function(Function),
}
