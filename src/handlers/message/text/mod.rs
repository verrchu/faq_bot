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
        {
            let (user_id, msg_id, context) = (user.id, msg.id, context.clone());
            tokio::spawn(async move {
                feedback::cleanup(user_id, feedback_message_id, msg_id, context).await
            });
        }

        {
            let (user_id, context) = (user.id, context);
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
