mod scripts;
use scripts::Scripts;

mod queries;
pub use queries::{feedback, grid, utils};

use std::sync::Arc;

use crate::config::DbConfig;

use anyhow::Context;
use r2d2::Pool;
use redis::cluster::ClusterClient;

#[derive(Clone)]
pub struct Db {
    pool: Pool<ClusterClient>,
    scripts: Arc<Scripts>,
}

impl Db {
    pub async fn connect(config: &DbConfig) -> anyhow::Result<Self> {
        let pool = {
            let nodes = config.nodes.iter().map(ToString::to_string).collect();
            let client = ClusterClient::open(nodes).context("failed to create cluster client")?;
            Pool::builder()
                .max_size(u32::from(config.pool_size))
                .connection_timeout(config.connection_timeout)
                .build(client)
                .context("failed to create db connection pool")?
        };

        tracing::info!("db cluster connected");

        let scripts = Scripts::load(&config.scripts_path).context("failed to load db scripts")?;

        tracing::info!("db scripts loaded");

        let this = Self {
            pool,
            scripts: Arc::new(scripts),
        };

        Self::init(&this).await.map(|_| this)
    }

    async fn init(&self) -> anyhow::Result<()> {
        // TODO: maybe it would be better to cancel all pending feedbacks
        // rather than silently forget them
        feedback::vanish(self.to_owned()).await
    }
}
