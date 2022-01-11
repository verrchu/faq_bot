use crate::Db;

use function_name::named;
use redis::AsyncCommands;

#[named]
pub async fn is_active(db: &mut Db, user_id: i64) -> anyhow::Result<bool> {
    tracing::debug!(user_id, "call {}", function_name!());

    db.conn
        .hexists("feedback", user_id)
        .await
        .map_err(anyhow::Error::from)
}

#[named]
pub async fn get_prelude_message_id(db: &mut Db, user_id: i64) -> anyhow::Result<Option<i32>> {
    tracing::debug!(user_id, "db::feedback::{}", function_name!());

    db.conn
        .hget("feedback", user_id)
        .await
        .map_err(anyhow::Error::from)
}

#[named]
pub async fn begin(db: &mut Db, user_id: i64, message_id: i32) -> anyhow::Result<bool> {
    tracing::debug!(user_id, message_id, "db::feedback::{}", function_name!());

    db.conn
        .hset_nx("feedback", user_id, message_id)
        .await
        .map_err(anyhow::Error::from)
}

#[named]
pub async fn end(db: &mut Db, user_id: i64) -> anyhow::Result<bool> {
    tracing::debug!(user_id, "db::feedback::{}", function_name!());

    db.conn
        .hdel("feedback", user_id)
        .await
        .map_err(anyhow::Error::from)
}

#[named]
pub async fn cancel(db: &mut Db, user: i64) -> anyhow::Result<Option<i32>> {
    tracing::debug!(user, "db::feedback::{}", function_name!());

    let mut invocation = db.scripts.cancel_feedback.prepare_invoke();

    invocation
        .arg(user)
        .invoke_async(&mut db.conn)
        .await
        .map_err(anyhow::Error::from)
}
