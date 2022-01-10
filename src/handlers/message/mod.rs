mod command;
mod text;

use std::sync::Arc;

use crate::Db;

use teloxide_core::{requests::Requester, types::Message, RequestError};
use tracing::Instrument;

pub async fn handle<R: Requester<Err = RequestError> + Send + Sync + 'static>(
    bot: Arc<R>,
    msg: &Message,
    db: Db,
) -> anyhow::Result<()>
where
    R::SendMessage: Send + Sync,
    R::DeleteMessage: Send + Sync,
{
    if let (Some(user), Some(text)) = (msg.from(), msg.text()) {
        let username = user.username.as_ref().map(AsRef::<str>::as_ref);

        match text {
            "/start" => {
                let span =
                    tracing::info_span!("handle_command", username, command = "/start", msg.id);

                command::start::handle(bot, user, db)
                    .instrument(span)
                    .await?;
            }
            text => {
                let span = tracing::info_span!("handle_message", username, msg.id);

                text::handle(bot, msg, user, text, db)
                    .instrument(span)
                    .await?;
            }
        }
    } else {
        tracing::warn!("unexpected message kind: {:?}", msg);
    }

    Ok(())
}
