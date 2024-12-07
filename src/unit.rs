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
    // NOT YET IMPLEMENTED ; requires dynamic loading
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
        }
    }

    pub fn from_pair(num: f32, unit: &str) -> Option<Self> {
        match unit {
            "cm" => Some(Self::Cm(num * (96f32 / 2.54f32))),
            "mm" => Some(Self::Mm(num * (96f32 / 25.4f32))),
            "Q" => Some(Self::Q(num * (96f32 / 101.6f32))),
            "in" => Some(Self::In(num * (96f32 / 1f32))),
            "pc" => Some(Self::Pc(num * (96f32 / 6f32))),
            "pt" => Some(Self::Pt(num * (96f32 / 72f32))),
            "px" => Some(Self::Px(num)),

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
            "deg" => Some(Self::Deg(num.to_radians())),
            "grad" => Some(Self::Grad(num * std::f32::consts::PI / 200f32)),
            "rad" => Some(Self::Rad(num)),
            "turn" => Some(Self::Turn(num * std::f32::consts::TAU)),

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
            "dpi" => Some(Self::Dpi(num / 96f32)),
            "dpcm" => Some(Self::Dpcm(num * 2.54f32 / 96f32)),
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
        match self {
            Self::Cm(num) => num,
            Self::Mm(num) => num,
            Self::Q(num) => num,
            Self::In(num) => num,
            Self::Pc(num) => num,
            Self::Pt(num) => num,
            Self::Px(num) => num,
        }
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

impl std::fmt::Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dimension::Length(length) => write!(f, "{}px", length.get()),
            Dimension::Angle(angle) => write!(f, "{}rad", angle.get()),
            Dimension::Time(time) => write!(f, "{}s", time.get()),
            Dimension::Frequency(frequency) => write!(f, ""),
            Dimension::Resolution(resolution) => write!(f, "{}x", resolution.get()),
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
        }
    }
}
