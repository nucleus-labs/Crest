
use super::{FromProperty, Generic, Number};

use crate::css::Error;

pub struct FontSize(pub Number);

// ============== IMPL ==============

impl FromProperty for FontSize {
    fn get_name() -> &'static str { "font-size" }

    fn from_property(property_values: Vec<super::Generic>) -> Result<Self, Error> {
        match property_values.as_slice() {
            [super::Generic::Number(number)] => {
                Ok(FontSize(number.clone()))
            }
            _ => Err(Error::StructureMismatch)
        }
    }
}
