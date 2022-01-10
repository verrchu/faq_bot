use crate::{grid, Db};

use teloxide_core::{
    payloads::setters::*,
    requests::{Request, Requester},
    types::CallbackQuery,
    RequestError,
};
use tracing::Instrument;

pub async fn handle<R: Requester<Err = RequestError>>(
    bot: R,
    cb: &CallbackQuery,
    hash: &str,
    mut db: Db,
) -> anyhow::Result<()> {
    let message_id = cb
        .message
        .as_ref()
        .map(|message| message.id)
        .ok_or_else(|| anyhow::anyhow!("no message in callback query: {:?}", cb))?;

    let key = db.get_key(hash).await?;
    let key_str = key.to_str().unwrap();

    let liked = db.toggle_like(key_str, cb.from.id).await?;

    let (header, keyboard) = grid::goto(key.clone(), false, db)
        .instrument(tracing::trace_span!(
            "grid_goto",
            key = key.to_str().unwrap()
        ))
        .await?;

    bot.edit_message_text(cb.from.id, message_id, header)
        .disable_web_page_preview(true)
        .reply_markup(keyboard)
        .send()
        .await
        .map_err(anyhow::Error::from)?;

    let icon = if liked { "👍" } else { "👎" };

    bot.answer_callback_query(&cb.id)
        .text(icon)
        .send()
        .await
        .map_err(anyhow::Error::from)?;

    Ok(())
}
