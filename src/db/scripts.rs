use std::path::{Path, PathBuf};

use anyhow::Context;
use redis::Script;

pub(super) struct Scripts {
    pub get_grid_header: Script,
    pub toggle_like: Script,
    pub cancel_feedback: Script,
}

impl Scripts {
    pub(super) fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        Ok(Self {
            get_grid_header: load_script(path.as_ref().into(), "get_grid_header")?,
            toggle_like: load_script(path.as_ref().into(), "toggle_like")?,
            cancel_feedback: load_script(path.as_ref().into(), "cancel_feedback")?,
        })
    }
}

fn load_script(mut path: PathBuf, name: &str) -> anyhow::Result<Script> {
    path.push(format!("{}.lua", name));
    let code = std::fs::read_to_string(path.clone()).context(format!(
        "failed to read db script: (name: {}, path: {:?})",
        name, path
    ))?;

    Ok(Script::new(&code))
}
