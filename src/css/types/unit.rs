
use strum_macros::EnumString;

use std::str::FromStr;

#[derive(Debug, Clone, EnumString)]
pub enum Unit {
    #[strum(serialize = "cm")]
    Cm,
    #[strum(serialize = "mm")]
    Mm,
    #[strum(serialize = "in")]
    In,
    #[strum(serialize = "px")]
    Px,
    #[strum(serialize = "pt")]
    Pt,
    #[strum(serialize = "pc")]
    Pc,

    #[strum(serialize = "em")]
    Em,
    #[strum(serialize = "ex")]
    Ex,
    #[strum(serialize = "ch")]
    Ch,
    #[strum(serialize = "rem")]
    Rem,
    #[strum(serialize = "vw")]
    VW,
    #[strum(serialize = "vh")]
    VH,
    #[strum(serialize = "vmin")]
    VMin,
    #[strum(serialize = "vmax")]
    VMax,
    #[strum(serialize = "%")]
    Percent,
}

// ============== IMPL ==============
