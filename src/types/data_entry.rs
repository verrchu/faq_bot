use std::collections::HashMap;

use crate::utils::unixtime_to_datetime;

use serde::{Deserialize, Serialize};

type Raw = HashMap<String, String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataEntry {
    pub text: String,
    pub created: String,
    pub views: u32,
}

impl TryFrom<Raw> for DataEntry {
    type Error = anyhow::Error;

    fn try_from(raw: Raw) -> anyhow::Result<Self> {
        let created = {
            let created = raw.get("created").ok_or_else(|| {
                anyhow::anyhow!("raw data entry has no 'created' field: {:?}", raw)
            })?;

            created
                .parse()
                .map(unixtime_to_datetime)
                .map_err(anyhow::Error::from)?
        };

        let views = {
            let views = raw
                .get("views")
                .ok_or_else(|| anyhow::anyhow!("raw data entry has no 'views' field: {:?}", raw))?;

            views.parse().map_err(anyhow::Error::from)?
        };

        let text = raw
            .get("text")
            .ok_or_else(|| anyhow::anyhow!("raw data entry has no 'text' field: {:?}", raw))?
            .to_owned();

        Ok(Self {
            created,
            views,
            text,
        })
    }
}
