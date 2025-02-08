// use std::sync::{Arc, Weak, RwLock};
// use std::collections::HashMap;

// use peacock_crest as crest;

use peacock_crest::style::prop_validation::types::*;
use peacock_crest::syntax::{CssParser, CssRule};
use peacock_crest::source::parse_source;
use peacock_crest::style::Stylesheet;
use peacock_crest::{CssAttributeValue, CssValue, SourceInfo};
use peacock_crest::unit::*;

// struct TreeNode {
//     attributes: HashMap<&str, (&str, Vec<&str>)>,

//     inline_props: crest::style::CssStyleProperties,
//     applied_properties: crest::style::CssStyleProperties,

//     handle: Arc<RwLock<Self>>,
//     parent: Weak<RwLock<Self>>,
//     children: Vec<Arc<RwLock<Self>>>,
// }

fn main() {
//     let css = r#"main {}"#;

//     println!("====================================\n{css}\n====================================");
//     let sheet: crest::style::Stylesheet = css.parse().expect("Failed to parse css");
//     println!("{sheet}\n====================================");

//     let css = r#"div.main {
//     width: 50pt;
//     height: 100px;
//     test-prop: auto;
// }"#;

//     println!("{css}\n====================================");
//     let sheet: crest::style::Stylesheet = css.parse().expect("Failed to parse css");
//     println!("{sheet}\n====================================");

    let decl = "* { padding: 0; width: 48em; background-color: white; }";
    let stylesheet: Stylesheet = decl.parse().unwrap();
    for (_, props) in stylesheet.style_rules.iter() {
        for (prop, _) in props.0.iter() {
            println!("-> {prop:?}");
            match prop {
                peacock_crest::style::properties::CssStyleProperty::BackgroundColor(css_attribute_value)
                    => assert!(matches!(css_attribute_value, CssAttributeValue::<CssBackgroundColor>::Keyword(<CssBackgroundColor as CssValue>::Keyword::White))),
                peacock_crest::style::properties::CssStyleProperty::Width(css_attribute_value)
                    => assert!(matches!(css_attribute_value, CssAttributeValue::<CssWidth>::Value(Unit::Dimension(Dimension::Length(Length::Em(48f32)))))),
                peacock_crest::style::properties::CssStyleProperty::Padding(
                    css_attribute_value,
                    css_attribute_value1,
                    css_attribute_value2,
                    css_attribute_value3
                ) => {
                    assert!(matches!(css_attribute_value, CssAttributeValue::<CssPadding>::Value(Unit::Number(0f32))));
                    assert!(matches!(css_attribute_value1, CssAttributeValue::<CssPadding>::Value(Unit::Number(0f32))));
                    assert!(matches!(css_attribute_value2, CssAttributeValue::<CssPadding>::Value(Unit::Number(0f32))));
                    assert!(matches!(css_attribute_value3, CssAttributeValue::<CssPadding>::Value(Unit::Number(0f32))));
                },
                _ => (),
            }
        }
    }
}
