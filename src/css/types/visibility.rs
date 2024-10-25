
use strum_macros::EnumString;

use std::str::FromStr;
use crate::css::Error;

#[derive(Default, EnumString)]
pub enum Visibility {
    #[strum(serialize = "visible")]
    #[default]
    Visible,
    #[strum(serialize = "hidden")]
    Hidden,
}

// ============== IMPL ==============

impl super::FromProperty for Visibility {
    fn get_name() -> &'static str { "visbility" }

    fn from_property(property_values: Vec<super::Generic>) -> Result<Self, Error> {
        match property_values.as_slice() {
            [super::Generic::Identifier(ident)] => {
                match Visibility::from_str(ident) {
                    Ok(position) => Ok(position),
                    Err(err) => Err(Error::ParseError(vec![err])),
                }
            }
            _ => Err(Error::StructureMismatch)
        }
    }
}
