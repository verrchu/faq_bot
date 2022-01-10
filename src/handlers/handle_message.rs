use crate::{grid, Db};

use teloxide_core::{
    payloads::setters::*,
    requests::{Request, Requester},
    types::Message,
    RequestError,
};
use tracing::Instrument;

pub async fn handle_message<R: Requester<Err = RequestError>>(
    bot: R,
    message: &Message,
    db: Db,
) -> anyhow::Result<()> {
    if let (Some(user), Some(text)) = (message.from(), message.text()) {
        let username = user.username.as_ref().map(AsRef::<str>::as_ref);

        match text {
            "/start" => {
                let span = tracing::info_span!("handle_message", username, text, message.id);

                let block = async {
                    let (header, keyboard) = grid::init(db)
                        .instrument(tracing::info_span!("grid_init"))
                        .await?;

                    bot.send_message(user.id, header)
                        .disable_web_page_preview(true)
                        .reply_markup(keyboard)
                        .send()
                        .await
                        .map(|_| ())
                        .map_err(anyhow::Error::from)
                };

                block.instrument(span).await?;
            }
            _ => {
                bot.delete_message(user.id, message.id)
                    .send()
                    .await
                    .map_err(anyhow::Error::from)?;

                tracing::warn!("unexpected message: {}", text);
            }
        }
    } else {
        tracing::warn!("unexpected message kind: {:?}", message);
    }

    Ok(())
}
