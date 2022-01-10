use crate::{Db, Tg};

use teloxide_core::{
    requests::{Request, Requester},
    types::CallbackQuery,
};

pub async fn handle(tg: Tg, cb: &CallbackQuery, mut db: Db) -> anyhow::Result<()> {
    let is_active = db.is_feedback_process_active(cb.from.id).await?;

    // TODO: signal in query response that feedback is in progress
    if !is_active {
        let message = tg
            .send_message(cb.from.id, "PLEASE SEND FEEDBACK")
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
