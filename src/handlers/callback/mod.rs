mod feedback;
mod goto;
mod like;

use crate::Context;

use teloxide_core::{
    requests::{Request, Requester},
    types::CallbackQuery,
};
use tracing::Instrument;

pub async fn handle(cb: &CallbackQuery, context: Context) -> anyhow::Result<()> {
    crate::feedback::cancel(cb.from.id, context.clone()).await?;

    if let Some(data) = &cb.data {
        let username = cb.from.username.as_ref().map(AsRef::<str>::as_ref);
        let lang = context.lang.as_str();

        if let Some(hash) = data.strip_prefix("/goto#") {
            let span = tracing::info_span!(
                "handle_query",
                username,
                query = "/goto",
                query.id = cb.id.as_str(),
                lang,
            );

            goto::handle(cb, hash, context).instrument(span).await?;
        } else if let Some(hash) = data.strip_prefix("/like#") {
            let span = tracing::info_span!(
                "handle_query",
                username,
                query = "/like",
                query.id = cb.id.as_str(),
                lang
            );

            like::handle(cb, hash, context).instrument(span).await?;
        } else if data == "/feedback" {
            // TODO: maybe pass hash as context
            let span = tracing::info_span!(
                "handle_query",
                username,
                query = "/feedback",
                query.id = cb.id.as_str(),
                lang,
            );

            feedback::handle(cb, context).instrument(span).await?;
        } else {
            tracing::warn!("unexpected callback query: {}", data);

            tracing::debug!("tg::answer_callback_query");
            context.tg.answer_callback_query(&cb.id).send().await?;
        }
    }

    Ok(())
}
