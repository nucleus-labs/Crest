
mod base;

mod display;
mod position;
mod visibility;
mod unit;
mod selectors;
mod layout;

mod font;
mod color;

use std::collections::HashMap;

pub use base::*;

pub use display::*;
pub use position::*;
pub use visibility::*;
pub use layout::*;

pub use font::*;
pub use color::*;

pub trait Styleable {
    fn get_name(&self) -> String;
    fn get_id(&self) -> Option<String>;
    fn get_classes(&self) -> Vec<String>;

}

#[derive(Debug, Clone)]
pub struct Stylesheet(pub Vec<RuleSet>);

// ============== IMPL ==============

impl Stylesheet {
    pub fn new(init: Option<Vec<RuleSet>>) -> Self {
        Self(Vec::new())
    }

    pub fn parse(&mut self, source: String) {

    }

    pub fn join(&mut self, other: Self) {
        let mut to_empty = other.0;
        self.0.append(&mut to_empty);
    }

    pub fn find_match(&self, other: impl Styleable) -> Option<HashMap<String, Vec<Generic>>> {
        let mut ruleset_match: Option<RuleSet> = None;

        for ruleset in self.0.iter() {
            if ruleset.selector.matches(&other) {
                if ruleset_match.is_none() {
                    ruleset_match = Some(ruleset.clone());
                }
                else if ruleset.selector.specificity() > ruleset_match.clone().unwrap().selector.specificity() {
                    ruleset_match = Some(ruleset.clone());
                }
            }
        }

        match ruleset_match {
            Some(_) => todo!(),
            None => todo!(),
        }
    }
}
