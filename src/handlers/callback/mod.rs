mod feedback;
mod goto;
mod like;

use crate::{Db, Tg};

use teloxide_core::{
    requests::{Request, Requester},
    types::CallbackQuery,
};
use tracing::Instrument;

pub async fn handle(tg: Tg, cb: &CallbackQuery, db: Db) -> anyhow::Result<()> {
    if let Some(data) = &cb.data {
        let username = cb.from.username.as_ref().map(AsRef::<str>::as_ref);

        if let Some(hash) = data.strip_prefix("/goto#") {
            let span = tracing::trace_span!(
                "handle_query",
                username,
                query = "/goto",
                query.id = cb.id.as_str()
            );

            goto::handle(tg, cb, hash, db).instrument(span).await?;
        } else if let Some(hash) = data.strip_prefix("/like#") {
            let span = tracing::trace_span!(
                "handle_query",
                username,
                query = "/like",
                query.id = cb.id.as_str()
            );

            like::handle(tg, cb, hash, db).instrument(span).await?;
        } else if data == "/feedback" {
            // TODO: maybe pass hash as context
            let span = tracing::trace_span!(
                "handle_query",
                username,
                query = "/feedback",
                query.id = cb.id.as_str()
            );

            feedback::handle(tg, cb, db).instrument(span).await?;
        } else {
            tracing::warn!("unexpected callback query: {}", data);

            tg.answer_callback_query(&cb.id).send().await?;
        }
    }

    Ok(())
}
