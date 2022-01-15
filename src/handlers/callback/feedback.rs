use crate::{db, feedback, l10n, Context};

use humantime::format_duration;
use teloxide_core::{
    requests::{Request, Requester},
    types::CallbackQuery,
};
use tracing::{Instrument, Span};

pub async fn handle(cb: &CallbackQuery, context: Context) -> anyhow::Result<()> {
    let tg = context.tg.clone();
    let lang = context.lang;

    let is_active = db::feedback::is_active(context.db.clone(), cb.from.id).await?;

    // TODO: signal in query response that feedback is in progress
    if !is_active {
        {
            let (user_id, context, span) = (cb.from.id, context.clone(), Span::current());
            tokio::spawn(async move {
                let timeout = context.config.feedback.timeout;
                let timeout_str = format_duration(timeout).to_string();

                tracing::debug!(delay = timeout_str.as_str(), "scheduling feedback::cancel");

                tokio::time::sleep(timeout).await;

                feedback::cancel(user_id, context).instrument(span).await
            });
        }

        tracing::info!(context = "feedback_prelude", "tg::send_message");
        let message = tg
            .send_message(cb.from.id, l10n::messages::feedback_prelude(lang))
            .send()
            .await
            .map_err(anyhow::Error::from)?;

        let inited = db::feedback::begin(context.db.clone(), cb.from.id, message.id).await?;

        // there might be a rare condition when one user tries to submit
        // feedback from several devices.
        // "inited" is supposed to handle this issue.
        if !inited {
            tracing::info!(context = "feedback_prelude", "tg::delete_message");
            tg.delete_message(cb.from.id, message.id)
                .send()
                .await
                .map_err(anyhow::Error::from)?;
        }
    }

    tracing::debug!("tg::answer_callback_query");
    tg.answer_callback_query(&cb.id)
        .send()
        .await
        .map_err(anyhow::Error::from)?;

    Ok(())
}
