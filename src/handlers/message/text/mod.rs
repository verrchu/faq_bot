use crate::Db;

use teloxide_core::{
    requests::{Request, Requester},
    types::{Message, User},
    RequestError,
};

pub async fn handle<R: Requester<Err = RequestError>>(
    bot: R,
    msg: &Message,
    user: &User,
    text: &str,
    mut db: Db,
) -> anyhow::Result<()> {
    if let Some(feedback_message_id) = db.get_feedback_message_id(user.id).await? {
        bot.delete_message(user.id, feedback_message_id)
            .send()
            .await
            .map_err(anyhow::Error::from)?;

        bot.delete_message(user.id, msg.id)
            .send()
            .await
            .map_err(anyhow::Error::from)?;

        db.end_feedback_process(user.id).await?;
    } else {
        bot.delete_message(user.id, msg.id)
            .send()
            .await
            .map_err(anyhow::Error::from)?;

        tracing::warn!("unexpected message: {}", text);
    }

    Ok(())
}
