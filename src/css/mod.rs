
pub mod error;
pub mod types;

pub use error::Error;
use crate::parse::{StackInfo, Rule};

#[derive(Debug, Clone)]
pub enum CssTokenValue {
    Leaf(String),
    Internal(Vec<CssToken>)
}

#[derive(Debug, Clone)]
pub struct CssToken {
    value: CssTokenValue,
    rule: Rule,
}

// ============== IMPL ==============

impl CssToken {
    pub fn is_leaf(&self) -> bool {
        matches!(self.value, CssTokenValue::Leaf(_))
    }

    pub fn get_children(&self) -> Result<Vec<CssToken>, ()> {
        match &self.value {
            CssTokenValue::Leaf(_) => Err(()),
            CssTokenValue::Internal(vec) => Ok(vec.clone()),
        }
    }

    pub fn get_source(&self) -> Result<String, ()> {
        match &self.value {
            CssTokenValue::Leaf(source) => Ok(source.clone()),
            CssTokenValue::Internal(vec) => Err(()),
        }
    }

    pub fn flatten(&self, into: &mut Vec<CssToken>) {
        match &self.value {
            CssTokenValue::Leaf(_) => into.push(self.clone()),
            CssTokenValue::Internal(vec) => {
                for child in self.get_children().unwrap().iter() {
                    child.flatten(into);
                }
            },
        }
    }

    pub fn build_string(&self) -> String {
        let mut all: Vec<_> = Vec::new();
        self.flatten(&mut all);
        all.iter().map(|token| token.get_source().unwrap()).collect::<Vec<_>>().join(" -> ")
    }
}

impl<'a> From<StackInfo<'a>> for CssToken {
    fn from(value: StackInfo<'a>) -> Self {
        if value.children.len() > 0 {
            let mut children: Vec<CssToken> = Vec::new();
            for child in value.children.iter() {
                children.push(CssToken::from(child.clone()));
            }
            
            Self{
                value: CssTokenValue::Internal(children),
                rule: value.rule
            }
        }
        else {
            Self{
                value: CssTokenValue::Leaf(value.positions.0.span(&value.positions.1.unwrap()).as_str().to_string()),
                rule: value.rule,
            }
        }
    }
}
