use crate::{grid, Context};

use teloxide_core::{
    payloads::setters::*,
    requests::{Request, Requester},
    types::User,
};
use tracing::Instrument;

pub async fn handle(user: &User, context: Context) -> anyhow::Result<()> {
    let db = context.db;
    let tg = context.tg;

    let (header, keyboard) = grid::init(db)
        .instrument(tracing::info_span!("grid_init"))
        .await?;

    tg.send_message(user.id, header)
        .disable_web_page_preview(true)
        .reply_markup(keyboard)
        .send()
        .await
        .map(|_| ())
        .map_err(anyhow::Error::from)?;

    Ok(())
}
