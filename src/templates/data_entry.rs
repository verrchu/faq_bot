use serde::{Deserialize, Serialize};
use tera::Tera;

static TEMPLATE: &str = r#"
{{ header }}

{{ data }}
"#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub header: String,
    pub data: String,
}

pub fn render(context: Context) -> anyhow::Result<String> {
    let context = tera::Context::from_serialize(context).map_err(anyhow::Error::from)?;
    Tera::one_off(TEMPLATE, &context, false).map_err(anyhow::Error::from)
}
