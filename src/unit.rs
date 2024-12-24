use crate::style::prop_validation::TokenExpected;

#[derive(Debug, Clone)]
pub enum Length {
    // absolute
    Cm(f32),
    Mm(f32),
    Q(f32),
    In(f32),
    Pc(f32),
    Pt(f32),
    Px(f32),

    // relative

    // font
    Cap(f32),
    Ch(f32),
    Em(f32),
    Ex(f32),
    Ic(f32),
    Lh(f32),

    // root element's font
    RCap(f32),
    RCh(f32),
    REm(f32),
    REx(f32),
    RIc(f32),
    RLh(f32),

    // viewport
    Vh(f32),
    Vw(f32),
    VMax(f32),
    VMin(f32),
    Vb(f32),
    Vi(f32),

    // container query
    Cqw(f32),
    Cqh(f32),
    Cqi(f32),
    Cqb(f32),
    CqMin(f32),
    CqMax(f32),
}

#[derive(Debug, Clone)]
pub enum Angle {
    Deg(f32),
    Grad(f32),
    Rad(f32),
    Turn(f32),
}

#[derive(Debug, Clone)]
pub enum Time {
    Sec(f32),
    Milli(f32),
}

#[derive(Debug, Clone)]
pub enum Frequency {}

#[derive(Debug, Clone)]
pub enum Resolution {
    Dpi(f32),
    Dpcm(f32),
    Dppx(f32),
}

#[derive(Debug, Clone, derive_more::From)]
pub enum Dimension {
    #[from]
    Length(Length),
    #[from]
    Angle(Angle),
    #[from]
    Time(Time),
    #[from]
    Frequency(Frequency),
    #[from]
    Resolution(Resolution),
}

#[derive(Debug, Clone, derive_more::From)]
pub struct Percentage(f32);

#[derive(Debug, Clone, derive_more::From)]
pub struct Hash(String);

#[derive(Debug, Clone, derive_more::From)]
pub enum Unit {
    #[from]
    Integer(i32),
    #[from]
    Number(f32),
    #[from]
    Dimension(Dimension),
    #[from]
    Percentage(Percentage),
    #[from]
    String(String),
    #[from]
    Hash(Hash),

    #[from]
    Function{
        name: String,
        required_return_type: TokenExpected,
        params: Vec<Unit>,
    },

    #[from]
    Shorthand(Box<Unit>, Box<Unit>),

    UnknownIdent(String),
}

impl Length {
    pub fn get(&self) -> f32 {
        match self {
            Length::Cm(num) => *num,
            Length::Mm(num) => *num,
            Length::Q(num) => *num,
            Length::In(num) => *num,
            Length::Pc(num) => *num,
            Length::Pt(num) => *num,
            Length::Px(num) => *num,

            Length::Cap(num) => *num,
            Length::Ch(num) => *num,
            Length::Em(num) => *num,
            Length::Ex(num) => *num,
            Length::Ic(num) => *num,
            Length::Lh(num) => *num,

            Length::RCap(num) => *num,
            Length::RCh(num) => *num,
            Length::REm(num) => *num,
            Length::REx(num) => *num,
            Length::RIc(num) => *num,
            Length::RLh(num) => *num,

            Length::Vh(num) => *num,
            Length::Vw(num) => *num,
            Length::VMax(num) => *num,
            Length::VMin(num) => *num,
            Length::Vb(num) => *num,
            Length::Vi(num) => *num,

            Length::Cqw(num) => *num,
            Length::Cqh(num) => *num,
            Length::Cqi(num) => *num,
            Length::Cqb(num) => *num,
            Length::CqMin(num) => *num,
            Length::CqMax(num) => *num,
        }
    }

    pub fn from_pair(num: f32, unit: &str) -> Option<Self> {
        match unit {
            "cm" => Some(Self::Cm(num)),
            "mm" => Some(Self::Mm(num)),
            "Q" => Some(Self::Q(num)),
            "in" => Some(Self::In(num)),
            "pc" => Some(Self::Pc(num)),
            "pt" => Some(Self::Pt(num)),
            "px" => Some(Self::Px(num)),

            "cap" => Some(Self::Cap(num)),
            "ch" => Some(Self::Ch(num)),
            "em" => Some(Self::Em(num)),
            "ex" => Some(Self::Ex(num)),
            "ic" => Some(Self::Ic(num)),
            "lh" => Some(Self::Lh(num)),

            "rcap" => Some(Self::RCap(num)),
            "rch" => Some(Self::RCh(num)),
            "rem" => Some(Self::REm(num)),
            "rex" => Some(Self::REx(num)),
            "ric" => Some(Self::RIc(num)),
            "rlh" => Some(Self::RLh(num)),

            "vh" => Some(Self::Vh(num)),
            "vw" => Some(Self::Vw(num)),
            "vMax" => Some(Self::VMax(num)),
            "vMin" => Some(Self::VMin(num)),
            "vb" => Some(Self::Vb(num)),
            "vi" => Some(Self::Vi(num)),

            "cqw" => Some(Self::Cqw(num)),
            "cqh" => Some(Self::Cqh(num)),
            "cqi" => Some(Self::Cqi(num)),
            "cqb" => Some(Self::Cqb(num)),
            "cqmin" => Some(Self::CqMin(num)),
            "cqmax" => Some(Self::CqMax(num)),

            _ => None,
        }
    }

    /// Converts all CSS length units to pixels for standard units.
    pub fn computed_value(self) -> f32 {
        match self {
            Length::Cm(cm) => cm * (96f32 / 2.54f32),
            Length::Mm(mm) => mm * (96f32 / 25.4f32),
            Length::Q(q) => q * (96f32 / 101.6f32),
            Length::In(inches) => inches * (96f32 / 1f32),
            Length::Pc(pc) => pc * (96f32 / 6f32),
            Length::Pt(pt) => pt * (96f32 / 72f32),
            Length::Px(px) => px,

            _ => unimplemented!(),
        }
    }
}

impl Angle {
    pub fn get(&self) -> f32 {
        match self {
            Angle::Deg(num) => *num,
            Angle::Grad(num) => *num,
            Angle::Rad(num) => *num,
            Angle::Turn(num) => *num,
        }
    }

    pub fn from_pair(num: f32, unit: &str) -> Option<Self> {
        match unit {
            "deg" => Some(Self::Deg(num)),
            "grad" => Some(Self::Grad(num)),
            "rad" => Some(Self::Rad(num)),
            "turn" => Some(Self::Turn(num)),

            _ => None,
        }
    }

    /// Converts all CSS angle units to radians for standard units.
    pub fn computed_value(self) -> f32 {
        match self {
            Angle::Deg(deg) => deg.to_radians(),
            Angle::Grad(grad) => grad * std::f32::consts::PI / 200f32,
            Angle::Rad(rad) => rad,
            Angle::Turn(turn) => turn * std::f32::consts::TAU,
        }
    }
}

impl Time {
    pub fn get(&self) -> f32 {
        match self {
            Time::Sec(num) => *num,
            Time::Milli(num) => *num,
        }
    }

    pub fn computed_value(self) -> f32 {
        unimplemented!()
    }
}

impl Frequency {
    pub fn computed_value(self) -> f32 {
        unimplemented!()
    }
}

impl Resolution {
    pub fn get(&self) -> f32 {
        match self {
            Resolution::Dpi(num) => *num,
            Resolution::Dpcm(num) => *num,
            Resolution::Dppx(num) => *num,
        }
    }

    /// Converts Dpi and Dpcm to Dppx for standard units.
    pub fn from_pair(num: f32, unit: &str) -> Option<Self> {
        match unit {
            "dpi" => Some(Self::Dpi(num)),
            "dpcm" => Some(Self::Dpcm(num)),
            "dppx" => Some(Self::Dppx(num)),
            "x" => Some(Self::Dppx(num)),

            _ => None,
        }
    }

    /// Converts Dpi and Dpcm to Dppx for standard units.
    pub fn computed_value(self) -> f32 {
        match self {
            Resolution::Dpi(dpi) => dpi / 96f32,
            Resolution::Dpcm(dpcm) => dpcm * 2.54f32 / 96f32,
            Resolution::Dppx(x) => x,
        }
    }
}

impl Dimension {
    pub fn from_pair(num: f32, unit: &str) -> Option<Self> {
        if let Some(length) = Length::from_pair(num, unit) {
            Some(Self::Length(length))
        } else if let Some(angle) = Angle::from_pair(num, unit) {
            Some(Self::Angle(angle))
        } else if let Some(resolution) = Resolution::from_pair(num, unit) {
            Some(Self::Resolution(resolution))
        } else {
            None
        }
    }
}

impl Into<f32> for Length {
    fn into(self) -> f32 {
        self.get()
    }
}

impl Into<f32> for Angle {
    fn into(self) -> f32 {
        match self {
            Angle::Deg(num) => num,
            Angle::Grad(num) => num,
            Angle::Rad(num) => num,
            Angle::Turn(num) => num,
        }
    }
}

impl Into<f32> for Time {
    fn into(self) -> f32 {
        match self {
            Self::Sec(num) => num,
            Self::Milli(num) => num,
        }
    }
}

impl Into<f32> for Resolution {
    fn into(self) -> f32 {
        match self {
            Resolution::Dpi(val) => val,
            Resolution::Dpcm(val) => val,
            Resolution::Dppx(val) => val,
        }
    }
}

impl Into<f32> for Dimension {
    fn into(self) -> f32 {
        match self {
            Dimension::Length(length) => length.into(),
            Dimension::Angle(angle) => angle.into(),
            Dimension::Time(_) => unimplemented!(),
            Dimension::Frequency(_) => unimplemented!(),
            Dimension::Resolution(resolution) => resolution.into(),
        }
    }
}

impl Into<f32> for Percentage {
    fn into(self) -> f32 {
        self.0
    }
}

impl std::fmt::Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Length::Cm(val) => write!(f, "{val}cm"),
            Length::Mm(val) => write!(f, "{val}mm"),
            Length::Q(val) => write!(f, "{val}Q"),
            Length::In(val) => write!(f, "{val}in"),
            Length::Pc(val) => write!(f, "{val}pc"),
            Length::Pt(val) => write!(f, "{val}pt"),
            Length::Px(val) => write!(f, "{val}px"),

            Length::Cap(val) => write!(f, "{val}cap"),
            Length::Ch(val) => write!(f, "{val}ch"),
            Length::Em(val) => write!(f, "{val}em"),
            Length::Ex(val) => write!(f, "{val}ex"),
            Length::Ic(val) => write!(f, "{val}ic"),
            Length::Lh(val) => write!(f, "{val}lh"),

            Length::RCap(val) => write!(f, "{val}rcap"),
            Length::RCh(val) => write!(f, "{val}rch"),
            Length::REm(val) => write!(f, "{val}rem"),
            Length::REx(val) => write!(f, "{val}rex"),
            Length::RIc(val) => write!(f, "{val}ric"),
            Length::RLh(val) => write!(f, "{val}rlh"),

            Length::Vh(val) => write!(f, "{val}vh"),
            Length::Vw(val) => write!(f, "{val}vw"),
            Length::VMax(val) => write!(f, "{val}vmax"),
            Length::VMin(val) => write!(f, "{val}vmin"),
            Length::Vb(val) => write!(f, "{val}vb"),
            Length::Vi(val) => write!(f, "{val}vi"),

            Length::Cqw(val) => write!(f, "{val}cqw"),
            Length::Cqh(val) => write!(f, "{val}cqh"),
            Length::Cqi(val) => write!(f, "{val}cqi"),
            Length::Cqb(val) => write!(f, "{val}cqb"),
            Length::CqMin(val) => write!(f, "{val}cqmin"),
            Length::CqMax(val) => write!(f, "{val}cqmax"),
        }
    }
}

impl std::fmt::Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Angle::Deg(val) => write!(f, "{val}deg"),
            Angle::Grad(val) => write!(f, "{val}grad"),
            Angle::Rad(val) => write!(f, "{val}rad"),
            Angle::Turn(val) => write!(f, "{val}turn"),
        }
    }
}

impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Resolution::Dpi(val) => write!(f, "{val}dpi"),
            Resolution::Dpcm(val) => write!(f, "{val}dpcm"),
            Resolution::Dppx(val) => write!(f, "{val}x"),
        }
    }
}

impl std::fmt::Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dimension::Length(length) => write!(f, "{length}"),
            Dimension::Angle(angle) => write!(f, "{angle}"),
            Dimension::Time(time) => todo!(),
            Dimension::Frequency(frequency) => todo!(),
            Dimension::Resolution(resolution) => write!(f, "{resolution}"),
        }
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Integer(int) => write!(f, "{int}"),
            Unit::Number(num) => write!(f, "{num}"),
            Unit::Dimension(dimension) => write!(f, "{dimension}"),
            Unit::Percentage(percentage) => write!(f, "{}", percentage.0),
            Unit::String(string) => write!(f, r#""{string}""#),
            Unit::Hash(hash) => write!(f, "#{}", hash.0),
            Unit::Shorthand(val1, val2) => write!(f, "{val1}/{val2}"),
            Unit::UnknownIdent(ident) => write!(f, "{ident}"),
            Unit::Function { name, required_return_type, params } => {
                let param_string: String = params.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "{name}({param_string})")
            }
        }
    }
}
