use crate::{l10n::Lang, DataEntry};

use serde::{Deserialize, Serialize};
use tera::Tera;

static RU: &str = r#"
*{{ header }}*

{{ data_entry.text }}

*_Опубликовано: {{ data_entry.created }}_*
{% if data_entry.updated -%}
*_Обновлено: {{ data_entry.created }}_*
{%- endif %}
*_Просмотрено: {{ data_entry.views }}_*
"#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub header: String,
    pub data_entry: DataEntry,
}

pub fn render(context: Context, lang: Lang) -> anyhow::Result<String> {
    tracing::debug!(lang = lang.as_str(), "template::render::data_entry");

    let context = tera::Context::from_serialize(context).map_err(anyhow::Error::from)?;

    let template = match lang {
        Lang::Ru => RU,
    };

    Tera::one_off(template, &context, false).map_err(anyhow::Error::from)
}
