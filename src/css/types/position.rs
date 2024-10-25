
use strum_macros::EnumString;

use std::str::FromStr;
use crate::css::Error;

#[derive(Default, EnumString)]
pub enum Position {
    #[strum(serialize = "static")]
    #[default]
    Static,
    #[strum(serialize = "relative")]
    Relative,
    #[strum(serialize = "absolute")]
    Absolute,
}

// ============== IMPL ==============

impl super::FromProperty for Position {
    fn get_name() -> &'static str { "position" }

    fn from_property(property_values: Vec<super::Generic>) -> Result<Self, Error> {
        match property_values.as_slice() {
            [super::Generic::Identifier(ident)] => {
                match Position::from_str(ident) {
                    Ok(position) => Ok(position),
                    Err(err) => Err(Error::ParseError(vec![err])),
                }
            }
            _ => Err(Error::StructureMismatch)
        }
    }
}
