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
    let key_str = key.to_str().unwrap();

    let liked = db::grid::toggle_like(context.db.clone(), key_str.to_string(), cb.from.id).await?;

    let (header, keyboard) = grid::render_data(key.clone(), false, context)
        .instrument(tracing::info_span!(
            "grid::render_data",
            key = key.to_str().unwrap()
        ))
        .await?;

    tracing::info!(context = "update likes", "tg::edit_message_text");
    tg.edit_message_text(cb.from.id, message_id, header)
        .disable_web_page_preview(true)
        .reply_markup(keyboard)
        .send()
        .await
        .map_err(anyhow::Error::from)?;

    let icon = if liked { "👍" } else { "👎" };

    tracing::info!(msg = icon, "tg::answer_callback_query");
    tg.answer_callback_query(&cb.id)
        .text(icon)
        .send()
        .await
        .map_err(anyhow::Error::from)?;

    Ok(())
}
