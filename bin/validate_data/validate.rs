use std::{
    collections::HashSet,
    fs::{read_dir, DirEntry},
    io,
    path::{Component, Path, PathBuf},
};

use anyhow::anyhow;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{AnyError, AnyResult};

static NAME_DIR: &str = "_name";
static DATA_DIR: &str = "_data";

static PATH_SEGMENT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(^[0-9a-z][0-9a-z_]*[0-9a-z]$|^[a-z0-9]+$)").unwrap());

pub fn validate(path: impl AsRef<Path>) -> AnyResult<()> {
    let mut path_acc = Path::new("/");

    tracing::info!("Validating data in {:?}", path_acc);

    let child_nodes = child_nodes(path.as_ref())?;

    tracing::debug!("Root level nodes: {:?}", child_nodes);

    for child_path in child_nodes.iter() {
        let child_last_segment = child_path.file_name().ok_or(anyhow!(
            "Failed to get child path final segment: {:?}",
            child_path
        ))?;

        validate_path_segment_name(&child_last_segment, &path_acc)?;

        let mut path_acc = path_acc.to_path_buf();
        path_acc.push(&child_last_segment);

        validate_child(child_path, path_acc);
    }

    Ok(())
}

fn validate_child(child_path: impl AsRef<Path>, path_acc: impl AsRef<Path>) -> AnyResult<()> {
    tracing::info!("Validating data in {:?}", path_acc.as_ref());

    let mut child_nodes = child_nodes(child_path.as_ref())?;

    let name_dir = {
        let mut child_path = child_path.as_ref().to_path_buf();
        child_path.push(NAME_DIR);
        child_path
    };

    if !child_nodes.contains(&name_dir) {
        tracing::error!("_name does not exist (path: {:?})", path_acc.as_ref());
    }
    // validate_name_localization(&child_path, &path_acc)?;

    todo!();
}

fn validate_path_segment_name(segment: impl AsRef<Path>, path: impl AsRef<Path>) -> AnyResult<()> {
    let segment_str = segment.as_ref().to_str().ok_or(anyhow!(
        "Failed to convert path segment to str: (segment: {:?}, path: {:?})",
        segment.as_ref(),
        path.as_ref()
    ))?;

    if !PATH_SEGMENT_RE.is_match(segment_str) {
        tracing::error!(
            "path segment has invalid name: (segment: {:?}, path: {:?})",
            segment.as_ref(),
            path.as_ref()
        );
    }

    Ok(())
}

fn child_nodes(path: impl AsRef<Path>) -> Result<HashSet<PathBuf>, io::Error> {
    read_dir(path)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect()
}
