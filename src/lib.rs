mod templates;
mod types;
pub use types::{DataEntry, Lang};

mod db;
pub use db::Db;

mod grid;
mod utils;

mod handlers;

use std::sync::Arc;

use teloxide_core::{
    requests::Requester,
    types::{Update, UpdateKind},
    RequestError,
};

pub async fn process_update<R: Requester<Err = RequestError> + Send + Sync + 'static>(
    bot: Arc<R>,
    update: &Update,
    db: Db,
) -> anyhow::Result<()>
where
    R::SendMessage: Send + Sync,
    R::DeleteMessage: Send + Sync,
{
    match &update.kind {
        UpdateKind::Message(inner) => handlers::message::handle(bot, inner, db).await,
        UpdateKind::CallbackQuery(inner) => handlers::callback::handle(bot, inner, db).await,
        _ => {
            tracing::warn!("unexpected update kind arrived: {:?}", update);
            Ok(())
        }
    }
}
