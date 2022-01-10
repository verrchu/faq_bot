mod templates;
mod types;
pub use types::{DataEntry, Lang};

mod db;
pub use db::Db;

mod grid;
mod utils;

mod handlers;

use teloxide_core::{
    adaptors::DefaultParseMode,
    types::{Update, UpdateKind},
    Bot,
};

type Tg = DefaultParseMode<Bot>;

pub async fn process_update(tg: Tg, update: &Update, db: Db) -> anyhow::Result<()> {
    match &update.kind {
        UpdateKind::Message(inner) => handlers::message::handle(tg, inner, db).await,
        UpdateKind::CallbackQuery(inner) => handlers::callback::handle(tg, inner, db).await,
        _ => {
            tracing::warn!("unexpected update kind arrived: {:?}", update);
            Ok(())
        }
    }
}
