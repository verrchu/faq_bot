use std::fmt;

pub static RU: &str = "ru";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Ru,
}

impl Lang {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ru => RU,
        }
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ru => write!(f, "{}", RU),
        }
    }
}
