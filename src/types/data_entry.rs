use std::collections::HashMap;

use crate::utils::unixtime_to_datetime;

use serde::{Deserialize, Serialize};

type Raw = HashMap<String, redis::Value>;

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

            if let redis::Value::Int(created) = created {
                u32::try_from(*created)
                    .map(unixtime_to_datetime)
                    .map_err(anyhow::Error::from)?
            } else {
                return Err(anyhow::anyhow!(
                    "raw data entry has invalid 'created' field: {:?}",
                    raw
                ));
            }
        };

        let views = {
            let created = raw
                .get("views")
                .ok_or_else(|| anyhow::anyhow!("raw data entry has no 'views' field: {:?}", raw))?;

            if let redis::Value::Int(views) = created {
                u32::try_from(*views).map_err(anyhow::Error::from)?
            } else {
                return Err(anyhow::anyhow!(
                    "raw data entry has invalid 'views' field: {:?}",
                    raw
                ));
            }
        };

        let text = {
            let created = raw
                .get("text")
                .ok_or_else(|| anyhow::anyhow!("raw data entry has no 'text' field: {:?}", raw))?;

            if let redis::Value::Data(text) = created {
                String::from_utf8(text.to_owned()).map_err(anyhow::Error::from)?
            } else {
                return Err(anyhow::anyhow!(
                    "raw data entry has invalid 'text' field: {:?}",
                    raw
                ));
            }
        };

        Ok(Self {
            created,
            views,
            text,
        })
    }
}
