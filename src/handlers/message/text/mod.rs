mod feedback;

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
    let tg = context.tg.clone();
    let mut db = context.db.clone();

    if let Some(feedback_message_id) = db.get_feedback_message_id(user.id).await? {
        feedback::cleanup(user.id, feedback_message_id, msg.id, context.clone())
            .await
            .map_err(anyhow::Error::from)?;

        {
            let user_id = user.id;
            tokio::spawn(async move { feedback::ack(user_id, context).await });
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
