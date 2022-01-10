mod command;
mod text;

use crate::{Db, Tg};

use teloxide_core::types::Message;
use tracing::Instrument;

pub async fn handle(tg: Tg, msg: &Message, db: Db) -> anyhow::Result<()> {
    if let (Some(user), Some(text)) = (msg.from(), msg.text()) {
        let username = user.username.as_ref().map(AsRef::<str>::as_ref);

        match text {
            "/start" => {
                let span =
                    tracing::info_span!("handle_command", username, command = "/start", msg.id);

                command::start::handle(tg, user, db)
                    .instrument(span)
                    .await?;
            }
            text => {
                let span = tracing::info_span!("handle_message", username, msg.id);

                text::handle(tg, msg, user, text, db)
                    .instrument(span)
                    .await?;
            }
        }
    } else {
        tracing::warn!("unexpected message kind: {:?}", msg);
    }

    Ok(())
}
