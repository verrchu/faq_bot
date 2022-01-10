use std::{sync::Arc, time::Duration};

use crate::Db;

use teloxide_core::{
    requests::{Request, Requester},
    types::{Message, User},
    RequestError,
};

pub async fn handle<R: Requester<Err = RequestError> + Send + Sync + 'static>(
    bot: Arc<R>,
    msg: &Message,
    user: &User,
    text: &str,
    mut db: Db,
) -> anyhow::Result<()>
where
    R::SendMessage: Send + Sync,
    R::DeleteMessage: Send + Sync,
{
    if let Some(feedback_message_id) = db.get_feedback_message_id(user.id).await? {
        {
            let user = user.to_owned();
            let msg = msg.to_owned();
            let bot = bot.clone();
            let mut db = db.clone();

            tokio::spawn(async move {
                bot.delete_message(user.id, feedback_message_id)
                    .send()
                    .await
                    .map_err(anyhow::Error::from)?;

                bot.delete_message(user.id, msg.id)
                    .send()
                    .await
                    .map_err(anyhow::Error::from)?;

                db.end_feedback_process(user.id).await?;

                Ok::<_, anyhow::Error>(())
            });
        }

        {
            let user = user.to_owned();
            let bot = bot.clone();

            tokio::spawn(async move {
                let message = bot
                    .send_message(user.id, "TEST")
                    .send()
                    .await
                    .map_err(anyhow::Error::from)?;

                tokio::time::sleep(Duration::from_secs(3)).await;

                bot.delete_message(user.id, message.id)
                    .send()
                    .await
                    .map_err(anyhow::Error::from)?;

                Ok::<_, anyhow::Error>(())
            });
        }
    } else {
        bot.delete_message(user.id, msg.id)
            .send()
            .await
            .map_err(anyhow::Error::from)?;

        tracing::warn!("unexpected message: {}", text);
    }

    Ok(())
}
