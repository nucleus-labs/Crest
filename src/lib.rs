#![doc = include_str!("../README.md")]
#![allow(warnings)]

mod selector;

pub(crate) mod boo;

pub mod error;
pub mod source;
pub mod style;
pub mod syntax;
pub mod unit;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub use selector::{SelectorNode, SelectorNodeType};
pub use source::SourceInfo;
pub use style::{CssAttributeValue, CssStyleAttribute, CssStyleProperties, CssValue, Stylesheet};
pub use unit::Unit;

pub type MatchResult = Option<()>;

/// The generic implementation for document nodes.
///
/// Used for selector matching and applying style properties
pub trait DomElement {
    // required functions

    // /// the element's namespace
    // /// eg: "m" in `<m:math xmlns:m="http://www.w3.org/1998/Math/MathML">...</m:math>`
    // fn get_namespace(&self) -> &str;
    // fn get_type(&self) -> &str;

    // /// inline attributes of a document node. **__First value must be the full, unsplit value!__**
    // /// eg: ("rel": ("nofollow noreferrer", ["nofollow", "noreferrer"]))
    // fn get_attributes(&self) -> &HashMap<String, &str>;

    // fn get_self(&self) -> Arc<RwLock<Self>>;
    // fn get_parent(&self) -> Arc<RwLock<Self>>;
    // fn get_children(&self) -> Box<[Arc<RwLock<Self>>]>;

    /// <button style="color: red;" />
    fn get_inline_style(&self) -> style::CssStyleProperties;

    /// This is for applying style properties to the current node immediately
    fn apply_style_properties(&mut self);

    // optional functions

    // /// whether the element has a specific state (such as "hover", "disabled", etc)
    // /// default implementation always returns false
    // fn match_state(&self, state: &str) -> MatchResult {
    //     None
    // }

    // /// pseudo-elements are for referencing a specific part of an element.
    // ///
    // /// eg: using `::selection` to get a reference to the highlighted selection of
    // /// an element.
    // fn match_component(&self, state: &str) -> MatchResult {
    //     None
    // }

    // provided functions

    // /// Does this element have the provided namespace?
    // /// eg: Does `<m:math xmlns:m="http://www.w3.org/1998/Math/MathML">...</m:math>` have namespace `m`?
    // ///
    // /// css selector reference:
    // /// ```css
    // /// @namespace m url('http://www.w3.org/1998/Math/MathML');
    // ///
    // /// m|* { /* ... */ }
    // /// ```
    // fn match_namespace(&self, namespace: &str) -> bool {
    //     self.get_namespace() == namespace
    // }

    // /// Is this element of the provided type?
    // /// eg: Is `<div>...</div>` of type `p`?
    // ///
    // /// css selector reference:
    // /// ```css
    // /// p { /* ... */ }
    // /// ```
    // fn match_type(&self, type_name: &str) -> bool {
    //     self.get_type() == type_name
    // }

    // /// Does this element have the provided id?
    // /// eg: is the id of `<div id="foo">...</div>` equal to "foo"?
    // ///
    // /// css selector reference:
    // /// ```css
    // /// div#bar { /* ... */ }
    // /// ```
    // fn match_id(&self, id: &str) -> bool {
    //     self.get_attributes()["id"] == id
    // }

    // /// Does the element have the provided class?
    // /// eg: does the element `<div class="foo bar baz">...</div>` contain "bar"?
    // ///
    // /// css selector reference:
    // /// ```css
    // /// div.bar { /* ... */ }
    // /// ```
    // fn match_class(&self, class_name: &str) -> bool {
    //     self.get_attributes()["class"]
    //         .split_whitespace()
    //         .any(|class| class == class_name)
    // }

    // /// Does the element have the provided classes?
    // /// eg: does the element `<div class="foo bar baz">...</div>` contain BOTH "bar" and "baz"?
    // ///
    // /// css selector reference:
    // /// ```css
    // /// div.bar.baz { /* ... */ }
    // /// ```
    // fn match_classes(&self, class_names: &Vec<String>) -> bool {
    //     let classes = self.get_attributes()["class"]
    //         .split_whitespace()
    //         .collect::<Vec<_>>();
    //     class_names
    //         .iter()
    //         .all(|class_name| classes.contains(&class_name.as_str()))
    // }

    // /// Does the element have the provided attribute?
    // /// eg: does the element `<a href="...">...</a>` have the attribute "href"?
    // ///
    // /// **CASE-SENSITIVE**
    // ///
    // /// css selector reference:
    // /// ```css
    // /// a[href] { /* ... */ }
    // /// ```
    // fn match_attribute_present(&self, attr_name: &str) -> bool {
    //     self.get_attributes().contains_key(attr_name)
    // }

    // /// Does the element's attribute have this exact match?
    // /// eg: does the element `<img alt="A beautiful mountain view">...</img>` have alt text of "A beautiful mountain view"?
    // ///
    // /// css selector reference:
    // /// ```css
    // /// img[alt="A beautiful mountain view"] { /* ... */ }
    // /// ```
    // fn match_attribute_value(
    //     &self,
    //     attr_name: &str,
    //     attr_value: &str,
    //     case_sensitive: bool,
    // ) -> bool {
    //     if self.match_attribute_present(attr_name) {
    //         let raw_value = *self.get_attributes().get(attr_name).unwrap();
    //         if case_sensitive {
    //             return raw_value == attr_value;
    //         } else {
    //             return raw_value.eq_ignore_ascii_case(attr_value);
    //         }
    //     }
    //     false
    // }

    // /// Does the element's attribute contain the provided value?
    // /// eg: does `<a href="..." rel="nofollow noreferrer">...</a>`'s rel attribute contain "noreferrer"?
    // ///
    // /// css selector reference:
    // /// ```css
    // /// a[rel~="noreferrer"] { /* ... */ }
    // /// ```
    // fn match_attribute_contains(
    //     &self,
    //     attr_name: &str,
    //     attr_value: &str,
    //     case_sensitive: bool,
    // ) -> bool {
    //     if self.match_attribute_present(attr_name) {
    //         let mut own_attrs = self
    //             .get_attributes()
    //             .get(attr_name)
    //             .unwrap()
    //             .split_whitespace();
    //         if case_sensitive {
    //             return own_attrs.any(|attribute_value| attribute_value == attr_value);
    //         } else {
    //             return own_attrs.any(|x| x.eq_ignore_ascii_case(attr_value));
    //         }
    //     }
    //     false
    // }

    // /// Does the element's attribute start with the provided value?
    // /// eg: does `<a href="https://example.com/"></a>`'s href attribute start with "https://"?
    // ///
    // /// __Case-insensitive matches contain string allocations__.
    // ///
    // /// css selector reference:
    // /// ```css
    // /// a[href^="https://"] { /* ... */ }
    // /// ```
    // fn match_attribute_startswith(
    //     &self,
    //     attr_name: &str,
    //     attr_value_prefix: &str,
    //     case_sensitive: bool,
    // ) -> bool {
    //     if self.match_attribute_present(attr_name) {
    //         let attr_raw: &str = self.get_attributes().get(attr_name).unwrap();
    //         if case_sensitive {
    //             return attr_raw.starts_with(attr_value_prefix);
    //         } else {
    //             return attr_raw
    //                 .to_lowercase()
    //                 .starts_with(&attr_value_prefix.to_lowercase());
    //         }
    //     }
    //     false
    // }

    // /// Does the element's attribute start with the provided value (with an added dash)?
    // /// eg: does `<p lang="en-US">...</p>`'s lang attribute equal "en" or start with "en-"?
    // ///
    // /// css selector reference:
    // /// ```css
    // /// p[lang|="en"] { /* ... */ }
    // /// ```
    // fn match_attribute_startswith_dash(
    //     &self,
    //     attr_name: &str,
    //     attr_value_prefix: &str,
    //     case_sensitive: bool,
    // ) -> bool {
    //     if self.match_attribute_value(attr_name, attr_value_prefix, case_sensitive) {
    //         true
    //     } else if self.match_attribute_present(attr_name) {
    //         let prefix_len = attr_value_prefix.len();
    //         (&self.get_attributes()[attr_name][prefix_len + 1..prefix_len + 2]) == "-"
    //     } else {
    //         false
    //     }
    // }

    // /// Does the element's attribute end with the provided value?
    // /// eg: does `<a href="document.pdf">...</a>`'s href attribute end with ".pdf"?
    // ///
    // /// __Case-insensitive matches contain string allocations__.
    // ///
    // /// css selector reference:
    // /// ```css
    // /// a[href$=".pdf"] { /* ... */ }
    // /// ```
    // fn match_attribute_endswith(
    //     &self,
    //     attr_name: &str,
    //     attr_value_prefix: &str,
    //     case_sensitive: bool,
    // ) -> bool {
    //     if self.match_attribute_present(attr_name) {
    //         let attr_raw: &str = self.get_attributes().get(attr_name).unwrap();
    //         if case_sensitive {
    //             return attr_raw.starts_with(attr_value_prefix);
    //         } else {
    //             return attr_raw
    //                 .to_lowercase()
    //                 .ends_with(&attr_value_prefix.to_lowercase());
    //         }
    //     }
    //     false
    // }

    // /// Does the element's raw attribute contain the provided value?
    // /// eg: does `<a href="https://example.com/home.htm">...</a>`'s href attribute contain "home"?
    // ///
    // /// css selector reference:
    // /// ```css
    // /// a[rel~="noreferrer"] { /* ... */ }
    // /// ```
    // fn match_attribute_raw_contains(
    //     &self,
    //     attr_name: &str,
    //     attr_value: &str,
    //     case_sensitive: bool,
    // ) -> bool {
    //     if self.match_attribute_present(attr_name) {
    //         let attr_raw: &str = self.get_attributes().get(attr_name).unwrap();
    //         if case_sensitive {
    //             return attr_raw.contains(&attr_value);
    //         } else {
    //             return attr_raw.to_lowercase().contains(&attr_value.to_lowercase());
    //         }
    //     }
    //     false
    // }

    // fn match_selector(&self, selector: &selector::SelectorNode) -> MatchResult {
    //     selector.iter().all(|selector_type| match selector_type {
    //         SelectorNodeType::Namespace(ns) => self.match_namespace(ns),
    //         SelectorNodeType::Universal => true,
    //         SelectorNodeType::TypeName(type_name) => self.match_type(type_name),
    //         SelectorNodeType::Id(id) => self.match_id(id),
    //         SelectorNodeType::Class(class) => self.match_class(class),
    //         SelectorNodeType::Attribute {
    //             name,
    //             attr_matcher,
    //             case_sensitive,
    //         } => match attr_matcher {
    //             selector::SelectorAttributeType::Present => self.match_attribute_present(name),
    //             selector::SelectorAttributeType::ExactMatch(against) => {
    //                 self.match_attribute_value(name, against, *case_sensitive)
    //             }
    //             selector::SelectorAttributeType::ListContains(against) => {
    //                 self.match_attribute_contains(name, against, *case_sensitive)
    //             }
    //             selector::SelectorAttributeType::StartsWith(against) => {
    //                 self.match_attribute_startswith(name, against, *case_sensitive)
    //             }
    //             selector::SelectorAttributeType::StartsWithDashed(against) => {
    //                 self.match_attribute_startswith_dash(name, against, *case_sensitive)
    //             }
    //             selector::SelectorAttributeType::Endswith(against) => {
    //                 self.match_attribute_endswith(name, against, *case_sensitive)
    //             }
    //             selector::SelectorAttributeType::RawContains(against) => {
    //                 self.match_attribute_raw_contains(name, against, *case_sensitive)
    //             }
    //         },
    //         SelectorNodeType::NextSibling(selector_node) => {
    //             self.match_next_sibling(selector_node.as_ref())
    //         }
    //         SelectorNodeType::SubsequentSibling(selector_node) => {
    //             self.match_subsequent_siblings(selector_node.as_ref())
    //         }
    //         SelectorNodeType::Child(selector_node) => self.match_child(selector_node.as_ref()),
    //         SelectorNodeType::Descendent(selector_node) => {
    //             self.match_descendent(selector_node.as_ref())
    //         }
    //         SelectorNodeType::PseudoClass(psclass) => self.match_state(psclass as &str).is_some(),
    //         SelectorNodeType::PseudoElement(pselement_name, pselement_arguments) => todo!(),
    //     });

    //     None
    // }

    // /// Is the provided node handle the same as the one for self?
    // /// A convenience function mostly used for internal identity checks
    // fn match_self(&self, other: Arc<RwLock<Self>>) -> bool {
    //     Arc::ptr_eq(&self.get_self(), &other)
    // }

    // fn get_siblings(&self) -> (Box<[Arc<RwLock<Self>>]>, Box<[Arc<RwLock<Self>>]>) {
    //     let mut before: Vec<Arc<RwLock<Self>>> = Vec::new();
    //     let mut after: Vec<Arc<RwLock<Self>>> = Vec::new();
    //     let mut found_self = false;
    //     for sibling in self.get_parent().read().unwrap().get_children().iter() {
    //         if self.match_self(sibling.clone()) {
    //             found_self = true;
    //         } else if !found_self {
    //             before.push(sibling.clone());
    //         } else {
    //             after.push(sibling.clone());
    //         }
    //     }

    //     (before.into_boxed_slice(), after.into_boxed_slice())
    // }

    // fn get_next_sibling(&self) -> Option<Arc<RwLock<Self>>> {
    //     let (prev, subsequent) = self.get_siblings();
    //     if !subsequent.is_empty() {
    //         Some(subsequent[0].clone())
    //     } else {
    //         None
    //     }
    // }

    // fn get_subsequent_siblings(&self) -> Box<[Arc<RwLock<Self>>]> {
    //     self.get_siblings().1
    // }

    // fn get_descendents(&self) -> Box<[Arc<RwLock<Self>>]> {
    //     let mut descendants = Vec::new();

    //     fn collect_descendants<T: DocumentNode>(
    //         node: &Arc<RwLock<T>>,
    //         descendants: &mut Vec<Arc<RwLock<T>>>,
    //     ) {
    //         for child in node.read().unwrap().get_children().iter() {
    //             descendants.push(child.clone());
    //             collect_descendants(child, descendants);
    //         }
    //     }

    //     collect_descendants(&self.get_self(), &mut descendants);

    //     descendants.into_boxed_slice()
    // }

    // fn match_next_sibling(&self, selector: &SelectorNode) -> bool {
    //     if let Some(sibling) = self.get_next_sibling() {
    //         sibling.read().unwrap().match_selector(selector).is_some()
    //     } else {
    //         false
    //     }
    // }

    // fn match_subsequent_siblings(&self, selector: &SelectorNode) -> bool {
    //     for sibling in self.get_subsequent_siblings().iter() {
    //         if sibling.read().unwrap().match_selector(selector).is_some() {
    //             return true;
    //         }
    //     }

    //     false
    // }

    // fn match_child(&self, selector: &SelectorNode) -> bool {
    //     for child in self.get_children().iter() {
    //         if child.read().unwrap().match_selector(selector).is_some() {
    //             return true;
    //         }
    //     }

    //     false
    // }

    // fn match_descendent(&self, selector: &SelectorNode) -> bool {
    //     for descendent in self.get_descendents().iter() {
    //         if descendent
    //             .read()
    //             .unwrap()
    //             .match_selector(selector)
    //             .is_some()
    //         {
    //             return true;
    //         }
    //     }

    //     false
    // }
}
