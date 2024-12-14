pub mod parse_attr;
pub mod properties;

use std::collections::HashMap;
use std::sync::Arc;

pub use properties::CssStyleProperties;

use crate::boo::Boo;
use crate::selector::SelectorNode;
use crate::source::{parse_source, SourceSlice};
use crate::syntax::{CssExpectError, CssParser, CssRule, CssToken, CssTokenTracker};
use parse_attr::CssStyleAttribute;

pub struct AtRule {
    pub(crate) name: String,
    // components: Vec<SourceSlice>,
    // decl_block: HashMap<String, Vec<CssToken>>,
}

#[derive(Clone, Debug)]
pub struct CssStyleValueExpector<'a> {
    css_expector: &'a CssTokenTracker<'a>,
    has_errored: bool,
    results: Vec<Result<CssStyleAttribute, CssExpectError>>,
}

pub struct Stylesheet {
    pub at_rules: HashMap<String, AtRule>,
    pub style_rules: Vec<(SelectorNode, CssStyleProperties)>,
}

impl<'a> CssStyleValueExpector<'a> {
    pub fn new(css_expector: &'a CssTokenTracker) -> Self {
        Self {
            css_expector,
            has_errored: false,
            results: Vec::new(),
        }
    }

    pub fn expect<T: parse_attr::CssValue>(&mut self) -> &mut Self
    where
        CssStyleAttribute: From<parse_attr::CssAttributeValue<T>>,
    {
        if self.has_errored {
            return self;
        }

        match T::parse(self.css_expector) {
            Ok(attr_val) => self.results.push(Ok(attr_val.into())),
            Err(err) => {
                self.has_errored = true;
                self.results
                    .push(Err(CssExpectError::InvalidAttributeValue(err)));
            }
        }

        self
    }

    pub fn resolve(&mut self) -> Result<Box<[CssStyleAttribute]>, CssExpectError> {
        if self.has_errored {
            Err(self.results.pop().unwrap().unwrap_err())
        } else {
            let mut results = Vec::new();
            results.append(&mut self.results);

            let mut attrs: Vec<CssStyleAttribute> =
                results.into_iter().map(|x| x.unwrap()).collect::<Vec<_>>();

            Ok(attrs.into_boxed_slice())
        }
    }
}

impl std::str::FromStr for Stylesheet {
    type Err = crate::error::Error;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match parse_source::<CssRule, CssParser>(source, CssRule::CSS) {
            Ok(css_token) => {
                let expector = CssTokenTracker::new(&css_token);
                assert_eq!(expector.peek().unwrap().get_rule(), CssRule::STYLESHEET);

                let stylesheet = expector.expect_stylesheet()?;
                Ok(stylesheet)
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
            writeln!(f, "// .");
            writeln!(f, "{at_rule}");
        }

        for (selector, props) in self.style_rules.iter() {
            writeln!(f, "// .");
            writeln!(f, "{selector} {{\n{props}\n}}\n");
        }

        Ok(())
    }
}
