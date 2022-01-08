use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lang {
    Ru,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ru => write!(f, "ru"),
        }
    }
}
