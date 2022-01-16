mod feedback;
mod grid;
mod l10n;
mod metrics;
mod utils;

mod types;
pub use types::DataEntry;

mod db;
pub use db::Db;

mod config;
pub use config::Config;

mod handlers;

static TOKEN: Lazy<String> = Lazy::new(|| env::var("TOKEN").expect("TOKEN not provided"));

use std::{env, io::stdout, path::PathBuf, sync::Arc};

use anyhow::Context as _;
use axum::{routing::get, AddExtensionLayer, Router, Server};
use clap::Parser;
use once_cell::sync::Lazy;
use teloxide_core::{
    adaptors::DefaultParseMode,
    types::{ParseMode, Update, UpdateKind},
    Bot,
};
use tracing::Instrument;
use tracing_subscriber::EnvFilter;
use typed_builder::TypedBuilder;
use ulid::Ulid;

#[derive(Debug, Clone, Parser)]
pub struct Args {
    #[clap(long, short)]
    pub config: PathBuf,
}

type Tg = DefaultParseMode<Bot>;

#[derive(Clone, TypedBuilder)]
pub struct Context {
    pub tg: Tg,
    pub db: Db,

    #[builder(default = l10n::Lang::Ru)]
    pub lang: l10n::Lang,

    pub config: Arc<Config>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv();

    let (writer, _guard) = tracing_appender::non_blocking(stdout());
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(writer)
        .init();

    let args = Args::parse();
    let config = Config::load(args.config)?;

    run(config).await
}

async fn run(config: Config) -> anyhow::Result<()> {
    let db = Db::connect(config.db).await?;
    let tg = DefaultParseMode::new(Bot::new(&*TOKEN), ParseMode::MarkdownV2);

    metrics::register();

    let router = Router::new()
        .route("/metrics", get(metrics::gather))
        .layer(AddExtensionLayer::new(db))
        .layer(AddExtensionLayer::new(tg));

    Server::bind(&config.http.bind)
        .serve(router.into_make_service())
        .await
        .context("http::bind")
}

pub async fn process_update(update: &Update, context: Context) -> anyhow::Result<()> {
    let span = tracing::info_span!("update", id = Ulid::new().to_string().as_str());

    let block = async {
        match &update.kind {
            UpdateKind::Message(inner) => handlers::message::handle(inner, context).await,
            UpdateKind::CallbackQuery(inner) => handlers::callback::handle(inner, context).await,
            _ => {
                tracing::warn!("unexpected update kind: {:?}", update);
                Ok(())
            }
        }
    };

    block.instrument(span).await
}
