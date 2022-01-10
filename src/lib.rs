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

#[derive(Clone)]
pub struct Context {
    pub tg: Tg,
    pub db: Db
}

pub async fn process_update(update: &Update, context: Context) -> anyhow::Result<()> {
    match &update.kind {
        UpdateKind::Message(inner) => handlers::message::handle(inner, context).await,
        UpdateKind::CallbackQuery(inner) => handlers::callback::handle(inner, context).await,
        _ => {
            tracing::warn!("unexpected update kind arrived: {:?}", update);
            Ok(())
        }
    }
}
