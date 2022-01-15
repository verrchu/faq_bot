use crate::{db, grid, Context};

use teloxide_core::{
    payloads::setters::*,
    requests::{Request, Requester},
    types::CallbackQuery,
};
use tracing::Instrument;

pub async fn handle(cb: &CallbackQuery, hash: &str, context: Context) -> anyhow::Result<()> {
    let tg = context.tg.clone();

    let message_id = cb
        .message
        .as_ref()
        .map(|message| message.id)
        .ok_or_else(|| anyhow::anyhow!("no message in callback query: {:?}", cb))?;

    let key = db::utils::get_key(context.db.clone(), hash.to_string()).await?;

    let (header, keyboard) = grid::goto(key.clone(), true, context)
        .instrument(tracing::info_span!("grid::goto",))
        .await?;

    tracing::info!(context = "update grid", "tg::edit_message_text");
    tg.edit_message_text(cb.from.id, message_id, header)
        .disable_web_page_preview(true)
        .reply_markup(keyboard)
        .send()
        .await
        .map_err(anyhow::Error::from)?;

    tracing::debug!("tg::answer_callback_query");
    tg.answer_callback_query(&cb.id).send().await?;

    Ok(())
}
