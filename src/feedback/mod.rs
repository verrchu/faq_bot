use crate::{l10n, Context};

use futures::try_join;
use humantime::format_duration;
use teloxide_core::requests::{Request, Requester};

pub async fn cancel(user_id: i64, context: Context) -> anyhow::Result<()> {
    let tg = context.tg;
    let mut db = context.db;

    let timeout = context.config.feedback.timeout;
    let timeout_str = format_duration(timeout).to_string();

    tracing::info!(delay = timeout_str.as_str(), "scheduling feedback::cancel");

    tokio::time::sleep(timeout).await;

    if let Some(fb_req_msg_id) = db.cancel_feedback(user_id).await? {
        tracing::info!(context = "feedback_cancel", "tg::delete_message");
        tg.delete_message(user_id, fb_req_msg_id)
            .send()
            .await
            .map_err(anyhow::Error::from)?;
    } else {
        tracing::info!(context = "feedback_cancel", "feedback already cancelled");
    }

    Ok(())
}

pub async fn cleanup(
    user_id: i64,
    fb_req_msg_id: i32,
    fb_res_msg_id: i32,
    context: Context,
) -> anyhow::Result<()> {
    let tg = context.tg;
    let mut db = context.db;

    try_join!(
        {
            tracing::info!(context = "feedback_cleanup (req)", "tg::delete_message");
            tg.delete_message(user_id, fb_req_msg_id).send()
        },
        {
            tracing::info!(context = "feedback_cleanup (res)", "tg::delete_message");
            tg.delete_message(user_id, fb_res_msg_id).send()
        }
    )
    .map_err(anyhow::Error::from)?;

    db.end_feedback(user_id).await?;

    Ok(())
}

pub async fn ack(user_id: i64, context: Context) -> anyhow::Result<()> {
    let lang = context.lang;

    let tg = context.tg;

    tracing::info!(context = "feedback_ack", "tg::send_message");
    let message = tg
        .send_message(user_id, l10n::messages::feedback_ack(lang))
        .send()
        .await
        .map_err(anyhow::Error::from)?;

    let ttl = context.config.feedback.ack_ttl;
    let ttl_str = format_duration(ttl).to_string();

    tracing::debug!(delay = ttl_str.as_str(), "scheduling ack message removal");
    tokio::time::sleep(ttl).await;

    tracing::info!(context = "feedback_ack", "tg::delete_message");
    tg.delete_message(user_id, message.id)
        .send()
        .await
        .map_err(anyhow::Error::from)?;

    Ok(())
}
