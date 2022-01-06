use crate::{grid, utils};

use teloxide_core::{
    payloads::setters::*,
    requests::{Request, Requester},
    types::Message,
    RequestError,
};

pub async fn handle_message<R: Requester<Err = RequestError>>(
    bot: R,
    message: &Message,
) -> anyhow::Result<()> {
    if let (Some(user), Some(text)) = (message.from(), message.text()) {
        match text {
            "/start" => {
                let (header, keyboard) = grid::goto(utils::hash("/")).await?;

                bot.send_message(user.id, header)
                    .reply_markup(keyboard)
                    .send()
                    .await?;
            }
            _ => tracing::warn!("unexpected message: {}", text),
        }
    } else {
        tracing::warn!("unexpected message kind: {:?}", message);
    }

    Ok(())
}
