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
                        .reply_markup(keyboard)
                        .send()
                        .await
                        .map(|_| ())
                        .map_err(anyhow::Error::from)
                };

                block.instrument(span).await?;
            }
            _ => tracing::warn!("unexpected message: {}", text),
        }
    } else {
        tracing::warn!("unexpected message kind: {:?}", message);
    }

    Ok(())
}
