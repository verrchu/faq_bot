mod db;
pub use db::Db;

mod grid;
mod utils;

mod handlers;
use handlers::{handle_callback_query, handle_message};

use teloxide_core::{
    requests::Requester,
    types::{Update, UpdateKind},
    RequestError,
};

pub async fn process_update<R: Requester<Err = RequestError>>(
    bot: R,
    update: &Update,
    db: Db,
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
