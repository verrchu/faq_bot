use crate::{l10n, Context};

use teloxide_core::{
    requests::{Request, Requester},
    types::CallbackQuery,
};

pub async fn handle(cb: &CallbackQuery, context: Context) -> anyhow::Result<()> {
    let mut db = context.db.clone();
    let tg = context.tg.clone();
    let lang = context.lang;

    let is_active = db.is_feedback_process_active(cb.from.id).await?;

    // TODO: signal in query response that feedback is in progress
    if !is_active {
        {
            let user_id = cb.from.id;
            tokio::spawn(async move { cancel(user_id, context).await });
        }

        let message = tg
            .send_message(cb.from.id, l10n::messages::feedback_prelude(lang))
            .send()
            .await
            .map_err(anyhow::Error::from)?;

        let inited = db.begin_feedback_process(cb.from.id, message.id).await?;

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

pub async fn cancel(user_id: i64, context: Context) -> anyhow::Result<()> {
    let tg = context.tg;
    let mut db = context.db;
    let config = context.config;

    tokio::time::sleep(config.feedback.timeout).await;

    if let Some(fb_req_msg_id) = db.cancel_feedback(user_id).await? {
        tg.delete_message(user_id, fb_req_msg_id)
            .send()
            .await
            .map_err(anyhow::Error::from)?;
    }

    Ok(())
}
