use std::path::PathBuf;

use redis::Script;

pub(super) struct Scripts {
    pub get_grid_header: Script,
    pub get_data_entry: Script,
    pub toggle_like: Script,
    pub cancel_feedback: Script,
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
