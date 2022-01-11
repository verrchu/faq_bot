use std::{collections::HashMap, path::PathBuf};

use crate::{DataEntry, Db};

use function_name::named;
use redis::Script;

pub(super) struct Scripts {
    get_grid_header: Script,
    get_data_entry: Script,
    toggle_like: Script,
    cancel_feedback: Script,
}

impl Scripts {
    pub(super) fn load(path: PathBuf) -> anyhow::Result<Self> {
        Ok(Self {
            get_grid_header: load_script(path.clone(), "get_grid_header")?,
            get_data_entry: load_script(path.clone(), "get_data_entry")?,
            toggle_like: load_script(path.clone(), "toggle_like")?,
            cancel_feedback: load_script(path, "cancel_feedback")?,
        })
    }
}

fn load_script(mut path: PathBuf, name: &str) -> anyhow::Result<Script> {
    path.push(format!("{}.lua", name));
    let code = std::fs::read_to_string(path).map_err(anyhow::Error::from)?;

    Ok(Script::new(&code))
}

impl Db {
    #[named]
    pub async fn get_data_entry(&mut self, key: &str, lang: &str) -> anyhow::Result<DataEntry> {
        tracing::debug!(key, lang, "call {}", function_name!());

        let mut invocation = self.scripts.get_data_entry.prepare_invoke();

        let raw = invocation
            .arg(key)
            .arg(lang)
            .invoke_async::<_, HashMap<String, String>>(&mut self.conn)
            .await
            .map_err(anyhow::Error::from)?;

        DataEntry::try_from(raw)
    }

    #[named]
    pub async fn get_grid_header(&mut self, key: &str, lang: &str) -> anyhow::Result<String> {
        tracing::debug!(key, lang, "call {}", function_name!());

        let mut invocation = self.scripts.get_grid_header.prepare_invoke();

        invocation
            .arg(key)
            .arg(lang)
            .invoke_async(&mut self.conn)
            .await
            .map_err(anyhow::Error::from)
    }

    #[named]
    pub async fn toggle_like(&mut self, key: &str, user: i64) -> anyhow::Result<bool> {
        tracing::debug!(key, user, "call {}", function_name!());

        let mut invocation = self.scripts.toggle_like.prepare_invoke();

        invocation
            .arg(key)
            .arg(user)
            .invoke_async(&mut self.conn)
            .await
            .map_err(anyhow::Error::from)
    }

    #[named]
    pub async fn cancel_feedback(&mut self, user: i64) -> anyhow::Result<Option<i32>> {
        tracing::debug!(user, "call {}", function_name!());

        let mut invocation = self.scripts.cancel_feedback.prepare_invoke();

        invocation
            .arg(user)
            .invoke_async(&mut self.conn)
            .await
            .map_err(anyhow::Error::from)
    }
}
