use std::collections::HashMap;

use super::parse_attr::{self, types, CssAttributeValue, CssStyleAttribute};
use crate::unit::Unit;

#[derive(Debug, Clone)]
pub struct CssStyleProperties {
    pub color: CssAttributeValue<types::CssColor>,
    pub background_color: CssAttributeValue<types::CssBackgroundColor>,
    pub width: CssAttributeValue<types::CssWidth>,
    pub height: CssAttributeValue<types::CssHeight>,
    pub padding: (
        CssAttributeValue<types::CssPadding>,
        CssAttributeValue<types::CssPadding>,
        CssAttributeValue<types::CssPadding>,
        CssAttributeValue<types::CssPadding>,
    ),
    pub spacing: (
        CssAttributeValue<types::CssSpacing>,
        CssAttributeValue<types::CssSpacing>,
        CssAttributeValue<types::CssSpacing>,
        CssAttributeValue<types::CssSpacing>,
    ),
    pub font_family: CssAttributeValue<types::CssFontFamily>,
    pub font_size: CssAttributeValue<types::CssFontSize>,
    pub line_height: (),
    pub font_shaping: (),
    pub text_wrap: (),
    pub text_align: (),
    pub max_width: (),
    pub max_height: (),
    pub justify_content: (),
    pub vertical_align: (),
    pub overflow: (),
    pub margin: ((), (), (), ()),
    pub align: ((), (), (), ()),
    pub object_fit: (),
    pub image_rendering: (),
    pub rotation: (),
    pub opacity: (),
}

impl CssStyleProperties {
    pub fn update(&mut self, attr: Box<[CssStyleAttribute]>) {
        match &*attr {
            [CssStyleAttribute::Width(val)] => self.width = val.clone(),
            [CssStyleAttribute::Height(val)] => self.height = val.clone(),
            [CssStyleAttribute::Padding(val1), CssStyleAttribute::Padding(val2), CssStyleAttribute::Padding(val3), CssStyleAttribute::Padding(val4)] => {
                self.padding = (val1.clone(), val2.clone(), val3.clone(), val4.clone())
            }

            _ => todo!("Update structure unknown"),
        }
    }

    pub fn apply(&self, other: &Self) -> Self {
        todo!()
    }

    pub fn calculated_values(&self) -> HashMap<&'static str, Box<[Unit]>> {
        todo!()
    }
}

impl std::default::Default for CssStyleProperties {
    fn default() -> Self {
        Self {
            color: CssAttributeValue::Keyword(parse_attr::color::KeywordColor::Black),
            background_color: CssAttributeValue::Keyword(parse_attr::color::KeywordColor::White),
            width: CssAttributeValue::Keyword(parse_attr::size::KeywordSize::Auto),
            height: CssAttributeValue::Keyword(parse_attr::size::KeywordSize::Auto),
            padding: (
                CssAttributeValue::Value(Unit::Integer(0)),
                CssAttributeValue::Value(Unit::Integer(0)),
                CssAttributeValue::Value(Unit::Integer(0)),
                CssAttributeValue::Value(Unit::Integer(0)),
            ),
            spacing: (
                CssAttributeValue::Value(Unit::Integer(0)),
                CssAttributeValue::Value(Unit::Integer(0)),
                CssAttributeValue::Value(Unit::Integer(0)),
                CssAttributeValue::Value(Unit::Integer(0)),
            ),
            font_family: CssAttributeValue::Keyword(parse_attr::font::KeywordFontFamily::SansSerif),
            font_size: CssAttributeValue::Keyword(parse_attr::font::KeywordFontSize::Medium),
            line_height: Default::default(),
            font_shaping: Default::default(),
            text_wrap: Default::default(),
            text_align: Default::default(),
            max_width: Default::default(),
            max_height: Default::default(),
            justify_content: Default::default(),
            vertical_align: Default::default(),
            overflow: Default::default(),
            margin: Default::default(),
            align: Default::default(),
            object_fit: Default::default(),
            image_rendering: Default::default(),
            rotation: Default::default(),
            opacity: Default::default(),
        }
    }
}

impl std::fmt::Display for CssStyleProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\tcolor: {};", self.color);
        writeln!(f, "\tbackground-color: {};", self.background_color);
        writeln!(f, "\twidth: {};", self.width);
        writeln!(f, "\theight: {};", self.height);
        writeln!(
            f,
            "\tpadding: {} {} {} {};",
            self.padding.0, self.padding.1, self.padding.2, self.padding.3
        );
        writeln!(
            f,
            "\tspacing: {} {} {} {};",
            self.spacing.0, self.spacing.1, self.spacing.2, self.spacing.3
        );
        writeln!(f, "\tfont-family: {};", self.font_family);
        writeln!(f, "\tfont-size: {};", self.font_size);

        Ok(())
    }
}
