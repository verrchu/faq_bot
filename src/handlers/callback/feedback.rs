use crate::{feedback, l10n, Context};

use teloxide_core::{
    requests::{Request, Requester},
    types::CallbackQuery,
};

pub async fn handle(cb: &CallbackQuery, context: Context) -> anyhow::Result<()> {
    let mut db = context.db.clone();
    let tg = context.tg.clone();
    let lang = context.lang;

    let is_active = db.is_feedback_active(cb.from.id).await?;

    // TODO: signal in query response that feedback is in progress
    if !is_active {
        {
            let user_id = cb.from.id;
            tokio::spawn(async move { feedback::cancel(user_id, context).await });
        }

        let message = tg
            .send_message(cb.from.id, l10n::messages::feedback_prelude(lang))
            .send()
            .await
            .map_err(anyhow::Error::from)?;

        let inited = db.begin_feedback(cb.from.id, message.id).await?;

        // there might be a rare condition when one user tries to submit
        // feedback from several devices.
        // "inited" is supposed to handle this issue.
        if !inited {
            tg.delete_message(cb.from.id, message.id)
                .send()
                .await
                .map_err(anyhow::Error::from)?;
        }
    }

    tg.answer_callback_query(&cb.id)
        .send()
        .await
        .map_err(anyhow::Error::from)?;

    Ok(())
}
