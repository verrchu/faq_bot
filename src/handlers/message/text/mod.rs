use crate::{db, feedback, Context};

use teloxide_core::{
    requests::{Request, Requester},
    types::{Message, User},
};
use tracing::{Instrument, Span};

pub async fn handle(
    msg: &Message,
    user: &User,
    text: &str,
    context: Context,
) -> anyhow::Result<()> {
    let tg = context.tg.clone();

    if let Some(feedback_message_id) =
        db::feedback::get_prelude_message_id(context.db.clone(), user.id).await?
    {
        let username = user
            .username
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("user has no username: {:?}", user))?;

        tracing::info!("publish feedback");
        db::feedback::publish(context.db.clone(), username.to_string(), text.to_string()).await?;

        feedback::cleanup(user.id, feedback_message_id, msg.id, context.clone())
            .await
            .map_err(anyhow::Error::from)?;

        {
            let (user_id, span) = (user.id, Span::current());
            tokio::spawn(async move { feedback::ack(user_id, context).instrument(span).await });
        }
    } else {
        tracing::warn!(context = "unexpected message", "tg::delete_message");
        tg.delete_message(user.id, msg.id)
            .send()
            .await
            .map_err(anyhow::Error::from)?;

        tracing::warn!("unexpected message: {}", text);
    }

    Ok(())
}
