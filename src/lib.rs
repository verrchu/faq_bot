mod utils;

mod grid;

mod handlers;
use handlers::{handle_callback_query, handle_message};

use redis::aio::ConnectionManager;
use teloxide_core::{
    requests::Requester,
    types::{Update, UpdateKind},
    RequestError,
};

pub async fn process_update<R: Requester<Err = RequestError>>(
    bot: R,
    update: &Update,
    db: ConnectionManager,
) -> anyhow::Result<()> {
    match &update.kind {
        UpdateKind::Message(inner) => handle_message(bot, inner, db).await,
        UpdateKind::CallbackQuery(inner) => handle_callback_query(bot, inner, db).await,
        _ => {
            tracing::warn!("unexpected update kind arrived: {:?}", update);
            Ok(())
        }
    }
}
