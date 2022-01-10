use crate::{grid, Context};

use teloxide_core::{
    payloads::setters::*,
    requests::{Request, Requester},
    types::CallbackQuery,
};
use tracing::Instrument;

pub async fn handle(cb: &CallbackQuery, hash: &str, context: Context) -> anyhow::Result<()> {
    let mut db = context.db;
    let tg = context.tg;

    let message_id = cb
        .message
        .as_ref()
        .map(|message| message.id)
        .ok_or_else(|| anyhow::anyhow!("no message in callback query: {:?}", cb))?;

    let key = db.get_key(hash).await?;

    let (header, keyboard) = grid::goto(key.clone(), true, db)
        .instrument(tracing::trace_span!(
            "grid_goto",
            key = key.to_str().unwrap()
        ))
        .await?;

    tg.edit_message_text(cb.from.id, message_id, header)
        .disable_web_page_preview(true)
        .reply_markup(keyboard)
        .send()
        .await
        .map_err(anyhow::Error::from)?;

    tg.answer_callback_query(&cb.id).send().await?;

    Ok(())
}
