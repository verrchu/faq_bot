use std::path::PathBuf;

use crate::Db;

use anyhow::Context;
use function_name::named;
use redis::Commands;

#[named]
pub async fn get_key(db: Db, hash: String) -> anyhow::Result<PathBuf> {
    tracing::debug!(hash = hash.as_str(), "db::utils::{}", function_name!());

    tokio::task::spawn_blocking(move || {
        db.pool
            .get()
            .context("get db connection from pool")?
            .hget::<_, _, String>("key_hashes", hash)
            .map(PathBuf::from)
            .context(format!("db::utils::{}", function_name!()))
    })
    .await
    .context(format!("db::utils::{}: await task", function_name!()))?
}
