// use std::sync::{Arc, Weak, RwLock};
// use std::collections::HashMap;

// use peacock_crest as crest;

// use peacock_crest::style::prop_validation::types::*;
// use peacock_crest::syntax::{CssParser, CssRule};
// use peacock_crest::source::parse_source;
use peacock_crest::style::Stylesheet;
// use peacock_crest::{CssAttributeValue, CssStyleProperties, CssValue};
// use peacock_crest::unit::*;

fn main() {
    let css = r#"main {}"#;

    println!("====================================\n{css}\n====================================");
    let sheet: Stylesheet = css.parse().expect("Failed to parse css");
    println!("{sheet}\n====================================");

    let css = r#"div.main {
    width: 50pt;
    height: 100px;
    test-prop: auto;
}"#;

    println!("{css}\n====================================");
    let sheet: Stylesheet = css.parse().expect("Failed to parse css");
    println!("{sheet}\n====================================");

    // let decl = "padding: 0; width: 48em; background-color: white; width: 56px; padding: 5px;";
    // let props: CssStyleProperties = decl.parse().unwrap();

    // let prop_names = props.iter()
    //     .map(|prop| prop.0.get_prop_name())
    //     .collect::<Vec<String>>();

    // for prop_name in prop_names.iter() {
    //     match props.eval_prop(prop_name).unwrap() {
    //         peacock_crest::style::properties::CssStyleProperty::BackgroundColor(css_attribute_value)
    //             => assert!(matches!(css_attribute_value, CssAttributeValue::<CssBackgroundColor>::Keyword(<CssBackgroundColor as CssValue>::Keyword::White))),
    //         peacock_crest::style::properties::CssStyleProperty::Width(css_attribute_value)
    //             => assert!(matches!(css_attribute_value, CssAttributeValue::<CssWidth>::Value(Unit::Dimension(Dimension::Length(Length::Px(56f32)))))),
    //         peacock_crest::style::properties::CssStyleProperty::Padding(
    //             css_attribute_value,
    //             css_attribute_value1,
    //             css_attribute_value2,
    //             css_attribute_value3
    //         ) => {
    //             assert!(matches!(css_attribute_value, CssAttributeValue::<CssPadding>::Value(Unit::Dimension(Dimension::Length(Length::Px(5f32))))));
    //             assert!(matches!(css_attribute_value1, CssAttributeValue::<CssPadding>::Value(Unit::Dimension(Dimension::Length(Length::Px(5f32))))));
    //             assert!(matches!(css_attribute_value2, CssAttributeValue::<CssPadding>::Value(Unit::Dimension(Dimension::Length(Length::Px(5f32))))));
    //             assert!(matches!(css_attribute_value3, CssAttributeValue::<CssPadding>::Value(Unit::Dimension(Dimension::Length(Length::Px(5f32))))));
    //         },
    //         _ => (),
    //     }
    // }
}
