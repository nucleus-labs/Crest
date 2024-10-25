
use strum_macros::EnumString;

use std::str::FromStr;
use crate::css::Error;

#[derive(Debug, Default, EnumString)]
pub enum DisplayOption {
    #[strum(serialize = "inherit")]
    #[default]
    Inherit,

    #[strum(serialize = "none")]
    None,
    #[strum(serialize = "block")]
    Block,
    #[strum(serialize = "inline")]
    Inline,
    #[strum(serialize = "flow")]
    Flow,
    // todo: inline-block, flex, grid
}

pub struct Display {
    pub outer: DisplayOption,
    pub inner: DisplayOption,
}

// ============== IMPL ==============

impl super::FromProperty for Display {
    fn get_name() -> &'static str { "display" }

    fn from_property(property_values: Vec<super::Generic>) -> Result<Self, Error> {
        match property_values.as_slice() {
            [super::Generic::Identifier(outer_raw), super::Generic::Identifier(inner_raw)] => {
                match [DisplayOption::from_str(outer_raw), DisplayOption::from_str(inner_raw)] {
                    [Ok(outer), Ok(inner)] => Ok(Self{
                        outer,
                        inner,
                    }),
                    [Ok(_), Err(err)] => Err(Error::ParseError(vec![err])),
                    [Err(err), Ok(_)] => Err(Error::ParseError(vec![err])),
                    [Err(err1), Err(err2)] => Err(Error::ParseError(vec![err1, err2])),
                }
            },
            [super::Generic::Identifier(outer_raw)] => {
                match DisplayOption::from_str(outer_raw) {
                    Ok(outer) => Ok(Self{
                        outer,
                        inner: DisplayOption::Flow,
                    }),
                    Err(err) => Err(Error::ParseError(vec![err])),
                }
            },
            _ => Err(Error::StructureMismatch)
        }
    }
}

impl Default for Display {
    fn default() -> Self {
        Self { outer: DisplayOption::Block, inner: DisplayOption::Flow }
    }
}
