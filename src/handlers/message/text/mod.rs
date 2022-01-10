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
) -> anyhow::Result<()> {
    bot.delete_message(user.id, msg.id)
        .send()
        .await
        .map_err(anyhow::Error::from)?;

    tracing::warn!("unexpected message: {}", text);

    Ok(())
}
