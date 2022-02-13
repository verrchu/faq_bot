mod feedback;
mod grid;
mod greeting;
mod l10n;
mod logs;
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

use std::{env, path::PathBuf, sync::Arc};

use anyhow::Context as _;
use axum::{
    extract,
    routing::{get, post},
    AddExtensionLayer, Router,
};
use axum_server::tls_rustls::RustlsConfig;
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

    let args = Args::parse();
    let config = Config::load(args.config).context("config::load")?;

    let (writer, _guard) = tracing_appender::non_blocking(logs::writer(&config));
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(writer)
        .init();

    run(config).await
}

async fn run(config: Config) -> anyhow::Result<()> {
    let db = Db::connect(config.db.clone()).await?;
    let tg = DefaultParseMode::new(Bot::new(&*TOKEN), ParseMode::MarkdownV2);

    let context = Context::builder()
        .db(db)
        .tg(tg)
        .config(Arc::new(config.clone()))
        .build();

    metrics::register();

    let router = Router::new()
        .route("/metrics", get(metrics::gather))
        .route("/notify", post(process_update))
        .layer(AddExtensionLayer::new(context));

    if let Some(tls) = &config.http.tls {
        let tls = RustlsConfig::from_pem_file(&tls.cert, &tls.key)
            .await
            .context("init tls")?;

        axum_server::bind_rustls(config.http.bind, tls)
            .serve(router.into_make_service())
            .await
            .context("http::bind")
    } else {
        axum::Server::bind(&config.http.bind)
            .serve(router.into_make_service())
            .await
            .context("http::bind")
    }
}

pub async fn process_update(
    extract::Json(update): extract::Json<Update>,
    extract::Extension(context): extract::Extension<Context>,
) {
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

    if let Err(err) = block.instrument(span).await {
        tracing::error!("failed to process event: {:?}", err);
    }
}
