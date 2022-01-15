use std::path::PathBuf;

use crate::Db;

use anyhow::Context;
use function_name::named;
use redis::AsyncCommands;

#[named]
pub async fn get_key(db: &mut Db, hash: &str) -> anyhow::Result<PathBuf> {
    tracing::debug!(hash, "db::utils::{}", function_name!());

    db.conn
        .hget::<_, _, String>("key_hashes", hash)
        .await
        .map(PathBuf::from)
        .context(format!("db::utils::{}", function_name!()))
}
