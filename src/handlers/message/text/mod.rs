use crate::Context;

use teloxide_core::{
    requests::{Request, Requester},
    types::{Message, User},
};

pub async fn handle(
    msg: &Message,
    user: &User,
    text: &str,
    context: Context,
) -> anyhow::Result<()> {
    let tg = context.tg;
    let mut db = context.db;
    let config = context.config;

    if let Some(feedback_message_id) = db.get_feedback_message_id(user.id).await? {
        {
            let user = user.to_owned();
            let msg = msg.to_owned();
            let tg = tg.clone();
            let mut db = db.clone();

            tokio::spawn(async move {
                tg.delete_message(user.id, feedback_message_id)
                    .send()
                    .await
                    .map_err(anyhow::Error::from)?;

                tg.delete_message(user.id, msg.id)
                    .send()
                    .await
                    .map_err(anyhow::Error::from)?;

                db.end_feedback_process(user.id).await?;

                Ok::<_, anyhow::Error>(())
            });
        }

        {
            let user = user.to_owned();
            let tg = tg.clone();

            tokio::spawn(async move {
                let message = tg
                    .send_message(user.id, "FEEDBACK ACCEPTED")
                    .send()
                    .await
                    .map_err(anyhow::Error::from)?;

                tokio::time::sleep(config.feedback.ack_ttl).await;

                tg.delete_message(user.id, message.id)
                    .send()
                    .await
                    .map_err(anyhow::Error::from)?;

                Ok::<_, anyhow::Error>(())
            });
        }
    } else {
        tg.delete_message(user.id, msg.id)
            .send()
            .await
            .map_err(anyhow::Error::from)?;

        tracing::warn!("unexpected message: {}", text);
    }

    Ok(())
}
