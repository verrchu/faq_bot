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

    if db::grid::is_data_entry(context.db.clone(), key.to_string_lossy().to_string()).await? {
        if let Err(err) = tg.delete_message(cb.from.id, message_id).send().await {
            tracing::warn!("failed to delete message: {:?}", err);
        }

        let (header, keyboard) = grid::render_data(key.clone(), true, context)
            .instrument(tracing::info_span!("grid::render_data"))
            .await?;

        tracing::debug!("tg::send_message");
        tg.send_message(cb.from.id, header)
            .disable_web_page_preview(true)
            .disable_notification(true)
            .reply_markup(keyboard)
            .send()
            .await
            .map_err(anyhow::Error::from)?;
    } else {
        let (header, keyboard) = grid::render_menu(key.clone(), context)
            .instrument(tracing::info_span!("grid::render_menu"))
            .await?;

        tracing::debug!("tg::edit_message_text");
        tg.edit_message_text(cb.from.id, message_id, header)
            .reply_markup(keyboard)
            .send()
            .await
            .map_err(anyhow::Error::from)?;
    };

    tracing::debug!("tg::answer_callback_query");
    tg.answer_callback_query(&cb.id).send().await?;

    Ok(())
}
