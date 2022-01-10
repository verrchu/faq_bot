mod command;
mod text;

use crate::Context;

use teloxide_core::types::Message;
use tracing::Instrument;

pub async fn handle(msg: &Message, context: Context) -> anyhow::Result<()> {
    if let (Some(user), Some(text)) = (msg.from(), msg.text()) {
        let username = user.username.as_ref().map(AsRef::<str>::as_ref);

        match text {
            "/start" => {
                let span =
                    tracing::info_span!("handle_command", username, command = "/start", msg.id);

                command::start::handle(user, context)
                    .instrument(span)
                    .await?;
            }
            text => {
                let span = tracing::info_span!("handle_message", username, msg.id);

                text::handle(msg, user, text, context)
                    .instrument(span)
                    .await?;
            }
        }
    } else {
        tracing::warn!("unexpected message kind: {:?}", msg);
    }

    Ok(())
}
