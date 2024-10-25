#![allow(warnings)]

//! # Crest
//!
//! Crest is [Peacock](https://github.com/nucleus-labs/peacock)'s core library for parsing css files. While Crest is intended for use by Peacock, it is designed to be usable for other projects as well.
//!
//! For more information on Peacock, [click here](https://github.com/nucleus-labs/peacock)!
//!

mod parse;
mod css;

use pest::Parser;

use std::collections::HashMap;

pub use css::*;
use parse::CssParser;
