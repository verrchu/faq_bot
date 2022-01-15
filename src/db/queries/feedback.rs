use std::ops::DerefMut;

use crate::{types::Feedback, Db};

use anyhow::Context;
use function_name::named;
use redis::Commands;

static FEEDBACK_PENDING: &str = "feedback:pending";
static FEEDBACK_STREAM: &str = "feedback:stream";

#[named]
pub async fn publish(db: Db, username: String, text: String) -> anyhow::Result<String> {
    tracing::debug!(
        username = username.as_str(),
        "db::feedback::{}",
        function_name!()
    );

    let feedback = Feedback::builder().username(username).text(text).build();

    tokio::task::spawn_blocking(move || {
        db.pool
            .get()
            .context("get db connection from pool")?
            .xadd(FEEDBACK_STREAM, "*", &feedback.as_pairs())
            .context(format!("db::feedback::{}", function_name!()))
    })
    .await
    .context(format!("db::feedback::{}: await task", function_name!()))?
}

#[named]
pub async fn is_active(db: Db, user_id: i64) -> anyhow::Result<bool> {
    tracing::debug!(user_id, "db::feedback::{}", function_name!());

    tokio::task::spawn_blocking(move || {
        db.pool
            .get()
            .context("get db connection from pool")?
            .hexists(FEEDBACK_PENDING, user_id)
            .context(format!("db::feedback::{}", function_name!()))
    })
    .await
    .context(format!("db::feedback::{}: await task", function_name!()))?
}

#[named]
pub async fn get_prelude_message_id(db: Db, user_id: i64) -> anyhow::Result<Option<i32>> {
    tracing::debug!(user_id, "db::feedback::{}", function_name!());

    tokio::task::spawn_blocking(move || {
        db.pool
            .get()
            .context("get db connection from pool")?
            .hget(FEEDBACK_PENDING, user_id)
            .context(format!("db::feedback::{}", function_name!()))
    })
    .await
    .context(format!("db::feedback::{}: await task", function_name!()))?
}

#[named]
pub async fn begin(db: Db, user_id: i64, message_id: i32) -> anyhow::Result<bool> {
    tracing::debug!(user_id, message_id, "db::feedback::{}", function_name!());

    tokio::task::spawn_blocking(move || {
        db.pool
            .get()
            .context("get db connection from pool")?
            .hset_nx(FEEDBACK_PENDING, user_id, message_id)
            .context(format!("db::feedback::{}", function_name!()))
    })
    .await
    .context(format!("db::feedback::{}: await task", function_name!()))?
}

#[named]
pub async fn end(db: Db, user_id: i64) -> anyhow::Result<bool> {
    tracing::debug!(user_id, "db::feedback::{}", function_name!());

    tokio::task::spawn_blocking(move || {
        db.pool
            .get()
            .context("get db connection from pool")?
            .hdel(FEEDBACK_PENDING, user_id)
            .context(format!("db::feedback::{}", function_name!()))
    })
    .await
    .context(format!("db::feedback::{}: await task", function_name!()))?
}

#[named]
pub async fn vanish(db: Db) -> anyhow::Result<()> {
    tracing::debug!("db::feedback::{}", function_name!());

    tokio::task::spawn_blocking(move || {
        db.pool
            .get()
            .context("get db connection from pool")?
            .del(FEEDBACK_PENDING)
            .context(format!("db::feedback::{}", function_name!()))
    })
    .await
    .context(format!("db::feedback::{}: await task", function_name!()))?
}

#[named]
pub async fn cancel(db: Db, user: i64) -> anyhow::Result<Option<i32>> {
    tracing::debug!(user, "db::feedback::{}", function_name!());

    tokio::task::spawn_blocking(move || {
        let mut invocation = db.scripts.cancel_feedback.prepare_invoke();
        let mut conn = db.pool.get().context("get db connection from pool")?;

        invocation
            .key(FEEDBACK_PENDING)
            .arg(user)
            .invoke(conn.deref_mut())
            .context(format!("db::feedback::{}", function_name!()))
    })
    .await
    .context(format!("db::feedback::{}: await task", function_name!()))?
}
