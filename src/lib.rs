mod grid;
mod templates;
mod utils;

mod types;
pub use types::{DataEntry, Lang};

mod db;
pub use db::Db;

mod config;
pub use config::Config;

mod handlers;

use std::sync::Arc;

use teloxide_core::{
    adaptors::DefaultParseMode,
    types::{Update, UpdateKind},
    Bot,
};

type Tg = DefaultParseMode<Bot>;

#[derive(Clone)]
pub struct Context {
    pub tg: Tg,
    pub db: Db,

    pub config: Arc<Config>,
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
