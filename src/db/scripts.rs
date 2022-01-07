use std::{path::PathBuf};

use super::Db;

use redis::Script;

pub(super) struct Scripts {
    get_grid_header: Script,
}

impl Scripts {
    pub(super) fn load(path: PathBuf) -> anyhow::Result<Self> {
        Ok(Self {
            get_grid_header: load_script(path, "get_grid_header")?,
        })
    }
}

fn load_script(mut path: PathBuf, name: &str) -> anyhow::Result<Script> {
    path.push(format!("{}.lua", name));
    let code = std::fs::read_to_string(path).map_err(anyhow::Error::from)?;

    Ok(Script::new(&code))
}

impl Db {
    pub async fn get_grid_header(&mut self, hash: &str, lang: &str) -> anyhow::Result<String> {
        let mut invocation = self.scripts.get_grid_header.prepare_invoke();

        invocation
            .arg(hash)
            .arg(lang)
            .invoke_async(&mut self.conn)
            .await
            .map_err(anyhow::Error::from)
    }
}
