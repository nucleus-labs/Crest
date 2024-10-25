
use super::{FromProperty, Generic, Number};

use crate::css::Error;

pub struct Width(pub Number);
pub struct Height(pub Number);

pub struct Margin(pub Number, pub Number, pub Number, pub Number);
pub struct Padding(pub Number, pub Number, pub Number, pub Number);

pub struct Top(pub Number);
pub struct Bottom(pub Number);
pub struct Left(pub Number);
pub struct Right(pub Number);

// ============== IMPL ==============

impl FromProperty for Width {
    fn get_name() -> &'static str { "width" }

    fn from_property(property_values: Vec<super::Generic>) -> Result<Self, Error> {
        match property_values.as_slice() {
            [Generic::Number(number)] => {
                Ok(Width(number.clone()))
            }
            _ => Err(Error::StructureMismatch)
        }
    }
}

impl FromProperty for Height {
    fn get_name() -> &'static str { "height" }

    fn from_property(property_values: Vec<super::Generic>) -> Result<Self, Error> {
        match property_values.as_slice() {
            [Generic::Number(number)] => {
                Ok(Height(number.clone()))
            }
            _ => Err(Error::StructureMismatch)
        }
    }
}

impl FromProperty for Margin {
    fn get_name() -> &'static str { "margin" }

    fn from_property(property_values: Vec<super::Generic>) -> Result<Self, Error> {
        match property_values.as_slice() {
            [Generic::Number(number)] => {
                Ok(Margin(number.clone(), number.clone(), number.clone(), number.clone()))
            },
            [Generic::Number(number1), Generic::Number(number2)] => {
                Ok(Margin(number1.clone(), number2.clone(), number1.clone(), number2.clone()))
            },
            [Generic::Number(number1), Generic::Number(number2), Generic::Number(number3)] => {
                Ok(Margin(number1.clone(), number2.clone(), number3.clone(), number2.clone()))
            },
            [Generic::Number(number1), Generic::Number(number2), Generic::Number(number3), Generic::Number(number4)] => {
                Ok(Margin(number1.clone(), number2.clone(), number3.clone(), number4.clone()))
            },
            _ => Err(Error::StructureMismatch),
        }
    }
}

impl FromProperty for Padding {
    fn get_name() -> &'static str { "padding" }

    fn from_property(property_values: Vec<super::Generic>) -> Result<Self, Error> {
        match property_values.as_slice() {
            [Generic::Number(number)] => {
                Ok(Padding(number.clone(), number.clone(), number.clone(), number.clone()))
            },
            [Generic::Number(number1), Generic::Number(number2)] => {
                Ok(Padding(number1.clone(), number2.clone(), number1.clone(), number2.clone()))
            },
            [Generic::Number(number1), Generic::Number(number2), Generic::Number(number3)] => {
                Ok(Padding(number1.clone(), number2.clone(), number3.clone(), number2.clone()))
            },
            [Generic::Number(number1), Generic::Number(number2), Generic::Number(number3), Generic::Number(number4)] => {
                Ok(Padding(number1.clone(), number2.clone(), number3.clone(), number4.clone()))
            },
            _ => Err(Error::StructureMismatch),
        }
    }
}

impl FromProperty for Top {
    fn get_name() -> &'static str { "top" }

    fn from_property(property_values: Vec<super::Generic>) -> Result<Self, Error> {
        match property_values.as_slice() {
            [Generic::Number(number)] => {
                Ok(Top(number.clone()))
            }
            _ => Err(Error::StructureMismatch)
        }
    }
}

impl FromProperty for Bottom {
    fn get_name() -> &'static str { "bottom" }

    fn from_property(property_values: Vec<super::Generic>) -> Result<Self, Error> {
        match property_values.as_slice() {
            [Generic::Number(number)] => {
                Ok(Bottom(number.clone()))
            }
            _ => Err(Error::StructureMismatch)
        }
    }
}

impl FromProperty for Left {
    fn get_name() -> &'static str { "left" }

    fn from_property(property_values: Vec<super::Generic>) -> Result<Self, Error> {
        match property_values.as_slice() {
            [Generic::Number(number)] => {
                Ok(Left(number.clone()))
            }
            _ => Err(Error::StructureMismatch)
        }
    }
}

impl FromProperty for Right {
    fn get_name() -> &'static str { "right" }

    fn from_property(property_values: Vec<super::Generic>) -> Result<Self, Error> {
        match property_values.as_slice() {
            [Generic::Number(number)] => {
                Ok(Right(number.clone()))
            }
            _ => Err(Error::StructureMismatch)
        }
    }
}
