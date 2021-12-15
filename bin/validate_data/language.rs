use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Language {
    Ru,
    En,
}

impl FromStr for Language {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ru" => Ok(Self::Ru),
            "en" => Ok(Self::En),
            other => Err(anyhow::anyhow!("Failed to parse language from '{}'", other)),
        }
    }
}
