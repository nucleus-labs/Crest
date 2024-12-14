#![allow(warnings)]

//! # Crest
//!
//! Crest is [Peacock](https://github.com/nucleus-labs/peacock)'s core library for parsing css files. While Crest is intended for use by Peacock, it is designed to be usable for other projects as well.
//!
//! For more information on Peacock, [click here](https://github.com/nucleus-labs/peacock)!
//!

mod selector;
mod syntax;

pub(crate) mod boo;
pub mod error;
pub(crate) mod source;
pub mod style;
pub mod unit;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub use unit::Unit;

/// The generic implementation for document nodes.
///
/// Used for selector matching and applying style properties
pub trait DocumentNode {
    /* required functions */

    /// the element's namespace
    /// eg: "m" in `<m:math xmlns:m="http://www.w3.org/1998/Math/MathML">...</m:math>`
    fn get_namespace(&self) -> &str;
    fn get_type(&self) -> &str;
    fn get_id(&self) -> &str;
    fn get_classes(&self) -> Box<[&str]>;

    /// inline attributes of a document node. **__First value must be the full, unsplit value!__**
    /// eg: ("rel": ("nofollow noreferrer", ["nofollow", "noreferrer"]))
    fn get_attributes(&self) -> &HashMap<&str, (&str, Box<[&str]>)>;

    fn get_self(&self) -> Arc<RwLock<Self>>;
    fn get_parent(&self) -> Arc<RwLock<Self>>;
    fn get_children(&self) -> Box<[Arc<RwLock<Self>>]>;

    ///
    fn get_inline_style(&self) -> &style::Stylesheet;
    fn apply_stylesheet(&mut self);

    /* provided functions */

    /// Is this element of the provided type?
    /// eg: Is `<div>...</div>` of type `p`?
    ///
    /// css selector reference:
    /// ```css
    /// p { /* ... */ }
    /// ```
    fn match_type(&self, type_name: &str) -> bool {
        self.get_type() == type_name
    }

    /// Does this element have the provided id?
    /// eg: is the id of `<div id="foo">...</div>` equal to "foo"?
    ///
    /// css selector reference:
    /// ```css
    /// div#bar { /* ... */ }
    /// ```
    fn match_id(&self, id: &str) -> bool {
        self.get_id() == id
    }

    /// Does the element have the provided class?
    /// eg: does the element `<div class="foo bar baz">...</div>` contain "bar"?
    ///
    /// css selector reference:
    /// ```css
    /// div.bar { /* ... */ }
    /// ```
    fn match_class(&self, class_name: &str) -> bool {
        self.get_classes().contains(&class_name)
    }

    /// Does the element have the provided classes?
    /// eg: does the element `<div class="foo bar baz">...</div>` contain BOTH "bar" and "baz"?
    ///
    /// css selector reference:
    /// ```css
    /// div.bar.baz { /* ... */ }
    /// ```
    fn match_classes(&self, class_names: Box<[&str]>) -> bool {
        for &class_name in class_names.iter() {
            if !self.match_class(class_name) {
                return false;
            }
        }
        return true;
    }

    /// Does the element have the provided attribute?
    /// eg: does the element `<a href="...">...</a>` have the attribute "href"?
    ///
    /// **CASE-SENSITIVE**
    ///
    /// css selector reference:
    /// ```css
    /// a[href] { /* ... */ }
    /// ```
    fn match_attribute_present(&self, attr_name: &str) -> bool {
        self.get_attributes().contains_key(&attr_name)
    }

    /// Does the element's attribute have this exact match?
    /// eg: does the element `<img alt="A beautiful mountain view">...</img>` have alt text of "A beautiful mountain view"?
    ///
    /// css selector reference:
    /// ```css
    /// img[alt="A beautiful mountain view"] { /* ... */ }
    /// ```
    fn match_attribute_value(
        &self,
        attr_name: &str,
        attr_value: &str,
        case_sensitive: bool,
    ) -> bool {
        if self.match_attribute_present(attr_name) {
            let raw_value = self.get_attributes().get(&attr_name).unwrap().0;
            if case_sensitive {
                return raw_value == attr_value;
            } else {
                return raw_value.eq_ignore_ascii_case(attr_value);
            }
        }
        false
    }

    /// Does the element's attribute contain the provided value?
    /// eg: does `<a href="..." rel="nofollow noreferrer">...</a>`'s rel attribute contain "noreferrer"?
    ///
    /// css selector reference:
    /// ```css
    /// a[rel~="noreferrer"] { /* ... */ }
    /// ```
    fn match_attribute_contains(
        &self,
        attr_name: &str,
        attr_value: &str,
        case_sensitive: bool,
    ) -> bool {
        if self.match_attribute_present(attr_name) {
            let values = &self.get_attributes().get(&attr_name).unwrap().1;
            if case_sensitive && !values.contains(&attr_value) {
                return true;
            } else if !case_sensitive {
                return values.iter().any(|&x| x.eq_ignore_ascii_case(attr_value));
            }
        }
        false
    }

    /// Does the element's attribute start with the provided value?
    /// eg: does `<a href="https://example.com/"></a>`'s href attribute start with "https://"?
    ///
    /// __Case-insensitive matches contain string allocations__.
    ///
    /// css selector reference:
    /// ```css
    /// a[href^="https://"] { /* ... */ }
    /// ```
    fn match_attribute_startswith(
        &self,
        attr_name: &str,
        attr_value_prefix: &str,
        case_sensitive: bool,
    ) -> bool {
        if self.match_attribute_present(attr_name) {
            let attr_raw: &str = self.get_attributes().get(&attr_name).unwrap().0;
            if case_sensitive && attr_raw.starts_with(attr_value_prefix) {
                return true;
            } else if !case_sensitive {
                return attr_raw
                    .to_lowercase()
                    .starts_with(&attr_value_prefix.to_lowercase());
            }
        }
        false
    }

    /// Does the element's attribute start with the provided value (with an added dash)?
    /// eg: does `<p lang="en-US">...</p>`'s lang attribute equal "en" or start with "en-"?
    ///
    /// css selector reference:
    /// ```css
    /// p[lang|="en"] { /* ... */ }
    /// ```
    fn match_attribute_startswith_dash(
        &self,
        attr_name: &str,
        attr_value_prefix: &str,
        case_sensitive: bool,
    ) -> bool {
        if self.match_attribute_value(attr_name, attr_value_prefix, case_sensitive) {
            true
        } else {
            let dashed_prefix = format!("{attr_value_prefix}-");
            self.match_attribute_contains(attr_name, &dashed_prefix, case_sensitive)
        }
    }

    /// Does the element's attribute end with the provided value?
    /// eg: does `<a href="document.pdf">...</a>`'s href attribute end with ".pdf"?
    ///
    /// __Case-insensitive matches contain string allocations__.
    ///
    /// css selector reference:
    /// ```css
    /// a[href$=".pdf"] { /* ... */ }
    /// ```
    fn match_attribute_endswith(
        &self,
        attr_name: &str,
        attr_value_prefix: &str,
        case_sensitive: bool,
    ) -> bool {
        if self.match_attribute_present(attr_name) {
            let attr_raw: &str = self.get_attributes().get(&attr_name).unwrap().0;
            if case_sensitive && attr_raw.starts_with(attr_value_prefix) {
                return true;
            } else if !case_sensitive {
                return attr_raw
                    .to_lowercase()
                    .ends_with(&attr_value_prefix.to_lowercase());
            }
        }
        false
    }

    /// Does the element's raw attribute contain the provided value?
    /// eg: does `<a href="https://example.com/home.htm">...</a>`'s href attribute contain "home"?
    ///
    /// css selector reference:
    /// ```css
    /// a[rel~="noreferrer"] { /* ... */ }
    /// ```
    fn match_attribute_raw_contains(
        &self,
        attr_name: &str,
        attr_value: &str,
        case_sensitive: bool,
    ) -> bool {
        if self.match_attribute_present(attr_name) {
            let attr_raw: &str = self.get_attributes().get(&attr_name).unwrap().0;
            if case_sensitive && attr_raw.contains(&attr_value) {
                return true;
            } else if !case_sensitive {
                return attr_raw.eq_ignore_ascii_case(attr_value);
            }
        }
        false
    }

    fn match_selector(&self, selector: &selector::SelectorNode) -> bool {
        if let Some(namespace) = &selector.namespace {
            if self.get_namespace() != namespace {
                return false;
            }
        }

        if selector.universal {
            return true;
        }

        if let Some(type_name) = &selector.type_name {
            if self.get_type() != type_name {
                return false;
            }
        }

        todo!()
    }

    /// Is the provided node handle the same as the one for self?
    /// A convenience function mostly used for internal identity checks
    fn match_self(&self, other: Arc<RwLock<Self>>) -> bool {
        Arc::ptr_eq(&self.get_self(), &other)
    }

    // fn match_parent(&s)
}

// impl std::str::FromStr for SelectorNode {
//     type Err = error::Error;

//     fn from_str(selector_source: &str) -> Result<Self, Self::Err> {
//         use pest::Parser as _;

//         let parse_result = parse::CssParser::parse(parse::Rule::Selector, selector_source);
//         match parse_result {
//             Ok(pairs) => {
//                 match parse::gen_token(pairs) {
//                     Some(token) => {
//                         let mut node = SelectorNode{
//                             universal: false,
//                             namespace: None,
//                             type_name: None,

//                             id: None,
//                             classes: Vec::new(),
//                             attributes: HashMap::new(),

//                             parent: None,
//                             siblings: Vec::new(),
//                             children: Vec::new(),
//                         };

//                         todo!();

//                         Ok(node)
//                     },
//                     None => Err(error::Error::generic("Failed to identify root node in token stream!")),
//                 }
//             },
//             Err(err) => Err(error::Error::CssParseError(err)),
//         }
//     }
// }

// impl From<SelectorToken> for SelectorNode {
//     fn from(value: SelectorToken) -> Self {
//         assert!(matches!(value.get_rule(), SelectorRule::SELECTOR_SIMPLE));
//         let selector = Self::default();
//         let components = value.get_children().unwrap();

//         for component in components.iter() {
//             match component.get_rule() {
//                 SelectorRule::SELECTOR_SIMPLE_BASIC => {
//                     let basic_component = component.get_children().unwrap()[0];
//                     match basic_component.get_rule() {
//                         SelectorRule::SELECTOR_SIMPLE_BASIC_ID => todo!(),
//                         SelectorRule::SELECTOR_SIMPLE_BASIC_CLASS => todo!(),
//                         SelectorRule::SELECTOR_SIMPLE_BASIC_TYPE => todo!(),
//                         SelectorRule::SELECTOR_SIMPLE_BASIC_UNIVERAL => todo!(),
//                     }
//                 },
//                 SelectorRule::SELECTOR_SIMPLE_PSEUDOCLASS => {
//                     todo!()
//                 },
//                 SelectorRule::SELECTOR_SIMPLE_PSEUDOELEMENT => {
//                     todo!()
//                 },

//                 _ => panic!("Impossible error!")
//             }
//         }

//         selector
//     }
// }
