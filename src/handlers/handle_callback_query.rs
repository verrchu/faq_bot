use crate::{grid, Db};

use teloxide_core::{
    payloads::setters::*,
    requests::{Request, Requester},
    types::CallbackQuery,
    RequestError,
};

pub async fn handle_callback_query<R: Requester<Err = RequestError>>(
    bot: R,
    cb: &CallbackQuery,
    db: Db,
) -> anyhow::Result<()> {
    if let Some(data) = &cb.data {
        if let Some(hash) = data.strip_prefix("/goto#") {
            let (header, keyboard) = grid::goto(hash, db).await?;

            bot.send_message(cb.from.id, header)
                .reply_markup(keyboard)
                .send()
                .await?;
        } else {
            tracing::warn!("unexpected callback query: {}", data);
        }
    }

    Ok(())
}
