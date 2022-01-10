use crate::Context;

use teloxide_core::requests::{Request, Requester};

pub async fn cleanup(
    user_id: i64,
    fb_req_msg_id: i32,
    fb_res_msg_id: i32,
    context: Context,
) -> anyhow::Result<()> {
    let tg = context.tg;
    let mut db = context.db;

    tg.delete_message(user_id, fb_req_msg_id)
        .send()
        .await
        .map_err(anyhow::Error::from)?;

    tg.delete_message(user_id, fb_res_msg_id)
        .send()
        .await
        .map_err(anyhow::Error::from)?;

    db.end_feedback_process(user_id).await?;

    Ok(())
}

pub async fn ack(user_id: i64, context: Context) -> anyhow::Result<()> {
    let config = context.config;

    let tg = context.tg;

    let message = tg
        .send_message(user_id, "FEEDBACK ACCEPTED")
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
