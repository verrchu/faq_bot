use crate::{grid, Db};

use teloxide_core::{
    payloads::setters::*,
    requests::{Request, Requester},
    types::CallbackQuery,
    RequestError,
};
use tracing::Instrument;

pub async fn handle_callback_query<R: Requester<Err = RequestError>>(
    bot: R,
    cb: &CallbackQuery,
    mut db: Db,
) -> anyhow::Result<()> {
    if let (Some(msg), Some(data)) = (&cb.message, &cb.data) {
        let username = cb.from.username.as_ref().map(AsRef::<str>::as_ref);
        let user_id = u64::try_from(cb.from.id).map_err(anyhow::Error::from)?;

        if let Some(hash) = data.strip_prefix("/goto#") {
            let span = tracing::trace_span!(
                "handle_query",
                username,
                query = "/goto",
                query.id = cb.id.as_str()
            );

            let block = async {
                let key = db.get_key(hash).await?;

                let (header, keyboard) = grid::goto(key.clone(), db)
                    .instrument(tracing::trace_span!(
                        "grid_goto",
                        key = key.to_str().unwrap()
                    ))
                    .await?;

                bot.edit_message_text(cb.from.id, msg.id, header)
                    .disable_web_page_preview(true)
                    .reply_markup(keyboard)
                    .send()
                    .await
                    .map_err(anyhow::Error::from)
            };

            block.instrument(span).await?;

            bot.answer_callback_query(&cb.id).send().await?;
        } else if let Some(hash) = data.strip_prefix("/like#") {
            let username = cb.from.username.as_ref().map(AsRef::<str>::as_ref);

            let span = tracing::trace_span!(
                "handle_query",
                username,
                query = "/like",
                query.id = cb.id.as_str()
            );

            let block = async {
                let key = db.get_key(hash).await?;
                let key_str = key.to_str().unwrap();

                let liked = db.toggle_like(key_str, user_id).await?;

                let (header, keyboard) = grid::goto(key.clone(), db)
                    .instrument(tracing::trace_span!(
                        "grid_goto",
                        key = key.to_str().unwrap()
                    ))
                    .await?;

                bot.edit_message_text(cb.from.id, msg.id, header)
                    .disable_web_page_preview(true)
                    .reply_markup(keyboard)
                    .send()
                    .await
                    .map_err(anyhow::Error::from)?;

                let text = if liked { "👍" } else { "👎" };

                bot.answer_callback_query(&cb.id)
                    .text(text)
                    .send()
                    .await
                    .map_err(anyhow::Error::from)
            };

            block.instrument(span).await?;
        } else {
            tracing::warn!("unexpected callback query: {}", data);

            bot.answer_callback_query(&cb.id).send().await?;
        }
    }

    Ok(())
}
