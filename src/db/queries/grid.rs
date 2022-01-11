use std::collections::{BTreeMap, HashMap};

use crate::{DataEntry, Db};

use function_name::named;
use redis::AsyncCommands;

#[named]
pub async fn inc_views(db: &mut Db, key: &str) -> anyhow::Result<u64> {
    tracing::debug!(key, "db::grid::{}", function_name!());

    db.conn
        .incr(format!("{}:views", key), 1)
        .await
        .map_err(anyhow::Error::from)
}

#[named]
pub async fn get_next_buttons(
    db: &mut Db,
    key: &str,
    lang: &str,
) -> anyhow::Result<BTreeMap<String, String>> {
    tracing::debug!(key, lang, "db::grid::{}", function_name!());

    db.conn
        .hgetall(format!("{}:next:{}", key, lang))
        .await
        .map_err(anyhow::Error::from)
}

#[named]
pub async fn is_data_entry(db: &mut Db, key: &str) -> anyhow::Result<bool> {
    tracing::debug!(key, "db::grid::{}", function_name!());

    db.conn
        .sismember("data_entries", key)
        .await
        .map_err(anyhow::Error::from)
}

#[named]
pub async fn toggle_like(db: &mut Db, key: &str, user: i64) -> anyhow::Result<bool> {
    tracing::debug!(key, user, "dg::grid::{}", function_name!());

    let mut invocation = db.scripts.toggle_like.prepare_invoke();

    invocation
        .arg(key)
        .arg(user)
        .invoke_async(&mut db.conn)
        .await
        .map_err(anyhow::Error::from)
}

#[named]
pub async fn get_grid_header(db: &mut Db, key: &str, lang: &str) -> anyhow::Result<String> {
    tracing::debug!(key, lang, "db::grid::{}", function_name!());

    let mut invocation = db.scripts.get_grid_header.prepare_invoke();

    invocation
        .arg(key)
        .arg(lang)
        .invoke_async(&mut db.conn)
        .await
        .map_err(anyhow::Error::from)
}

#[named]
pub async fn get_data_entry(db: &mut Db, key: &str, lang: &str) -> anyhow::Result<DataEntry> {
    tracing::debug!(key, lang, "call {}", function_name!());

    let mut invocation = db.scripts.get_data_entry.prepare_invoke();

    let raw = invocation
        .arg(key)
        .arg(lang)
        .invoke_async::<_, HashMap<String, String>>(&mut db.conn)
        .await
        .map_err(anyhow::Error::from)?;

    DataEntry::try_from(raw)
}
