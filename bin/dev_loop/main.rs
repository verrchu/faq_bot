mod args;
use args::Args;

mod config;
use config::Config;

use std::{env, io::stdout, sync::Arc};

use help_desk_bot as bot;

use clap::Parser;
use once_cell::sync::Lazy;
use teloxide_core::{
    adaptors::DefaultParseMode,
    payloads::setters::*,
    requests::{Request, Requester},
    types::{AllowedUpdate, ParseMode},
    Bot,
};

use bot as handler;

use tracing_subscriber::EnvFilter;

static TOKEN: Lazy<String> = Lazy::new(|| env::var("TOKEN").expect("TOKEN not provided"));

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
    let tg = DefaultParseMode::new(Bot::new(&*TOKEN), ParseMode::MarkdownV2);
    let db = bot::Db::connect(&config.bot.db).await?;

    let context = bot::Context::builder()
        .tg(tg)
        .db(db)
        .config(Arc::new(config.bot))
        .build();

    let mut offset = 0;
    let mut get_updates = context
        .tg
        .get_updates()
        .allowed_updates([AllowedUpdate::Message, AllowedUpdate::CallbackQuery]);

    loop {
        tracing::debug!("getting updates");

        tokio::time::sleep(config.interval).await;
        let updates = get_updates.send_ref().await;

        match updates {
            Ok(updates) => {
                tracing::debug!("fetched updates: {:?}", updates);
                if !updates.is_empty() {
                    tracing::info!("fetched {} updates", updates.len());
                }

                for update in updates.iter() {
                    if update.id >= offset {
                        offset = update.id + 1;
                        get_updates.offset = Some(offset);
                    }

                    if let Err(err) = handler::process_update(update, context.clone()).await {
                        tracing::error!("failed to process update: {:?}", err);
                    }
                }
            }
            Err(err) => {
                tracing::error!("failed to get updates: {:?}", err);
            }
        }
    }
}
