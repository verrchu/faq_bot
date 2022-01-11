use crate::{l10n, Context};

use futures::try_join;
use teloxide_core::requests::{Request, Requester};

pub async fn cancel(user_id: i64, context: Context) -> anyhow::Result<()> {
    let tg = context.tg;
    let mut db = context.db;
    let config = context.config;

    tokio::time::sleep(config.feedback.timeout).await;

    if let Some(fb_req_msg_id) = db.cancel_feedback(user_id).await? {
        tg.delete_message(user_id, fb_req_msg_id)
            .send()
            .await
            .map_err(anyhow::Error::from)?;
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
        tg.delete_message(user_id, fb_req_msg_id).send(),
        tg.delete_message(user_id, fb_res_msg_id).send()
    )
    .map_err(anyhow::Error::from)?;

    db.end_feedback(user_id).await?;

    Ok(())
}

pub async fn ack(user_id: i64, context: Context) -> anyhow::Result<()> {
    let config = context.config;
    let lang = context.lang;

    let tg = context.tg;

    let message = tg
        .send_message(user_id, l10n::messages::feedback_ack(lang))
        .send()
        .await
        .map_err(anyhow::Error::from)?;

    tokio::time::sleep(config.feedback.ack_ttl).await;

    tg.delete_message(user_id, message.id)
        .send()
        .await
        .map_err(anyhow::Error::from)?;

    Ok(())
}
