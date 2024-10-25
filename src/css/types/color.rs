
use super::{FromProperty, Generic, Number};

use crate::css::Error;

pub struct Color(pub [u8; 3]);
pub struct BackgroundColor(pub [u8; 3]);

// ============== IMPL ==============

impl FromProperty for Color {
    fn get_name() -> &'static str { "color" }

    fn from_property(property_values: Vec<super::Generic>) -> Result<Self, Error> {
        match property_values.as_slice() {
            [super::Generic::Color(color)] => {
                Ok(Color(color.clone()))
            }
            _ => Err(Error::StructureMismatch)
        }
    }
}

impl FromProperty for BackgroundColor {
    fn get_name() -> &'static str { "background-color" }

    fn from_property(property_values: Vec<super::Generic>) -> Result<Self, Error> {
        match property_values.as_slice() {
            [super::Generic::Color(color)] => {
                Ok(BackgroundColor(color.clone()))
            }
            _ => Err(Error::StructureMismatch)
        }
    }
}

