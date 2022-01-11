use std::path::PathBuf;

use crate::Db;

use function_name::named;
use redis::AsyncCommands;

#[named]
pub async fn get_key(db: &mut Db, hash: &str) -> anyhow::Result<PathBuf> {
    tracing::debug!(hash, "db::grid::{}", function_name!());

    db.conn
        .hget::<_, _, String>("key_hashes", hash)
        .await
        .map(PathBuf::from)
        .map_err(anyhow::Error::from)
}
