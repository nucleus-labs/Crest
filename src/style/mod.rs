pub mod prop_validation;
pub mod properties;

use std::collections::HashMap;
use std::sync::Arc;

use crate::boo::Boo;
use crate::error::Error;
use crate::selector::SelectorNode;
use crate::source::{parse_source, SourceInfo, SourceSlice};
use crate::syntax::{CssExpectError, CssParser, CssRule, CssToken, CssTokenTracker};

pub use prop_validation::{CssAttributeValue, CssStyleAttribute, CssValue};
pub use properties::CssStyleProperties;

#[derive(Debug, Clone)]
pub struct AtRule {
    pub(crate) name: String,
    // components: Vec<SourceSlice>,
    // decl_block: HashMap<String, Vec<CssToken>>,
}

#[derive(Debug, Clone, Default)]
pub struct Stylesheet {
    pub at_rules: HashMap<String, AtRule>,
    pub style_rules: Vec<(SelectorNode, CssStyleProperties)>,
}

#[derive(Debug, Clone)]
pub struct CssStyleValueExpector<'a> {
    decl_groups: Vec<&'a CssToken>,
    css_expector: CssTokenTracker<'a>,
    has_errored: bool,
    results: Vec<Result<CssStyleAttribute, CssExpectError>>,
}

impl Stylesheet {
    pub fn from_source(source: &str) -> Result<Self, Error> {
        source.parse::<Self>()
    }

    pub fn from_filepath(filepath: &std::path::Path) -> Result<Self, Error> {
        let source = std::fs::read_to_string(filepath)?;
        source.parse::<Self>()
    }
}

impl<'a> CssStyleValueExpector<'a> {
    pub fn new(declaration_groups_token: &'a CssToken) -> Self {
        assert_eq!(
            declaration_groups_token.get_rule(),
            CssRule::DECLARATION_GROUPS
        );
        let mut decl_groups: Vec<&'a CssToken> = declaration_groups_token
            .get_children()
            .unwrap()
            .iter()
            .collect();

        let decl_group = decl_groups.remove(0);
        let decl_values = &decl_group.get_children().unwrap()[0];
        let decl_value = &decl_values.get_children().unwrap()[0];
        let css_expector = CssTokenTracker::new(decl_value).unwrap();

        Self {
            decl_groups,
            css_expector,
            has_errored: false,
            results: Vec::new(),
        }
    }

    pub fn expect<T: prop_validation::CssValue>(&mut self) -> &mut Self
    where
        CssStyleAttribute: From<prop_validation::CssAttributeValue<T>>,
    {
        if self.has_errored {
            return self;
        }

        if self.css_expector.is_empty() {
            if self.decl_groups.is_empty() {
                self.has_errored = true;
                self.results.push(Err(CssExpectError::TooFewTokens(
                    "DECLARATION_VALUES".into(),
                )));
                return self;
            } else {
                let decl_group = self.decl_groups.remove(0);
                let decl_values = &decl_group.get_children().unwrap()[0];
                let decl_value = &decl_values.get_children().unwrap()[0];
                self.css_expector = CssTokenTracker::new(decl_value).unwrap();
            }
        }

        match T::parse(&self.css_expector) {
            Ok(attr_val) => self.results.push(Ok(attr_val.into())),
            Err(err) => {
                self.has_errored = true;
                self.results.push(Err(CssExpectError::InvalidAttributeValue(
                    self.css_expector.get_location().unwrap().start,
                    err,
                )));
            }
        }

        self
    }

    pub fn optional<T: prop_validation::CssValue>(&mut self) -> &mut Self
    where
        CssStyleAttribute: From<prop_validation::CssAttributeValue<T>>,
    {
        if self.has_errored {
            return self;
        }

        if self.css_expector.is_empty() {
            if self.decl_groups.is_empty() {
                return self;
            } else {
                let decl_group = self.decl_groups.remove(0);
                let decl_values = &decl_group.get_children().unwrap()[0];
                self.css_expector = CssTokenTracker::new(decl_values).unwrap();
            }
        }

        println!("^ {}", self.css_expector.peek().unwrap().get_rule());
        match T::parse(&self.css_expector) {
            Ok(attr_val) => self.results.push(Ok(attr_val.into())),
            Err(err) => (),
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
    type Err = Error;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match parse_source::<CssRule, CssParser>(SourceInfo::new(source.into()), CssRule::CSS) {
            Ok(css_token) => {
                let expector = CssTokenTracker::new(&css_token).unwrap();
                assert_eq!(expector.peek().unwrap().get_rule(), CssRule::STYLESHEET);

                let stylesheet = expector.expect_stylesheet()?;
                Ok(stylesheet)
            }
            Err(err) => Err(Error::CssError(err.into())),
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
        for (selector, props) in self.style_rules.iter() {
            writeln!(f, "{selector} {{\n{props}}}\n");
        }

        Ok(())
    }
}
