use std::collections::HashMap;

use crate::syntax::{CssExpectError, CssParser, CssRule, CssToken, CssTokenTracker};
use crate::source::{parse_source, SourceInfo, SourceSlice};
use crate::error::Error;
use crate::unit::Unit;

use super::prop_validation::{self, types, CssAttributeValue, CssStyleAttribute};

#[derive(Debug, Clone)]
pub enum CssStyleProperty {
    Color(CssAttributeValue<types::CssColor>),
    BackgroundColor(CssAttributeValue<types::CssBackgroundColor>),

    Width(CssAttributeValue<types::CssWidth>),
    Height(CssAttributeValue<types::CssHeight>),

    Padding(
        CssAttributeValue<types::CssPadding>,
        CssAttributeValue<types::CssPadding>,
        CssAttributeValue<types::CssPadding>,
        CssAttributeValue<types::CssPadding>,
    ),

    Spacing(
        CssAttributeValue<types::CssSpacing>,
        CssAttributeValue<types::CssSpacing>,
        CssAttributeValue<types::CssSpacing>,
        CssAttributeValue<types::CssSpacing>,
    ),

    FontFamily(CssAttributeValue<types::CssFontFamily>),
    FontSize(CssAttributeValue<types::CssFontSize>),
    // FontShaping(CssAttributeValue<types::CssFontShaping>),
    LineHeight(CssAttributeValue<types::CssLineHeight>),
    WordBreak(CssAttributeValue<types::CssWordBreak>),
    // TextAlign(CssAttributeValue<types::CssTextAlign>),
    MaxWidth(CssAttributeValue<types::CssMaxWidth>),
    MaxHeight(CssAttributeValue<types::CssMaxHeight>),

    JustifyContent(CssAttributeValue<types::CssJustifyContent>),
    VerticalAlign(CssAttributeValue<types::CssVerticalAlign>),

    OverflowX(CssAttributeValue<types::CssOverflowX>),
    OverflowY(CssAttributeValue<types::CssOverflowY>),

    ObjectFit(CssAttributeValue<types::CssObjectFit>),
    ImageRendering(CssAttributeValue<types::CssImageRendering>),
    // Rotation(CssAttributeValue<types::CssRotation>),
    Opacity(CssAttributeValue<types::CssOpacity>),

    UnknownProperty(String, Vec<Vec<Unit>>),
}

/// Whether the property is marked 'important!'
pub type IsImportant = bool;

#[derive(Debug, Clone, Default)]
pub struct CssStyleProperties(pub Vec<(CssStyleProperty, IsImportant)>);

impl CssStyleProperty {
    pub fn from_attribute_values(values: Box<[CssStyleAttribute]>) -> Self {
        match &*values {
            [CssStyleAttribute::Color(val)] => Self::Color(val.clone()),
            [CssStyleAttribute::BackgroundColor(val)] => Self::BackgroundColor(val.clone()),
            [CssStyleAttribute::Width(val)] => Self::Width(val.clone()),
            [CssStyleAttribute::Height(val)] => Self::Height(val.clone()),
            [CssStyleAttribute::MaxWidth(val)] => Self::MaxWidth(val.clone()),
            [CssStyleAttribute::MaxHeight(val)] => Self::MaxHeight(val.clone()),

            [CssStyleAttribute::LineHeight(val)] => Self::LineHeight(val.clone()),

            [CssStyleAttribute::VerticalAlign(val)] => Self::VerticalAlign(val.clone()),

            // BEGIN PADDING
            [CssStyleAttribute::Padding(val)] => {
                Self::Padding(val.clone(), val.clone(), val.clone(), val.clone())
            }
            [CssStyleAttribute::Padding(val1), CssStyleAttribute::Padding(val2)] => {
                Self::Padding(val1.clone(), val2.clone(), val1.clone(), val2.clone())
            }
            [CssStyleAttribute::Padding(val1), CssStyleAttribute::Padding(val2), CssStyleAttribute::Padding(val3)] => {
                Self::Padding(val1.clone(), val2.clone(), val3.clone(), val2.clone())
            }
            [CssStyleAttribute::Padding(val1), CssStyleAttribute::Padding(val2), CssStyleAttribute::Padding(val3), CssStyleAttribute::Padding(val4)] => {
                Self::Padding(val1.clone(), val2.clone(), val3.clone(), val4.clone())
            }

            // BEGIN SPACING
            [CssStyleAttribute::Spacing(val)] => {
                Self::Spacing(val.clone(), val.clone(), val.clone(), val.clone())
            }
            [CssStyleAttribute::Spacing(val1), CssStyleAttribute::Spacing(val2)] => {
                Self::Spacing(val1.clone(), val2.clone(), val1.clone(), val2.clone())
            }
            [CssStyleAttribute::Spacing(val1), CssStyleAttribute::Spacing(val2), CssStyleAttribute::Spacing(val3)] => {
                Self::Spacing(val1.clone(), val2.clone(), val3.clone(), val2.clone())
            }
            [CssStyleAttribute::Spacing(val1), CssStyleAttribute::Spacing(val2), CssStyleAttribute::Spacing(val3), CssStyleAttribute::Spacing(val4)] => {
                Self::Spacing(val1.clone(), val2.clone(), val3.clone(), val4.clone())
            }
            [CssStyleAttribute::FontFamily(val)] => Self::FontFamily(val.clone()),
            [CssStyleAttribute::FontSize(val)] => Self::FontSize(val.clone()),
            [CssStyleAttribute::UnknownProperty(prop_name, prop_values)] => {
                Self::UnknownProperty(prop_name.clone(), prop_values.clone())
            }

            _ => todo!("Update structure unknown: {values:?}"),
        }
    }

    pub fn get_prop_name(&self) -> String {
        match self {
            CssStyleProperty::Color(_) => "color".into(),
            CssStyleProperty::BackgroundColor(_) => "background-color".into(),
            CssStyleProperty::Width(_) => "width".into(),
            CssStyleProperty::Height(_) => "height".into(),
            CssStyleProperty::Padding(_, _, _, _) => "padding".into(),
            CssStyleProperty::Spacing(_, _, _, _) => "spacing".into(),
            CssStyleProperty::FontFamily(_) => "font-family".into(),
            CssStyleProperty::FontSize(_) => "font-size".into(),
            CssStyleProperty::LineHeight(_) => "line-height".into(),
            CssStyleProperty::WordBreak(_) => "word-break".into(),
            CssStyleProperty::MaxWidth(_) => "max-width".into(),
            CssStyleProperty::MaxHeight(_) => "max-height".into(),
            CssStyleProperty::JustifyContent(_) => "justify-content".into(),
            CssStyleProperty::VerticalAlign(_) => "vertical-align".into(),
            CssStyleProperty::OverflowX(_) => "overflow-x".into(),
            CssStyleProperty::OverflowY(_) => "overflow-y".into(),
            CssStyleProperty::ObjectFit(_) => "object-fit".into(),
            CssStyleProperty::ImageRendering(_) => "image-rendering".into(),
            CssStyleProperty::Opacity(_) => "opacity".into(),
            CssStyleProperty::UnknownProperty(prop, _) => prop.clone(),
        }
    }
}

impl CssStyleProperties {
    pub fn from_source(source: &str) -> Result<Self, Error> {
        source.parse::<Self>()
    }
}

impl std::str::FromStr for CssStyleProperties {
    type Err = Error;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match parse_source::<CssRule, CssParser>(SourceInfo::new(source.into()), CssRule::DECLARATION_LIST) {
            Ok(css_token) => {
                let expector = CssTokenTracker::new(&css_token).unwrap();
                let props = expector.expect_decl_list()?;
                
                Ok(props)
            },
            Err(err) => Err(Error::CssError(err.into())),
        }
    }
}

impl std::ops::Deref for CssStyleProperties {
    type Target = Vec<(CssStyleProperty, bool)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for CssStyleProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for CssStyleProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (item, important) in self.iter() {
            write!(f, "    ");
            match item {
                CssStyleProperty::UnknownProperty(prop_name, prop_values) => {
                    // [[a, b, c], [d, e, f]] => "a b c, d e f"
                    let prop_string = prop_values.iter()
                        .map(|group| {
                            // [a, b, c] => "a b c"
                            group.iter()
                                .map(|values| values.to_string())
                                .collect::<Vec<_>>()
                                .join(" ")
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                        ;
                    write!(f, "{prop_name}: {prop_string} /* this prop is unknown and has no validation or guarantees */")
                },
                CssStyleProperty::Color(css_attribute_value)
                    => write!(f, "color: {css_attribute_value}"),
                CssStyleProperty::BackgroundColor(css_attribute_value)
                    => write!(f, "background-color: {css_attribute_value}"),
                CssStyleProperty::Width(css_attribute_value)
                    => write!(f, "width: {css_attribute_value}"),
                CssStyleProperty::Height(css_attribute_value)
                    => write!(f, "height: {css_attribute_value}"),
                CssStyleProperty::Padding(css_attribute_value1, css_attribute_value2, css_attribute_value3, css_attribute_value4)
                    => write!(f, "padding: {css_attribute_value1} {css_attribute_value2} {css_attribute_value3} {css_attribute_value4}"),
                CssStyleProperty::Spacing(css_attribute_value1, css_attribute_value2, css_attribute_value3, css_attribute_value4)
                    => write!(f, "spacing: {css_attribute_value1} {css_attribute_value2} {css_attribute_value3} {css_attribute_value4}"),
                CssStyleProperty::FontFamily(css_attribute_value)
                    => write!(f, "font-family: {css_attribute_value}"),
                CssStyleProperty::FontSize(css_attribute_value)
                    => write!(f, "font-size: {css_attribute_value}"),
                CssStyleProperty::LineHeight(css_attribute_value)
                    => write!(f, "line-height: {css_attribute_value}"),
                CssStyleProperty::WordBreak(css_attribute_value)
                    => write!(f, "word-break: {css_attribute_value}"),
                CssStyleProperty::MaxWidth(css_attribute_value)
                    => write!(f, "max-width: {css_attribute_value}"),
                CssStyleProperty::MaxHeight(css_attribute_value)
                    => write!(f, "max-height: {css_attribute_value}"),
                CssStyleProperty::JustifyContent(css_attribute_value)
                    => write!(f, "justify-content: {css_attribute_value}"),
                CssStyleProperty::VerticalAlign(css_attribute_value)
                    => write!(f, "vertical-align: {css_attribute_value}"),
                CssStyleProperty::OverflowX(css_attribute_value)
                    => write!(f, "overflow-x: {css_attribute_value}"),
                CssStyleProperty::OverflowY(css_attribute_value)
                    => write!(f, "overflow-y: {css_attribute_value}"),
                CssStyleProperty::ObjectFit(css_attribute_value)
                    => write!(f, "object-fit: {css_attribute_value}"),
                CssStyleProperty::ImageRendering(css_attribute_value)
                    => write!(f, "image-rendering: {css_attribute_value}"),
                CssStyleProperty::Opacity(css_attribute_value)
                    => write!(f, "opacity: {css_attribute_value}"),
            };

            if *important {
                write!(f, " !important");
            }

            writeln!(f, ";");
        }

        Ok(())
    }
}

impl CssStyleProperties {
    pub fn eval_prop(&self, prop_name: &str) -> Option<CssStyleProperty> {
        let mut result: Option<CssStyleProperty> = None;

        for (prop, important) in self.0.iter() {
            if prop_name.eq_ignore_ascii_case(&prop.get_prop_name()) {
                result = Some(prop.clone());
            }
        }

        result
    }
}
