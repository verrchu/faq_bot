mod goto;
mod like;

use crate::Db;

use teloxide_core::{
    requests::{Request, Requester},
    types::CallbackQuery,
    RequestError,
};
use tracing::Instrument;

pub async fn handle<R: Requester<Err = RequestError>>(
    bot: R,
    cb: &CallbackQuery,
    db: Db,
) -> anyhow::Result<()> {
    if let Some(data) = &cb.data {
        let username = cb.from.username.as_ref().map(AsRef::<str>::as_ref);

        if let Some(hash) = data.strip_prefix("/goto#") {
            let span = tracing::trace_span!(
                "handle_query",
                username,
                query = "/goto",
                query.id = cb.id.as_str()
            );

            goto::handle(bot, cb, hash, db).instrument(span).await?;
        } else if let Some(hash) = data.strip_prefix("/like#") {
            let span = tracing::trace_span!(
                "handle_query",
                username,
                query = "/like",
                query.id = cb.id.as_str()
            );

            like::handle(bot, cb, hash, db).instrument(span).await?;
        } else {
            tracing::warn!("unexpected callback query: {}", data);

            bot.answer_callback_query(&cb.id).send().await?;
        }
    }

    Ok(())
}
