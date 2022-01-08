use crate::Lang;

use serde::{Deserialize, Serialize};
use tera::Tera;

static RU: &str = r#"
{{ header }}

{{ data }}

_Опубликовано: {{ created }}_
_Просмотрено: {{ views }}_
"#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub header: String,
    pub data: String,
    pub created: String,
    pub views: u64,
}

pub fn render(context: Context, lang: Lang) -> anyhow::Result<String> {
    let context = tera::Context::from_serialize(context).map_err(anyhow::Error::from)?;

    let template = match lang {
        Lang::Ru => RU,
    };

    Tera::one_off(template, &context, false).map_err(anyhow::Error::from)
}
