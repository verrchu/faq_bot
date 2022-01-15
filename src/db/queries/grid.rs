use std::{collections::BTreeMap, ops::DerefMut};

use crate::{utils, DataEntry, Db};

use anyhow::Context;
use function_name::named;
use redis::Commands;

#[named]
pub async fn inc_views(db: Db, key: String) -> anyhow::Result<u64> {
    tracing::debug!(key = key.as_str(), "db::grid::{}", function_name!());

    tokio::task::spawn_blocking(move || {
        db.pool
            .get()
            .context("get db connection from pool")?
            .incr(format!("{{l10n:none}}:{}:views", key), 1)
            .context(format!("db::grid::{}", function_name!()))
    })
    .await
    .context(format!("db::grid::{}: await task", function_name!()))?
}

#[named]
pub async fn get_next_buttons(
    db: Db,
    key: String,
    lang: String,
) -> anyhow::Result<BTreeMap<String, String>> {
    tracing::debug!(
        key = key.as_str(),
        lang = lang.as_str(),
        "db::grid::{}",
        function_name!()
    );

    tokio::task::spawn_blocking(move || {
        db.pool
            .get()
            .context("get db connection from pool")?
            .hgetall(format!("{{l10n:{lang}}}:{key}:next"))
            .context(format!("db::grid::{}", function_name!()))
    })
    .await
    .context(format!("db::grid::{}: await task", function_name!()))?
}

#[named]
pub async fn is_data_entry(db: Db, key: String) -> anyhow::Result<bool> {
    tracing::debug!(key = key.as_str(), "db::grid::{}", function_name!());

    tokio::task::spawn_blocking(move || {
        db.pool
            .get()
            .context("get db connection from pool")?
            .sismember("data_entries", key)
            .context(format!("db::grid::{}", function_name!()))
    })
    .await
    .context(format!("db::grid::{}: await task", function_name!()))?
}

#[named]
pub async fn toggle_like(db: Db, key: String, user: i64) -> anyhow::Result<bool> {
    tracing::debug!(key = key.as_str(), user, "dg::grid::{}", function_name!());

    tokio::task::spawn_blocking(move || {
        let mut invocation = db.scripts.toggle_like.prepare_invoke();
        let mut conn = db.pool.get().context("get db connection from pool")?;

        invocation
            .key("l10n:none")
            .arg(key)
            .arg(user)
            .invoke(conn.deref_mut())
            .context(format!("db::grid::{}", function_name!()))
    })
    .await
    .context(format!("db::grid::{}: await task", function_name!()))?
}

#[named]
pub async fn get_grid_header(db: Db, key: String, lang: String) -> anyhow::Result<String> {
    tracing::debug!(
        key = key.as_str(),
        lang = lang.as_str(),
        "db::grid::{}",
        function_name!()
    );

    tokio::task::spawn_blocking(move || {
        let mut invocation = db.scripts.get_grid_header.prepare_invoke();
        let mut conn = db.pool.get().context("get db connection from pool")?;

        invocation
            .key(format!("l10n:{lang}"))
            .arg(key)
            .arg(lang)
            .invoke(conn.deref_mut())
            .context(format!("db::grid::{}", function_name!()))
    })
    .await
    .context(format!("db::grid::{}: await task", function_name!()))?
}

#[named]
pub async fn get_data_entry(db: Db, key: String, lang: String) -> anyhow::Result<DataEntry> {
    tracing::debug!(
        key = key.as_str(),
        lang = lang.as_str(),
        "db::grid::{}",
        function_name!()
    );

    tokio::task::spawn_blocking(move || {
        let mut conn = db.pool.get().context("get db connection from pool")?;

        let (data, created, likes, views): (String, u32, u32, u32) = redis::cluster::cluster_pipe()
            .get(format!("{{l10n:{lang}}}:{key}:data"))
            .get(format!("{{l10n:none}}:{key}:created"))
            .scard(format!("{{l10n:none}}:{key}:likes"))
            .get(format!("{{l10n:none}}:{key}:views"))
            .query(conn.deref_mut())
            .context(format!("db::grid::{}", function_name!()))?;

        Ok(DataEntry::builder()
            .text(data)
            .created(utils::unixtime_to_datetime(created))
            .likes(likes)
            .views(views)
            .build())
    })
    .await
    .context(format!("db::grid::{}: await task", function_name!()))?
}
