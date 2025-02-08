use peacock_crest::style::prop_validation::types::*;
use peacock_crest::syntax::{CssParser, CssRule};
use peacock_crest::source::parse_source;
use peacock_crest::style::Stylesheet;
use peacock_crest::{CssAttributeValue, CssStyleProperties, CssValue, SourceInfo};
use peacock_crest::unit::*;

#[test]
fn token_gen() {
    let css = "\nmain {}";
    let source_info = SourceInfo::new(css.into());
    parse_source::<CssRule, CssParser>(source_info, CssRule::CSS).unwrap();
}

#[test]
fn basic() {
    let css = "* { font: 10px/1 Verdana, sans-serif; margin: 1.5em; border: .5em solid black; padding: 0; width: 48em; background-color: white; }";
    css.parse::<Stylesheet>().unwrap(); 
}

#[test]
fn basic_display() {
    let css = "* { font: 10px/1 Verdana, sans-serif; margin: 1.5em; border: .5em solid black; padding: 0; width: 48em; background-color: white; }";
    css.parse::<Stylesheet>().unwrap().to_string(); 
}

#[test]
fn declarations() {
    let decl = "padding: 0; width: 48em; background-color: white;";
    let props: CssStyleProperties = decl.parse().unwrap();
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
