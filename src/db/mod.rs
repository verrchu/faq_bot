mod scripts;
use scripts::Scripts;

mod queries;
pub use queries::{feedback, grid, utils};

use std::{net::Ipv4Addr, path::PathBuf, sync::Arc};

use redis::{aio::ConnectionManager, Client};

#[derive(Clone)]
pub struct Db {
    conn: ConnectionManager,
    scripts: Arc<Scripts>,
}

impl Db {
    pub async fn connect(host: Ipv4Addr, port: u16, scripts_path: PathBuf) -> anyhow::Result<Self> {
        let client = Client::open((host.to_string(), port)).map_err(anyhow::Error::from)?;

        let conn = ConnectionManager::new(client)
            .await
            .map_err(anyhow::Error::from)?;

        let scripts = Scripts::load(scripts_path)?;

        let mut this = Self {
            conn,
            scripts: Arc::new(scripts),
        };

        Self::init(&mut this).await.map(|_| this)
    }

    async fn init(&mut self) -> anyhow::Result<()> {
        // TODO: maybe it would be better to cancel all pending feedbacks
        // rather than silently forget them
        feedback::vanish(self).await
    }
}
