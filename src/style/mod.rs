pub mod parse_attr;
pub mod properties;

use std::collections::HashMap;
use std::sync::Arc;

pub use properties::CssStyleProperties;

use crate::parse::expectors::{CssTokenTracker, ExpectError};
use crate::parse::{boo::Boo, parse_css, CssToken};
use crate::parse::{CssRule, SourceSlice};

pub struct AtRule {
    pub(crate) name: String,
    // components: Vec<SourceSlice>,
    // decl_block: HashMap<String, Vec<CssToken>>,
}

pub struct Stylesheet {
    pub at_rules: HashMap<String, AtRule>,
    pub style_rules: Vec<(crate::SelectorNode, CssStyleProperties)>,
}

impl std::str::FromStr for Stylesheet {
    type Err = crate::error::Error;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match parse_css(source) {
            Ok(css_token) => {
                let expector = CssTokenTracker::new(Boo::Owned(vec![css_token]));
                assert_eq!(expector.peek().unwrap().get_rule(), CssRule::CSS);
                let stylesheet = expector.expect_css_stylesheet().unwrap();

                let mut at_rules: HashMap<String, AtRule> = HashMap::new();
                let mut style_rules: Vec<(crate::SelectorNode, CssStyleProperties)> = Vec::new();

                while stylesheet.len() > 0 {
                    if let Ok(style_rule) = stylesheet.expect_style_rule() {
                        for selector in style_rule.0.into_iter() {
                            style_rules.push((selector.clone(), style_rule.1.clone()));
                        }
                    } else if let Ok(at_rule) = stylesheet.expect_at_rule() {
                        at_rules.insert(at_rule.name.clone(), at_rule);
                    }
                }

                Ok(Self {
                    at_rules,
                    style_rules,
                })
            }
            Err(err) => Err(err),
        }
    }
}

impl std::fmt::Display for AtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, r#"@{} "at-rules aren't (yet) implemented";"#, self.name)
    }
}

impl std::fmt::Display for Stylesheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, at_rule) in self.at_rules.iter() {
            writeln!(f, "{at_rule}");
        }

        for (selector, props) in self.style_rules.iter() {
            writeln!(f, "{selector} {{\n{props}\n}}\n");
        }

        Ok(())
    }
}
