use std::fmt;

use crate::utils;

pub fn data(cmd: Command, key: &str) -> String {
    format!("/{}#{}", cmd, utils::hash(key))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Goto,
    Like,
    Feedback,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Goto => write!(f, "goto"),
            Self::Like => write!(f, "like"),
            Self::Feedback => write!(f, "feedback"),
        }
    }
}
